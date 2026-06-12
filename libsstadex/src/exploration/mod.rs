pub mod conditions;
pub mod table;

pub use conditions::{
    ExplorationFilter, FilterEqualColumnsError, FilterKnownColumnsError, FilterPhase,
    RangeCondition, SpecificationResult, filter_conditions, filter_equal_columns,
    filter_known_columns,
};
pub use table::{
    ExplorationColumn, ExplorationTable, ExplorationTableError, assemble_filtered_table,
};
