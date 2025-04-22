use apollo_compiler::Name;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
/// Selection fields for `@connect`
pub struct SelectionFields {
    /// Collection of field mappings
    pub fields: Vec<FieldMapping>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
/// Selection Field Mapping expressions
pub struct FieldMapping {
    /// Field Name
    pub field_name: Name,
    /// Expression mapping on how to resolve `field_name`
    pub expression: MappingExpression,
}

/// Mapping expressions for the selection field
///
/// Example:
/// ```ignore
///  id: account_number
///  name
///  bank: $args.bank
///  status: data->first.status->match(
///    @ => {
///      [1 => "online"]
///      [0 => "offline"]
///      [_ => "suspended"]
///    }
///  )
/// ```
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum MappingExpression {
    #[default]
    /// field name is the same as response, eg. `name`.
    Direct,
    /// field name is aliased in the response, eg. `id: account_number`.
    Alias(Name),
    /// field name is a named argument, eg. `bank: $args.bank`.
    // TODO: Replace content with `Vec<Name>` for better navigation
    NamedArgument(String),
    /// field name is a named path, eg. `bank: data.bank.name`.
    // TODO: Replace content with `Vec<Name>` for better navigation
    NamedPath(String),
    /// field name is created upon transforming received data, eg. `status: data->first`
    Transform {
        source: String,
        transform: TransformFn,
    },
    /// matcher function to override values in source field
    Matcher {
        source: String,
        // TODO: Could use a serde serialize_with to follow graphql semantics and avoid `null`
        matchers: Vec<(Option<String>, String)>,
    },
    Nested {
        fields: Vec<FieldMapping>,
    },
    AliasNested {
        source: Name,
        fields: Vec<FieldMapping>,
    },
}

impl MappingExpression {
    pub(crate) fn into_alias(self, source: Name) -> Self {
        match self {
            Self::Direct => Self::Alias(source),
            Self::Nested { fields } => Self::AliasNested { source, fields },
            others => others,
        }
    }
}

/// Functions that allow to transform data in response
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransformFn {
    /// Gets the first value on an array
    First,
}

#[allow(clippy::fallible_impl_from)]
impl From<&str> for TransformFn {
    fn from(value: &str) -> Self {
        match value {
            "first" => Self::First,
            other => panic!("Functions `{other}` no impletements"),
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<String> for TransformFn {
    fn from(value: String) -> Self {
        match &value[..] {
            "first" => Self::First,
            other => panic!("Functions `{other}` no impletements"),
        }
    }
}

impl From<Vec<FieldMapping>> for SelectionFields {
    fn from(fields: Vec<FieldMapping>) -> Self {
        Self { fields }
    }
}
