use syn::*;

pub fn str_value_of_meta_item(item: &Meta, name: &str) -> String {
    match *item {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(ref value),
            ..
        }) => value.value(),
        _ => panic!(
            r#"`{}` must be in the form `#[{}="something"]`"#,
            name, name
        ),
    }
}

pub fn get_options_from_input(
    name: &Path,
    attrs: &[Attribute],
    on_bug: fn() -> !,
) -> Option<Vec<Meta>> {
    let options = attrs
        .iter()
        .find(|a| &a.path == name)
        .map(Attribute::parse_meta);
    match options {
        Some(Ok(Meta::List(MetaList { ref nested, .. }))) => Some(
            nested
                .iter()
                .map(|o| match *o {
                    NestedMeta::Meta(ref m) => m.clone(),
                    _ => on_bug(),
                })
                .collect(),
        ),
        Some(_) => on_bug(),
        None => None,
    }
}

pub fn get_option(options: &[Meta], option_name: &str, on_bug: fn() -> !) -> String {
    get_optional_option(options, option_name).unwrap_or_else(|| on_bug())
}

pub fn get_optional_option(options: &[Meta], option_name: &str) -> Option<String> {
    options
        .iter()
        .find(|a| a.name() == option_name)
        .map(|a| str_value_of_meta_item(a, option_name))
}

pub fn get_rust_migrations_from_input(
    name: &Path,
    attrs: &[Attribute],
    on_bug: fn() -> !,
) -> Option<Vec<String>> {
    let options = attrs
        .iter()
        .find(|a| &a.path == name)
        .map(Attribute::parse_meta);
    match options {
        Some(Ok(Meta::List(MetaList { ref nested, .. }))) => Some(
            nested
                .iter()
                .map(|o| match *o {
                    NestedMeta::Literal(Lit::Str(ref s)) => s.value(),
                    _ => {
                        panic!(r#"Rust migrations must be given in the form `["expr1", "expr2", ...]`. Each expr must evaluate to a type that impls `Migration`."#);
                    }
                })
                .collect(),
        ),
        Some(_) => on_bug(),
        None => None,
    }
}
