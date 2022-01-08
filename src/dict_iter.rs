use crate::dict::Dict;

pub struct DictIter<'a> {
    inner: Box<dyn Iterator<Item = String> + 'a>,
}

impl<'a> DictIter<'a> {
    pub fn new(iter: Box<dyn Iterator<Item = String> + 'a>) -> Self {
        Self { inner: iter }
    }

    pub fn filter_by_length(mut self, len: usize) -> Self {
        self.inner = Box::new(self.inner.filter(move |word| word.chars().count() == len));
        self
    }

    pub fn starts_with(mut self, s: &'a str) -> Self {
        if s.len() > 0 {
            self.inner = Box::new(self.inner.filter(move |word| word.starts_with(s)));
        }
        self
    }

    pub fn ends_with(mut self, s: &'a str) -> Self {
        if s.len() > 0 {
            self.inner = Box::new(self.inner.filter(move |word| word.ends_with(s)));
        }
        self
    }

    pub fn contains(mut self, chars: &'a str) -> Self {
        if chars.len() > 0 {
            self.inner = Box::new(
                self.inner
                    .filter(move |word| chars.chars().all(|char| word.contains(char))),
            );
        }
        self
    }

    pub fn not_contains(mut self, chars: &'a str) -> Self {
        if chars.len() > 0 {
            self.inner = Box::new(
                self.inner
                    .filter(move |word| chars.chars().all(|char| !word.contains(char))),
            );
        }
        self
    }

    pub fn apply(self) -> Dict {
        Dict::from_vec(self.inner.collect())
    }
}
