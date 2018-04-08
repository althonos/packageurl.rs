//! utils.rs

pub trait JoinableIterator: Iterator {
    fn join(&mut self, separator: &str) -> String;
}

impl<T: ?Sized, I> JoinableIterator for T
where
    T: Iterator<Item = I>,
    I: AsRef<str>,
{
    fn join(&mut self, separator: &str) -> String {
        let mut st = String::new();
        let mut pair = (self.next(), self.next());

        while let Some(next) = pair.1 {
            st.push_str(pair.0.unwrap().as_ref());
            st.push_str(separator);
            pair = (Some(next), self.next());
        }

        if let Some(last) = pair.0 {
            st.push_str(last.as_ref())
        }

        st
    }
}

pub trait QuickFind {
    fn quickfind(&self, needle: u8) -> Option<usize>;
    fn quickrfind(&self, needle: u8) -> Option<usize>;
}

impl<T> QuickFind for T
where
    T: AsRef<str>,
{
    #[cfg(not(feature = "memchr"))]
    fn quickfind(&self, needle: u8) -> Option<usize> {
        self.as_ref().find(char::from(needle))
    }

    #[cfg(not(feature = "memchr"))]
    fn quickrfind(&self, needle: u8) -> Option<usize> {
        self.as_ref().rfind(char::from(needle))
    }

    #[cfg(feature = "memchr")]
    fn quickfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memchr(needle, self.as_ref().as_bytes())
    }

    #[cfg(feature = "memchr")]
    fn quickrfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memrchr(needle, self.as_ref().as_bytes())
    }
}

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
    fn test_join() {
        let src = vec!["abc", "def", "ghi"];
        let dst = src.iter().join("/");
        assert_eq!(dst, "abc/def/ghi");

        let src = vec!["abc"];
        let dst = src.iter().join("/");
        assert_eq!(dst, "abc");

        let src: Vec<&str> = vec![];
        let dst = src.iter().join("/");
        assert_eq!(dst, "");
    }

}
