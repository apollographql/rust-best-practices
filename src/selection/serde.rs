use super::domain::SelectionFields;

pub trait Serde: Sized {
    /// Saves the [`FieldMapping`] into a JSON format
    ///
    /// # Errors
    ///
    /// Returns [`serde_json::Error`]
    fn save(&self) -> serde_json::Result<String>;
    /// Loads a josn file into [`FieldMapping`]
    ///
    /// # Errors
    ///
    /// Returns [`serde_json::Error`]
    fn load(json: &str) -> serde_json::Result<Self>;
}

impl Serde for SelectionFields {
    fn save(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    fn load(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use crate::selection::parser::selection_parser;

    use super::*;

    #[test]
    fn testing_serde() {
        let mut input = r#"
        id: account_number,
        name,
        bank: {$args.bank},
        account: data.users->first,
        status: status->match(
        @ => {
            [1 => "online"],
            [0 => "offline"],
            [_ => "suspended"],
        }),
"#;
        let result = selection_parser(&mut input).unwrap();

        let selection_fields: SelectionFields = result.into();
        let json = selection_fields.save().unwrap();
        let fields = SelectionFields::load(&json).unwrap();

        assert_eq!(selection_fields, fields);
    }
}
