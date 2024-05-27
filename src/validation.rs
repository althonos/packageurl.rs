/// Check whether a type is valid regarding the specification.
pub fn is_type_valid(ty: &str) -> bool {
    let first = match ty.chars().next() {
        Some(c) => c,
        None => return false,
    };
    if first.is_ascii_digit() {
        return false;
    }

    #[allow(clippy::match_like_matches_macro)]
    ty.chars().all(|c| match c {
        '.' | '-' | '+' | 'a'..='z' | 'A'..='Z' | '0'..='9' => true,
        _ => false,
    })
}

/// Check whether a qualifier key is valid regarding the specification.
pub fn is_qualifier_key_valid(key: &str) -> bool {
    // check the key doesn't start with a digit
    let first = match key.chars().next() {
        Some(c) => c,
        None => return false,
    };
    if first.is_ascii_digit() {
        return false;
    }

    // check the key contains only valid characters
    // The key must be composed only of ASCII letters and numbers, '.', '-' and '_' (period, dash and underscore)
    #[allow(clippy::match_like_matches_macro)]
    key.chars().all(|c| match c {
        '.' | '-' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => true,
        _ => false,
    })
}

/// Check whether a namespace component is valid regarding the specification.
pub fn is_namespace_component_valid(component: &str) -> bool {
    !component.is_empty() && !component.chars().any(|c| c == '/')
}

/// Check whether a subpath segment is valid regarding the specification.
pub fn is_subpath_segment_valid(segment: &str) -> bool {
    !segment.is_empty() && segment != "." && segment != ".." && !segment.chars().any(|c| c == '/')
}
