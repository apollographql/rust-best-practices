use apollo_compiler::Schema;

use self::error::Error as VerificationError;
use crate::selection::domain::SelectionFields;
pub mod error;

/// Receives a [`SelectionFields`] and an [`apollo_compiler::Schema`] and verifies if they align.
///
/// # Errors
///
/// Fails if there is a missing field from the schema connect directive in the selection
pub fn verify(selection: &SelectionFields, schema: &Schema) -> Result<(), VerificationError> {
    todo!()
}
