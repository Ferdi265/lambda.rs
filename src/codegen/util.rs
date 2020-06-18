use super::*;

pub fn is_reserved(name: &str, words: &[&str]) -> bool {
    words.contains(&name)
}

pub fn is_numeric(name: &str) -> bool {
    name.starts_with(char::is_numeric)
}

pub fn is_underscore(name: &str) -> bool {
    name.starts_with('_')
}

pub fn is_end_underscore(name: &str) -> bool {
    name.ends_with('_')
}

pub fn generate_identifier(ident: Identifier<'_>, words: &[&str]) -> String {
    if is_reserved(ident, words) || is_numeric(ident) || is_underscore(ident) {
        format!("_{}", ident)
    } else {
        ident.to_string()
    }
}
