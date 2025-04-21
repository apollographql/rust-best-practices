use std::char;

use apollo_compiler::Name;
use winnow::Result as ParserResult;
use winnow::ascii::{line_ending, multispace0};
use winnow::combinator::{
    alt, delimited, opt, preceded, separated, separated_pair, terminated, trace,
};
use winnow::prelude::*;
use winnow::token::{literal, take_while};

use super::domain::{FieldMapping, MappingExpression};

fn ignore_whitespace<'a, O, F>(inner: F) -> impl Parser<&'a str, O, winnow::error::ContextError>
where
    F: Parser<&'a str, O, winnow::error::ContextError>,
{
    delimited(multispace0, inner, multispace0)
}

fn identifier(input: &mut &str) -> ParserResult<String> {
    trace(
        "identifier",
        ignore_whitespace(take_while(1.., |c: char| c.is_alphanumeric() || c == '_')),
    )
    .map(ToString::to_string)
    .parse_next(input)
}

fn named_arg(input: &mut &str) -> ParserResult<MappingExpression> {
    trace(
        "named_args",
        delimited(
            literal('{'),
            (
                literal('$'),
                take_while(0.., |c: char| c.is_alphanumeric() || c == '.' || c == '_'),
            ),
            literal('}'),
        ),
    )
    .map(|(comp, path)| MappingExpression::NamedArgument(format!("{comp}{path}")))
    .parse_next(input)
}

fn named_path_str(input: &mut &str) -> ParserResult<String> {
    trace(
        "named_path_as_str",
        separated(
            1..,
            take_while(1.., |c: char| c.is_alphanumeric() || c == '_'),
            literal('.'),
        ),
    )
    .map(|path: Vec<&str>| path.join("."))
    .parse_next(input)
}

fn named_path(input: &mut &str) -> ParserResult<MappingExpression> {
    trace(
        "named_path",
        separated(
            2..,
            take_while(1.., |c: char| c.is_alphanumeric() || c == '_'),
            literal('.'),
        ),
    )
    .map(|path: Vec<&str>| MappingExpression::NamedPath(path.join(".")))
    .parse_next(input)
}

fn map_match_case(input: &mut &str) -> ParserResult<(Option<String>, String)> {
    trace(
        "map_match_case",
        delimited(
            ignore_whitespace(literal("[")),
            separated_pair(
                take_while(1.., |c: char| !c.is_whitespace() && c != ']').map(|content: &str| {
                    if content == "_" || content.is_empty() {
                        None
                    } else {
                        Some(content.trim().to_string())
                    }
                }),
                ignore_whitespace(literal("=>")),
                take_while(0.., |c: char| !c.is_whitespace() && c != ']')
                    .map(str::trim)
                    .map(ToString::to_string),
            ),
            ignore_whitespace(literal("]")),
        ),
    )
    .parse_next(input)
}

fn match_expression(input: &mut &str) -> ParserResult<Vec<(Option<String>, String)>> {
    trace(
        "match_expression",
        delimited(
            (
                ignore_whitespace(literal("match(")),
                ignore_whitespace(literal('@')),
                ignore_whitespace(literal("=>")),
                ignore_whitespace(literal('{')),
                opt(line_ending),
            ),
            ignore_whitespace(terminated(
                separated(0.., map_match_case, ignore_whitespace(literal(','))),
                opt(literal(',')),
            )),
            (
                opt(line_ending),
                ignore_whitespace(literal('}')),
                ignore_whitespace(literal(')')),
            ),
        ),
    )
    .parse_next(input)
}

fn mapping_expression(input: &mut &str) -> ParserResult<MappingExpression> {
    let match_transform = (
        alt((named_path_str, identifier)),
        literal("->"),
        match_expression,
    )
        .map(|(source, _, matchers)| MappingExpression::Matcher { source, matchers });

    let alias = identifier.map(|alias| {
        MappingExpression::Alias(
            Name::new(&alias)
                .unwrap_or_else(|err| panic!("Invalid alias field mapping: `{alias}`: {err}")),
        )
    });

    let transform_fn = separated_pair(alt((named_path_str, identifier)), literal("->"), identifier)
        .map(|(source, r#fn)| MappingExpression::Transform {
            source,
            transform: r#fn.into(),
        });

    trace(
        "mapping_expression",
        preceded(
            ignore_whitespace(literal(":")),
            alt((match_transform, transform_fn, named_arg, named_path, alias)),
        ),
    )
    .parse_next(input)
}

fn field_mapping(input: &mut &str) -> ParserResult<FieldMapping> {
    trace(
        "field_mapping",
        (ignore_whitespace(identifier), opt(mapping_expression)),
    )
    .map(|(field_name, expr)| FieldMapping {
        field_name: Name::new(&field_name)
            .unwrap_or_else(|err| panic!("Invalid field name mapping: `{field_name}`: {err}")),
        expression: expr.unwrap_or_default(),
    })
    .parse_next(input)
}

fn nested_values(input: &mut &str) -> ParserResult<FieldMapping> {
    separated_pair(
        identifier,
        multispace0,
        delimited(
            literal('{'),
            ignore_whitespace(selection_parser),
            literal('}'),
        ),
    )
    .map(|(identifier, fields)| FieldMapping {
        field_name: Name::new(&identifier)
            .unwrap_or_else(|err| panic!("Invalid field name mapping: `{identifier}`: {err}")),
        expression: MappingExpression::Nested { fields },
    })
    .parse_next(input)
}

/// Field mapping for the `@connect` selection field
///
/// Example:
/// ```ignore
///  id: account_number,
///  name,
///  bank: $args.bank,
///  status: data->first.status->match(
///    @ => {
///      [1 => "online"],
///      [0 => "offline"],
///      [_ => "suspended"],
///    }
///  )
/// ```
///
/// # Errors
///
/// Returns a [`winnow::error::ContextError`]
///
/// # Panics
///
/// This function panics if the selection identifiers cannot be parsed [`apollo_compiler::Name`]
pub fn selection_parser(input: &mut &str) -> ParserResult<Vec<FieldMapping>> {
    separated(
        0..,
        alt((ignore_whitespace(nested_values), field_mapping)),
        alt((line_ending, literal(','), literal(';'))),
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use crate::selection::domain::TransformFn;

    use super::*;

    #[test]
    fn direct_field_name_identifier() {
        let result = selection_parser(&mut "name").unwrap();

        assert_eq!(
            result,
            vec![FieldMapping {
                field_name: Name::new_static_unchecked("name"),
                expression: MappingExpression::Direct
            }]
        );
    }

    #[test]
    fn alias_field_name_identifier() {
        let result = selection_parser(&mut "id: account_number").unwrap();

        assert_eq!(
            result,
            vec![FieldMapping {
                field_name: Name::new_static_unchecked("id"),
                expression: MappingExpression::Alias(Name::new_static_unchecked("account_number"))
            }]
        );
    }

    #[test]
    fn match_cases() {
        assert_eq!(
            map_match_case(&mut "[1 => online]").unwrap(),
            (Some("1".to_string()), "online".to_string())
        );
        assert_eq!(
            map_match_case(&mut "[0 => offline ]").unwrap(),
            (Some("0".to_string()), "offline".to_string())
        );
        assert_eq!(
            map_match_case(&mut "[ _ =>  suspended]").unwrap(),
            (None, "suspended".to_string())
        );
    }

    #[test]
    fn multiple_match_cases() {
        let mut input = "
        [1 => online], 
        [0 => offline ],  
        [ _ =>  suspended] ";
        let val: Vec<(Option<String>, String)> =
            separated(0.., map_match_case, ignore_whitespace(literal(',')))
                .parse_next(&mut input)
                .unwrap();

        assert_eq!(
            val,
            vec![
                (Some("1".to_string()), "online".to_string()),
                (Some("0".to_string()), "offline".to_string()),
                (None, "suspended".to_string()),
            ]
        );
    }

    #[test]
    fn simple_match_exp() {
        let mut input = r#"match(@ => {
            [1 => "online"],
            [0  => "offline"],
            [_ => "suspended"],
        })"#;
        let result = match_expression(&mut input);

        assert_eq!(
            result,
            Ok(vec![
                (Some("1".to_string()), "\"online\"".to_string()),
                (Some("0".to_string()), "\"offline\"".to_string()),
                (None, "\"suspended\"".to_string()),
            ])
        );
    }

    #[test]
    fn field_with_match_expression() {
        let mut input = r#"
status: status->match(@ => {
    [1 => "online"],
    [0 => "offline"],
    [_ => "suspended"],
})
"#;
        let result = selection_parser(&mut input).unwrap();

        assert_eq!(
            result,
            vec![FieldMapping {
                field_name: Name::new_static_unchecked("status"),
                expression: MappingExpression::Matcher {
                    source: "status".to_string(),
                    matchers: vec![
                        (Some("1".to_string()), "\"online\"".to_string()),
                        (Some("0".to_string()), "\"offline\"".to_string()),
                        (None, "\"suspended\"".to_string()),
                    ]
                }
            }]
        );
    }

    #[test]
    fn named_argument_field_name_identifier() {
        let result = selection_parser(&mut "bank: {$args.bank}").unwrap();

        assert_eq!(
            result,
            vec![FieldMapping {
                field_name: Name::new_static_unchecked("bank"),
                expression: MappingExpression::NamedArgument(String::from("$args.bank"))
            }]
        );
    }

    #[test]
    fn named_path_field_name_identifier() {
        let result = selection_parser(&mut "bank: data.result.bank").unwrap();

        assert_eq!(
            result,
            vec![FieldMapping {
                field_name: Name::new_static_unchecked("bank"),
                expression: MappingExpression::NamedPath(String::from("data.result.bank"))
            }]
        );
    }

    #[test]
    fn fn_first_with_path() {
        let result = selection_parser(&mut "bank: data.result.banks->first").unwrap();

        assert_eq!(
            result,
            vec![FieldMapping {
                field_name: Name::new_static_unchecked("bank"),
                expression: MappingExpression::Transform {
                    source: "data.result.banks".to_string(),
                    transform: TransformFn::First
                }
            }]
        );
    }

    #[test]
    fn fn_first_without_path() {
        let result = selection_parser(&mut "bank: banks->first").unwrap();

        assert_eq!(
            result,
            vec![FieldMapping {
                field_name: Name::new_static_unchecked("bank"),
                expression: MappingExpression::Transform {
                    source: "banks".to_string(),
                    transform: TransformFn::First
                }
            }]
        );
    }

    #[test]
    fn multiline() {
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

        assert_eq!(
            result,
            vec![
                FieldMapping {
                    field_name: Name::new_static_unchecked("id"),
                    expression: MappingExpression::Alias(Name::new_static_unchecked(
                        "account_number"
                    ))
                },
                FieldMapping {
                    field_name: Name::new_static_unchecked("name"),
                    expression: MappingExpression::Direct
                },
                FieldMapping {
                    field_name: Name::new_static_unchecked("bank"),
                    expression: MappingExpression::NamedArgument("$args.bank".to_string())
                },
                FieldMapping {
                    field_name: Name::new_static_unchecked("account"),
                    expression: MappingExpression::Transform {
                        source: "data.users".to_string(),
                        transform: TransformFn::First
                    }
                },
                FieldMapping {
                    field_name: Name::new_static_unchecked("status"),
                    expression: MappingExpression::Matcher {
                        source: "status".to_string(),
                        matchers: vec![
                            (Some("1".to_string()), "\"online\"".to_string()),
                            (Some("0".to_string()), "\"offline\"".to_string()),
                            (None, "\"suspended\"".to_string()),
                        ]
                    }
                }
            ]
        );
    }

    #[test]
    fn simple_nested_values() {
        let mut input = "
        variants {
          name,
          price     
        }";

        let result = selection_parser(&mut input).unwrap();

        assert_eq!(
            result,
            vec![FieldMapping {
                field_name: Name::new_static_unchecked("variants"),
                expression: MappingExpression::Nested {
                    fields: vec![
                        FieldMapping {
                            field_name: Name::new_static_unchecked("name"),
                            expression: MappingExpression::Direct,
                        },
                        FieldMapping {
                            field_name: Name::new_static_unchecked("price"),
                            expression: MappingExpression::Direct,
                        },
                    ]
                }
            }]
        );
    }

    #[test]
    fn multi_nested_values() {
        let mut input = "
        variants {
          name,
          tag: variant_tag,
          price {
            original,
            final
          }        
        }";
        let result = selection_parser(&mut input).unwrap();

        assert_eq!(
            result,
            vec![FieldMapping {
                field_name: Name::new_static_unchecked("variants"),
                expression: MappingExpression::Nested {
                    fields: vec![
                        FieldMapping {
                            field_name: Name::new_static_unchecked("name"),
                            expression: MappingExpression::Direct,
                        },
                        FieldMapping {
                            field_name: Name::new_static_unchecked("tag"),
                            expression: MappingExpression::Alias(Name::new_static_unchecked(
                                "variant_tag"
                            ))
                        },
                        FieldMapping {
                            field_name: Name::new_static_unchecked("price"),
                            expression: MappingExpression::Nested {
                                fields: vec![
                                    FieldMapping {
                                        field_name: Name::new_static_unchecked("original"),
                                        expression: MappingExpression::Direct,
                                    },
                                    FieldMapping {
                                        field_name: Name::new_static_unchecked("final"),
                                        expression: MappingExpression::Direct,
                                    },
                                ]
                            },
                        },
                    ]
                }
            }]
        );
    }
}
