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
