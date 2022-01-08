use crate::dict::Dict;

pub struct DictFilters<'a> {
    inner: Box<dyn Iterator<Item = String> + 'a>,
}

impl<'a> DictFilters<'a> {
    pub fn new(iter: Box<dyn Iterator<Item = String> + 'a>) -> Self {
        Self { inner: iter }
    }

    pub fn filter_by_length(mut self, len: usize) -> Self {
        self.inner = Box::new(self.inner.filter(move |word| word.chars().count() == len));
        self
    }

    pub fn starts_with(mut self, s: &'a str) -> Self {
        if !s.is_empty() {
            self.inner = Box::new(self.inner.filter(move |word| word.starts_with(s)));
        }
        self
    }

    pub fn ends_with(mut self, s: &'a str) -> Self {
        if !s.is_empty() {
            self.inner = Box::new(self.inner.filter(move |word| word.ends_with(s)));
        }
        self
    }

    pub fn contains_str(mut self, s: &'a str) -> Self {
        if !s.is_empty() {
            self.inner = Box::new(self.inner.filter(move |word| word.contains(s)));
        }
        self
    }

    pub fn not_contains_str(mut self, s: &'a str) -> Self {
        if !s.is_empty() {
            self.inner = Box::new(self.inner.filter(move |word| !word.contains(s)));
        }
        self
    }

    pub fn contains_chars(mut self, chars: &'a str) -> Self {
        if !chars.is_empty() {
            self.inner = Box::new(
                self.inner
                    .filter(move |word| chars.chars().all(|char| word.contains(char))),
            );
        }
        self
    }

    pub fn not_contains_chars(mut self, chars: &'a str) -> Self {
        if !chars.is_empty() {
            self.inner = Box::new(
                self.inner
                    .filter(move |word| chars.chars().all(|char| !word.contains(char))),
            );
        }
        self
    }

    pub fn positional_contains_chars(mut self, chars: &'a [Option<char>]) -> Self {
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

    pub fn positional_not_contains_chars(mut self, chars: &'a [Option<char>]) -> Self {
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

    pub fn apply(self) -> Dict {
        Dict::from_vec(self.inner.collect())
    }
}

#[cfg(test)]
mod tests {
    use super::DictFilters;

    #[test]
    fn filter_by_length() -> () {
        let res = DictFilters::new(Box::new(
            [
                "foo".to_string(),
                "foobar".to_string(),
                "foobarbaz".to_string(),
            ]
            .into_iter(),
        ))
        .filter_by_length(6)
        .apply();

        assert!(res.len() == 1);
        assert!(res.at(0) == "foobar")
    }
    #[test]
    fn starts_with() -> () {
        let res = DictFilters::new(Box::new(
            [
                "bfoo".to_string(),
                "foobar".to_string(),
                "bfoobarbaz".to_string(),
                "foobarbaz".to_string(),
            ]
            .into_iter(),
        ))
        .starts_with("foo")
        .apply();

        assert!(res.len() == 2);
        assert!(res.at(0) == "foobar");
        assert!(res.at(1) == "foobarbaz");
    }
    #[test]
    fn ends_with() -> () {
        let res = DictFilters::new(Box::new(
            [
                "foo".to_string(),
                "foobar".to_string(),
                "foobarbaz".to_string(),
                "foobarbazbar".to_string(),
            ]
            .into_iter(),
        ))
        .ends_with("bar")
        .apply();

        assert!(res.len() == 2);
        assert!(res.at(0) == "foobar");
        assert!(res.at(1) == "foobarbazbar");
    }
    #[test]
    fn contains_str() -> () {
        let res = DictFilters::new(Box::new(
            [
                "foo".to_string(),
                "foobar".to_string(),
                "foobarbaz".to_string(),
            ]
            .into_iter(),
        ))
        .contains_str("bar")
        .apply();

        assert!(res.len() == 2);
        assert!(res.at(0) == "foobar");
        assert!(res.at(1) == "foobarbaz");
    }
    #[test]
    fn not_contains_str() -> () {
        let res = DictFilters::new(Box::new(
            [
                "foo".to_string(),
                "foobar".to_string(),
                "foobarbaz".to_string(),
            ]
            .into_iter(),
        ))
        .not_contains_str("bar")
        .apply();

        assert!(res.at(0) == "foo");
        assert!(res.len() == 1);
    }
    #[test]
    fn contains_chars() -> () {
        let res = DictFilters::new(Box::new(
            [
                "foo".to_string(),
                "foobar".to_string(),
                "foobarbaz".to_string(),
            ]
            .into_iter(),
        ))
        .contains_chars("br")
        .apply();

        assert!(res.len() == 2);
        assert!(res.at(0) == "foobar");
        assert!(res.at(1) == "foobarbaz");
    }
    #[test]
    fn not_contains_chars() -> () {
        let res = DictFilters::new(Box::new(
            [
                "foo".to_string(),
                "foobar".to_string(),
                "foobarbaz".to_string(),
            ]
            .into_iter(),
        ))
        .not_contains_chars("bz")
        .apply();

        assert!(res.len() == 1);
        assert!(res.at(0) == "foo");
    }
    #[test]
    fn positional_contains_chars() -> () {
        let res = DictFilters::new(Box::new(
            [
                "foo".to_string(),
                "foobar".to_string(),
                "foobarbaz".to_string(),
                "fobarbaz".to_string(),
            ]
            .into_iter(),
        ))
        .positional_contains_chars(&[None, None, Some('o'), None, Some('a')])
        .apply();

        assert!(res.len() == 2);
        assert!(res.at(0) == "foobar");
        assert!(res.at(1) == "foobarbaz");
    }
    #[test]
    fn positional_not_contains_chars() -> () {
        let res = DictFilters::new(Box::new(
            [
                "foo".to_string(),
                "baz".to_string(),
                "foobar".to_string(),
                "foobarbaz".to_string(),
                "fozbarbaz".to_string(),
                "fobarbaz".to_string(),
            ]
            .into_iter(),
        ))
        .positional_not_contains_chars(&[None, None, Some('o'), None, Some('a')])
        .apply();

        assert!(res.len() == 2);
        assert!(res.at(0) == "baz");
        assert!(res.at(1) == "fobarbaz");
    }
}
