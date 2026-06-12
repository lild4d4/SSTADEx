pub mod candidate;
pub mod conditions;
pub mod table;

pub use candidate::{
    AxisFilterError, CandidateAxis, CandidateFilterError, CandidateGenerationError,
    CandidatePoint, CandidateSet, CandidateSetBuildError, candidate_set_from_columns,
    filter_candidate_axes, filter_candidate_points, generate_candidate_combinations,
    generate_candidate_grid,
};
pub use conditions::{
    ExplorationFilter, FilterEqualColumnsError, FilterKnownColumnsError, FilterPhase,
    RangeCondition, SpecificationResult, filter_conditions, filter_equal_columns,
    filter_known_columns,
};
pub use table::{
    ExplorationColumn, ExplorationTable, ExplorationTableError, assemble_filtered_table,
};
