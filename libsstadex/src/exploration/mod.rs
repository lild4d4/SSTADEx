pub mod candidate;
pub mod conditions;
pub mod evaluation;
pub mod flow;
pub mod hierarchy;
pub mod io;
pub mod prepare;
pub mod spec;
pub mod table;

pub use candidate::{
    build_filtered_candidates, candidate_column_name, candidate_column_names,
    candidate_set_from_columns, candidate_set_from_prefixed_columns, filter_candidate_axes,
    filter_candidate_points, generate_candidate_combinations, generate_candidate_grid,
    AxisFilterError, CandidateAxis, CandidateFilterError, CandidateGenerationError,
    CandidatePipelineError, CandidatePoint, CandidateSet, CandidateSetBuildError,
};
pub use conditions::{
    filter_conditions, filter_equal_columns, filter_known_columns, shared_node_filter,
    ExplorationFilter, FilterEqualColumnsError, FilterKnownColumnsError, FilterPhase,
    RangeCondition, SpecificationResult,
};
pub use evaluation::{
    evaluate_candidate_expression, evaluate_candidates, evaluate_prepared_candidate_spec,
    evaluate_prepared_candidate_specs, evaluate_prepared_expression_spec,
    evaluate_prepared_expression_specs, required_candidate_value, CandidateEvaluation,
    CandidateEvaluationError,
};
pub use flow::{run_candidate_expression_flow, run_prepared_expression_flow, ExplorationFlowError};
pub use hierarchy::{
    compact_parameter_column_name, submacro_results_to_compact_candidate_set, ExplorationOutput,
    HierarchicalCandidateInput, HierarchicalExplorationProject, InterfaceVariable,
    MacroExplorationWorkspace, SubmacroCandidateError, SubmacroConditionRule,
    SubmacroConditionSource,
};
pub use io::{
    load_exploration_candidates, load_exploration_specs, load_testbenches,
    save_exploration_candidates, save_exploration_specs, save_testbenches,
    ExplorationCandidateInput, ExplorationIoError,
};
pub use prepare::{
    prepare_candidate_expression_spec, prepare_candidate_expression_specs,
    prepare_macro_testbench_specs, prepare_macro_testbench_specs_with_mode,
    prepare_transfer_function_spec, prepare_transfer_function_spec_for_macro_testbench,
    prepare_transfer_function_spec_for_macro_testbench_with_mode,
    prepare_transfer_function_spec_from_analysis, PreparedSpec, PreparedSpecSource,
    SpecPrepareError,
};
pub use spec::{
    CircuitView, CompactOutputBinding, DutRef, ExplorationSpec, FrequencySweep, SpecOutput,
    SpecParameter, SpecSource, SpecVariable, TestbenchElement, TestbenchSpec,
};
pub use table::{
    assemble_filtered_table, ExplorationColumn, ExplorationTable, ExplorationTableError,
};
