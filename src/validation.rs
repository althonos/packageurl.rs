/// Check whether a qualifier key is valid regarding to the specification.
pub fn is_qualifier_key_valid(key: &str) -> bool {
    // check the key doesn't start with a digit
    let first = match key.chars().next() {
        Some(c) => c,
        None => return false,
    };
    if first.is_digit(10) {
        return false;
    }

    // check the key contains only valid characters
    // The key must be composed only of ASCII letters and numbers, '.', '-' and '_' (period, dash and underscore)
    key.chars().all(|c| match c {
        '.' | '-' | '_' | 'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' => true,
        _ => false,
    })
}
