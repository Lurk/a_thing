pub struct Filters<'filter_lifetime> {
    inner: Box<dyn Iterator<Item = &'filter_lifetime String> + 'filter_lifetime>,
}

impl<'filter_lifetime> Filters<'filter_lifetime> {
    pub fn new(
        iter: Box<dyn Iterator<Item = &'filter_lifetime String> + 'filter_lifetime>,
    ) -> Self {
        Self { inner: iter }
    }

    pub fn filter_by_length(mut self, len: usize) -> Self {
        self.inner = Box::new(self.inner.filter(move |word| word.chars().count() == len));
        self
    }

    pub fn starts_with(mut self, s: &'filter_lifetime str) -> Self {
        if !s.is_empty() {
            self.inner = Box::new(self.inner.filter(move |word| word.starts_with(s)));
        }
        self
    }

    pub fn ends_with(mut self, s: &'filter_lifetime str) -> Self {
        if !s.is_empty() {
            self.inner = Box::new(self.inner.filter(move |word| word.ends_with(s)));
        }
        self
    }

    pub fn contains_str(mut self, s: &'filter_lifetime str) -> Self {
        if !s.is_empty() {
            self.inner = Box::new(self.inner.filter(move |word| word.contains(s)));
        }
        self
    }

    pub fn not_contains_str(mut self, s: &'filter_lifetime str) -> Self {
        if !s.is_empty() {
            self.inner = Box::new(self.inner.filter(move |word| !word.contains(s)));
        }
        self
    }

    pub fn contains_chars(mut self, chars: &'filter_lifetime str) -> Self {
        if !chars.is_empty() {
            self.inner = Box::new(
                self.inner
                    .filter(move |word| chars.chars().all(|char| word.contains(char))),
            );
        }
        self
    }

    pub fn not_contains_chars(mut self, chars: &'filter_lifetime str) -> Self {
        if !chars.is_empty() {
            self.inner = Box::new(
                self.inner
                    .filter(move |word| chars.chars().all(|char| !word.contains(char))),
            );
        }
        self
    }

    pub fn positional_contains_chars(mut self, chars: &'filter_lifetime [Option<char>]) -> Self {
        self.inner = Box::new(self.inner.filter(|word| {
            for (i, char) in chars.iter().enumerate() {
                if let Some(lhs) = char {
                    if let Some(rhs) = word.chars().nth(i) {
                        if *lhs != rhs {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
            true
        }));
        self
    }

    pub fn positional_not_contains_chars(
        mut self,
        chars: &'filter_lifetime [Option<char>],
    ) -> Self {
        self.inner = Box::new(self.inner.filter(|word| {
            for (i, char) in chars.iter().enumerate() {
                if let Some(lhs) = char {
                    if let Some(rhs) = word.chars().nth(i) {
                        if *lhs == rhs {
                            return false;
                        }
                    }
                }
            }
            true
        }));
        self
    }

    pub fn take(mut self, n: usize) -> Self {
        self.inner = Box::new(self.inner.take(n));
        self
    }

    pub fn apply(self) -> Vec<String> {
        self.inner.map(|s| s.to_string()).collect()
    }
}

pub fn filter_by_length(dict: &[String], len: usize) -> Filters<'_> {
    Filters::new(Box::new(dict.iter())).filter_by_length(len)
}

pub fn starts_with<'filter_lifetime>(
    dict: &'filter_lifetime [String],
    s: &'filter_lifetime str,
) -> Filters<'filter_lifetime> {
    Filters::new(Box::new(dict.iter())).starts_with(s)
}

pub fn ends_with<'filter_lifetime>(
    dict: &'filter_lifetime [String],
    s: &'filter_lifetime str,
) -> Filters<'filter_lifetime> {
    Filters::new(Box::new(dict.iter())).ends_with(s)
}

pub fn contains_str<'filter_lifetime>(
    dict: &'filter_lifetime [String],
    s: &'filter_lifetime str,
) -> Filters<'filter_lifetime> {
    Filters::new(Box::new(dict.iter())).contains_str(s)
}

pub fn not_contains_str<'filter_lifetime>(
    dict: &'filter_lifetime [String],
    s: &'filter_lifetime str,
) -> Filters<'filter_lifetime> {
    Filters::new(Box::new(dict.iter())).not_contains_str(s)
}

pub fn contains_chars<'filter_lifetime>(
    dict: &'filter_lifetime [String],
    chars: &'filter_lifetime str,
) -> Filters<'filter_lifetime> {
    Filters::new(Box::new(dict.iter())).contains_chars(chars)
}
pub fn not_contains_chars<'filter_lifetime>(
    dict: &'filter_lifetime [String],
    chars: &'filter_lifetime str,
) -> Filters<'filter_lifetime> {
    Filters::new(Box::new(dict.iter())).not_contains_chars(chars)
}

pub fn positional_contains_chars<'filter_lifetime>(
    dict: &'filter_lifetime [String],
    chars: &'filter_lifetime [Option<char>],
) -> Filters<'filter_lifetime> {
    Filters::new(Box::new(dict.iter())).positional_contains_chars(chars)
}

pub fn positional_not_contains_chars<'filter_lifetime>(
    dict: &'filter_lifetime [String],
    chars: &'filter_lifetime [Option<char>],
) -> Filters<'filter_lifetime> {
    Filters::new(Box::new(dict.iter())).positional_not_contains_chars(chars)
}

#[cfg(test)]
mod tests {

    use crate::filters::{
        contains_chars, contains_str, ends_with, filter_by_length, not_contains_chars,
        not_contains_str, positional_contains_chars, positional_not_contains_chars, starts_with,
    };

    fn test_dict() -> [String; 5] {
        [
            "foo".to_string(),
            "bfoo".to_string(),
            "foobar".to_string(),
            "foobarbaz".to_string(),
            "bfoobarbaz".to_string(),
        ]
    }

    #[test]
    fn by_length() -> () {
        let res = filter_by_length(&test_dict(), 6).apply();

        assert_eq!(res.len(), 1);
        assert_eq!(res[0], "foobar")
    }
    #[test]
    fn starts_with_test() -> () {
        let res = starts_with(&test_dict(), "foo").apply();

        assert_eq!(res.len(), 3);
        assert_eq!(res[0], "foo");
        assert_eq!(res[1], "foobar");
        assert_eq!(res[2], "foobarbaz");
    }
    #[test]
    fn ends_with_test() -> () {
        let res = ends_with(&test_dict(), "baz").apply();

        assert_eq!(res.len(), 2);
        assert_eq!(res[0], "foobarbaz");
        assert_eq!(res[1], "bfoobarbaz");
    }
    #[test]
    fn contains_str_test() -> () {
        let res = contains_str(&test_dict(), "bar").apply();

        assert_eq!(res.len(), 3);
        assert_eq!(res[0], "foobar");
        assert_eq!(res[1], "foobarbaz");
        assert_eq!(res[2], "bfoobarbaz");
    }
    #[test]
    fn not_contains_str_test() -> () {
        let res = not_contains_str(&test_dict(), "bar").apply();

        assert_eq!(res[0], "foo");
        assert_eq!(res[1], "bfoo");
        assert_eq!(res.len(), 2);
    }
    #[test]
    fn contains_chars_test() -> () {
        let res = contains_chars(&test_dict(), "br").apply();

        assert_eq!(res.len(), 3);
        assert_eq!(res[0], "foobar");
        assert_eq!(res[1], "foobarbaz");
        assert_eq!(res[2], "bfoobarbaz");
    }
    #[test]
    fn not_contains_chars_test() -> () {
        let res = not_contains_chars(&test_dict(), "bz").apply();

        assert_eq!(res.len(), 1);
        assert_eq!(res[0], "foo");
    }
    #[test]
    fn positional_contains_chars_test() -> () {
        let res =
            positional_contains_chars(&test_dict(), &[None, None, Some('o'), None, Some('a')])
                .apply();

        assert_eq!(res.len(), 2);
        assert_eq!(res[0], "foobar");
        assert_eq!(res[1], "foobarbaz");
    }
    #[test]
    fn positional_not_contains_chars_test() -> () {
        let res = positional_not_contains_chars(&test_dict(), &[None, Some('f')]).apply();

        assert_eq!(res.len(), 3);
        assert_eq!(res[0], "foo");
        assert_eq!(res[1], "foobar");
        assert_eq!(res[2], "foobarbaz");
    }
}
