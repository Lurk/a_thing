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
                    } else {
                        return false;
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
