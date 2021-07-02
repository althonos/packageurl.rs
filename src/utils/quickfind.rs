pub trait QuickFind {
    fn quickfind(&self, needle: u8) -> Option<usize>;
    fn quickrfind(&self, needle: u8) -> Option<usize>;
}

#[cfg(not(feature = "memchr"))]
impl<T> QuickFind for T
where
    T: AsRef<str>,
{
    fn quickfind(&self, needle: u8) -> Option<usize> {
        self.as_ref().find(char::from(needle))
    }
    fn quickrfind(&self, needle: u8) -> Option<usize> {
        self.as_ref().rfind(char::from(needle))
    }
}

#[cfg(feature = "memchr")]
impl<T> QuickFind for T
where
    T: AsRef<str>,
{
    fn quickfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memchr(needle, self.as_ref().as_bytes())
    }
    fn quickrfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memrchr(needle, self.as_ref().as_bytes())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_quickfind() {
        let buf = "Hello, world !";
        assert_eq!(buf.quickfind(b'o'), Some(4));
        assert_eq!(buf.quickfind(b'c'), None);
    }

    #[test]
    fn test_quickrfind() {
        let buf = "Hello, world !";
        assert_eq!(buf.quickrfind(b'o'), Some(8));
        assert_eq!(buf.quickrfind(b'c'), None)
    }
}
