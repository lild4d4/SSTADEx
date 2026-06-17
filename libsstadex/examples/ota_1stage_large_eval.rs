use std::env;
use std::time::Instant;

use libsstadex::exploration::{
    CandidatePoint, CandidateSet, ExplorationFilter, ExplorationSpec, FilterPhase, RangeCondition,
    SpecOutput, SpecSource, build_filtered_candidates, evaluate_prepared_expression_specs,
    filter_conditions, prepare_candidate_expression_specs, shared_node_filter,
};

fn main() -> Result<(), String> {
    let points_per_primitive = env_usize("SSTADEX_LARGE_EVAL_POINTS", 80);
    let shared_node_bins = env_usize("SSTADEX_LARGE_EVAL_SHARED_BINS", 10);

    let diffpair = diffpair_candidates(points_per_primitive, shared_node_bins);
    let current_mirror = current_mirror_candidates(points_per_primitive);
    let current_source = current_source_candidates(points_per_primitive, shared_node_bins);

    let raw_count =
        diffpair.points.len() * current_mirror.points.len() * current_source.points.len();
    let filters = vec![
        shared_node_filter(vec!["xdp.vtail", "xcs.vout"]),
        ExplorationFilter::new(
            FilterPhase::CandidatePreEvaluation,
            "xdp.vtail",
            RangeCondition::new(Some(0.30), Some(0.75)),
        ),
    ];
    let specs = ota_specs();

    println!("OTA 1-stage large candidate evaluation");
    println!("points_per_primitive={points_per_primitive}");
    println!("shared_node_bins={shared_node_bins}");
    println!("raw_candidate_count={raw_count}");

    let build_start = Instant::now();
    let candidates =
        build_filtered_candidates(&[], &[diffpair, current_mirror, current_source], &filters)
            .map_err(|error| format!("{error:?}"))?;
    let build_elapsed = build_start.elapsed();

    let prepare_start = Instant::now();
    let prepared_specs =
        prepare_candidate_expression_specs(&specs).map_err(|error| format!("{error:?}"))?;
    let prepare_elapsed = prepare_start.elapsed();

    let eval_start = Instant::now();
    let spec_results = evaluate_prepared_expression_specs(&candidates, &prepared_specs)
        .map_err(|error| format!("{error:?}"))?;
    let eval_elapsed = eval_start.elapsed();

    let post_filter_start = Instant::now();
    let post_mask = filter_conditions(&spec_results).map_err(|error| format!("{error:?}"))?;
    let final_rows = post_mask.iter().filter(|is_kept| **is_kept).count();
    let post_filter_elapsed = post_filter_start.elapsed();

    println!("pre_evaluation_candidate_count={}", candidates.len());
    println!("final_row_count={final_rows}");
    println!(
        "candidate_reduction={:.3}x",
        raw_count as f64 / candidates.len() as f64
    );
    println!(
        "post_filter_reduction={:.3}x",
        candidates.len() as f64 / final_rows.max(1) as f64
    );
    println!("build_filtered_candidates={build_elapsed:?}");
    println!("prepare_specs={prepare_elapsed:?}");
    println!("evaluate_specs={eval_elapsed:?}");
    println!("post_filter_specs={post_filter_elapsed:?}");

    for result in spec_results {
        print_spec_summary(&result.name, &result.values);
    }

    Ok(())
}

fn diffpair_candidates(points: usize, shared_bins: usize) -> CandidateSet {
    let gm = linspace(0.5e-3, 3.0e-3, points);
    let ro = linspace(2.0e4, 2.0e5, points);
    let vtail = quantized_lut_axis(0.20, 0.90, points, shared_bins);
    let width = linspace(2.0e-6, 40.0e-6, points);

    CandidateSet::new(
        "diffpair",
        (0..points)
            .map(|idx| {
                CandidatePoint::new(vec![
                    ("xdp.gm".to_string(), gm[idx]),
                    ("xdp.ro".to_string(), ro[points - 1 - idx]),
                    ("xdp.vtail".to_string(), vtail[idx]),
                    ("xdp.width".to_string(), width[idx]),
                ])
            })
            .collect(),
    )
}

fn current_mirror_candidates(points: usize) -> CandidateSet {
    let gm = linspace(0.4e-3, 2.2e-3, points);
    let ro = linspace(3.0e4, 2.5e5, points);
    let width = linspace(1.5e-6, 35.0e-6, points);

    CandidateSet::new(
        "current_mirror",
        (0..points)
            .map(|idx| {
                CandidatePoint::new(vec![
                    ("xcm.gm".to_string(), gm[idx]),
                    ("xcm.ro".to_string(), ro[idx]),
                    ("xcm.width".to_string(), width[idx]),
                ])
            })
            .collect(),
    )
}

fn current_source_candidates(points: usize, shared_bins: usize) -> CandidateSet {
    let gm = linspace(0.2e-3, 1.5e-3, points);
    let ro = linspace(5.0e4, 3.0e5, points);
    let vout = quantized_lut_axis(0.20, 0.90, points, shared_bins);
    let ibias = linspace(2.0e-6, 80.0e-6, points);
    let width = linspace(1.0e-6, 20.0e-6, points);

    CandidateSet::new(
        "current_source",
        (0..points)
            .map(|idx| {
                CandidatePoint::new(vec![
                    ("xcs.gm".to_string(), gm[idx]),
                    ("xcs.ro".to_string(), ro[idx]),
                    ("xcs.vout".to_string(), vout[idx]),
                    ("xcs.ibias".to_string(), ibias[idx]),
                    ("xcs.width".to_string(), width[idx]),
                ])
            })
            .collect(),
    )
}

fn ota_specs() -> Vec<ExplorationSpec> {
    vec![
        ExplorationSpec::new(
            "gain",
            RangeCondition::min(50.0),
            SpecSource::CandidateExpression {
                expression: "xdp.gm * (xdp.ro * xcm.ro / (xdp.ro + xcm.ro))".to_string(),
            },
            SpecOutput::Eval,
        ),
        ExplorationSpec::new(
            "rout",
            RangeCondition::min(2.5e4),
            SpecSource::CandidateExpression {
                expression: "xdp.ro * xcm.ro / (xdp.ro + xcm.ro)".to_string(),
            },
            SpecOutput::Eval,
        ),
        ExplorationSpec::new(
            "power",
            RangeCondition::max(100.0e-6),
            SpecSource::CandidateExpression {
                expression: "1.8 * xcs.ibias".to_string(),
            },
            SpecOutput::Eval,
        ),
        ExplorationSpec::new(
            "area",
            RangeCondition::max(60.0e-6),
            SpecSource::CandidateExpression {
                expression: "xdp.width + xcm.width + xcs.width".to_string(),
            },
            SpecOutput::Eval,
        ),
    ]
}

fn linspace(start: f64, stop: f64, points: usize) -> Vec<f64> {
    match points {
        0 => Vec::new(),
        1 => vec![start],
        _ => {
            let step = (stop - start) / ((points - 1) as f64);
            (0..points).map(|idx| start + step * (idx as f64)).collect()
        }
    }
}

fn quantized_lut_axis(start: f64, stop: f64, points: usize, bins: usize) -> Vec<f64> {
    let bins = bins.max(1);
    let values = linspace(start, stop, bins);

    (0..points).map(|idx| values[idx % bins]).collect()
}

fn env_usize(name: &str, default: usize) -> usize {
    env::var(name)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn print_spec_summary(name: &str, values: &[f64]) {
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mean = values.iter().sum::<f64>() / values.len().max(1) as f64;

    println!("{name}: min={min:.6e} mean={mean:.6e} max={max:.6e}");
}
