pub mod candidate;
pub mod conditions;
pub mod derived;
pub mod evaluation;
pub mod flow;
pub mod hierarchy;
pub mod io;
pub mod prepare;
pub mod spec;
pub mod table;

pub use candidate::{
    AxisFilterError, CandidateAxis, CandidateFilterError, CandidateGenerationError,
    CandidatePipelineError, CandidatePoint, CandidateSet, CandidateSetBuildError,
    build_filtered_candidates, candidate_column_name, candidate_column_names,
    candidate_set_from_columns, candidate_set_from_prefixed_columns, filter_candidate_axes,
    filter_candidate_points, generate_candidate_combinations, generate_candidate_grid,
};
pub use conditions::{
    ExplorationFilter, FilterEqualColumnsError, FilterKnownColumnsError, FilterPhase,
    RangeCondition, SpecificationResult, filter_conditions, filter_equal_columns,
    filter_known_columns, shared_node_filter,
};
pub use derived::{DerivedColumnError, DerivedColumnSpec, apply_derived_columns};
pub use evaluation::{
    CandidateEvaluation, CandidateEvaluationError, evaluate_candidate_expression,
    evaluate_candidates, evaluate_prepared_candidate_spec, evaluate_prepared_candidate_specs,
    evaluate_prepared_expression_spec, evaluate_prepared_expression_specs,
    required_candidate_value,
};
pub use flow::{
    ExplorationFlowError, run_candidate_expression_flow, run_prepared_expression_flow,
    run_prepared_expression_flow_with_derived_columns,
};
pub use hierarchy::{
    ExplorationOutput, HierarchicalCandidateInput, HierarchicalExplorationProject,
    InterfaceVariable, MacroExplorationWorkspace, SubmacroCandidateError, SubmacroConditionError,
    SubmacroConditionRule, SubmacroConditionSource, compact_parameter_column_name,
    derive_submacro_condition_filters, submacro_results_to_candidate_set,
    submacro_results_to_compact_candidate_set,
};
pub use io::{
    ExplorationCandidateInput, ExplorationIoError, load_exploration_candidates,
    load_exploration_specs, load_testbenches, save_exploration_candidates, save_exploration_specs,
    save_testbenches,
};
pub use prepare::{
    PreparedSpec, PreparedSpecSource, SpecPrepareError, prepare_candidate_expression_spec,
    prepare_candidate_expression_specs, prepare_macro_testbench_specs,
    prepare_macro_testbench_specs_with_mode, prepare_transfer_function_spec,
    prepare_transfer_function_spec_for_macro_testbench,
    prepare_transfer_function_spec_for_macro_testbench_with_mode,
    prepare_transfer_function_spec_from_analysis,
};
pub use spec::{
    CircuitView, CompactOutputBinding, DutRef, ExplorationSpec, FrequencySweep, SpecOutput,
    SpecParameter, SpecSource, SpecVariable, TestbenchElement, TestbenchSpec,
};
pub use table::{
    ExplorationColumn, ExplorationTable, ExplorationTableError, add_automatic_area_column,
    assemble_filtered_table,
};
