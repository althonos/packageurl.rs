mod encodable;
mod quickfind;

pub use self::encodable::PercentCodec;
pub use self::quickfind::QuickFind;

pub fn rcut(input: &str, sep: u8) -> (&str, &str) {
    if let Some(i) = input.quickrfind(sep) {
        (&input[..i], &input[i + 1..])
    } else {
        ("", input)
    }
}

pub fn cut(input: &str, sep: u8) -> (&str, &str) {
    if let Some(i) = input.quickfind(sep) {
        (&input[..i], &input[i + 1..])
    } else {
        (input, "")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cut() {
        let buf = "A:B:C";
        assert_eq!(cut(buf, b':'), ("A", "B:C"));
        assert_eq!(cut(buf, b','), ("A:B:C", ""));
    }

    #[test]
    fn test_rcut() {
        let buf = "A:B:C";
        assert_eq!(rcut(buf, b':'), ("A:B", "C"));
        assert_eq!(rcut(buf, b','), ("", "A:B:C"));
    }
}
