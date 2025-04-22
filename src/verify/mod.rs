use std::collections::HashMap;

use apollo_compiler::{Name, Schema, schema::ExtendedType};

use self::error::Error as VerificationError;
use crate::selection::{domain::SelectionFields, parser::selection_parser};
pub mod error;

/// Receives a [`SelectionFields`] and an [`apollo_compiler::Schema`] and verifies if they align.
///
/// # Errors
///
/// Fails if there is a missing field from the schema connect directive in the selection
pub fn verify(selection: &SelectionFields, schema: &Schema) -> Result<(), VerificationError> {
    todo!()
}
