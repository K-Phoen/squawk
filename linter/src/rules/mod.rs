pub mod adding_field_with_default;
pub use adding_field_with_default::*;
pub mod adding_not_null_field;
pub use adding_not_null_field::*;
pub mod adding_primary_key_constraint;
pub use adding_primary_key_constraint::*;
pub mod bad_drop_database;
pub use bad_drop_database::*;
pub mod changing_column_type;
pub use changing_column_type::*;
pub mod constraint_missing_not_valid;
pub use constraint_missing_not_valid::*;
pub mod disallow_unique_constraint;
pub use disallow_unique_constraint::*;
pub mod prefer_robust_stmts;
pub use prefer_robust_stmts::*;
pub mod prefer_text_field;
pub use prefer_text_field::*;
pub mod renaming_column;
pub use renaming_column::*;
pub mod renaming_table;
pub use renaming_table::*;
pub mod require_concurrent_index_creation;
pub use require_concurrent_index_creation::*;
pub mod ban_char_field;
pub use ban_char_field::*;
mod utils;
