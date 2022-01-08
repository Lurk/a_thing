use crate::dict_filters::DictFilters;
use crate::utils::read_lines;
use std::collections::HashMap;
use std::io::Error;

#[derive(Debug)]
pub struct Dict {
    inner: Vec<String>,
}

type Freq = HashMap<char, HashMap<usize, usize>>;

impl Dict {
    pub fn from_file(path: &str) -> Result<Self, Error> {
        let lines = read_lines(path)?;
        Ok(Self::from_vec(
            lines
                .filter_map(|line| {
                    if let Ok(word) = line {
                        return Some(word);
                    }
                    None
                })
                .collect(),
        ))
    }

    pub fn from_file_with_len(path: &str, len: usize) -> Result<Self, Error> {
        let lines = read_lines(path)?;
        Ok(Self::from_vec(
            lines
                .filter_map(|line| {
                    if let Ok(word) = line {
                        if word.chars().count() == len {
                            return Some(word);
                        }
                    }
                    None
                })
                .collect(),
        ))
    }

    pub fn from_vec(v: Vec<String>) -> Self {
        Self { inner: v }
    }

    pub fn get_char_freq(&self) -> Freq {
        let mut freq: Freq = HashMap::new();
        for w in self.inner.iter() {
            for (i, char) in w.chars().enumerate() {
                let count = freq.entry(char).or_insert_with(HashMap::new);
                let positional_count = count.entry(i).or_insert(0);
                *positional_count += 1;
            }
        }
        freq
    }

    pub fn most_common(&self, freq: &Freq, count: usize) -> Self {
        let mut words_with_weight: HashMap<String, usize> = HashMap::new();
        for word in &self.inner {
            let count = words_with_weight.entry(word.to_string()).or_insert(0);
            let mut chars: Vec<char> = vec![];
            for (i, char) in word.chars().enumerate() {
                if !chars.contains(&char) {
                    if let Some(char_freq) = freq.get(&char) {
                        *count += char_freq.get(&i).unwrap_or(&0);
                    }
                    chars.push(char);
                }
            }
        }
        let mut v: Vec<_> = words_with_weight.iter().clone().collect();
        v.sort_by(|a, b| b.1.cmp(a.1));
        let n = if v.len() >= count { count } else { v.len() };
        Self::from_vec(v[0..n].iter().map(|(word, _)| word.to_string()).collect())
    }

    pub fn filter_by_length<'a>(self, len: usize) -> DictFilters<'a> {
        DictFilters::new(Box::new(self.inner.into_iter())).filter_by_length(len)
    }

    pub fn starts_with(self, s: &str) -> DictFilters<'_> {
        DictFilters::new(Box::new(self.inner.into_iter())).starts_with(s)
    }

    pub fn ends_with(self, s: &str) -> DictFilters<'_> {
        DictFilters::new(Box::new(self.inner.into_iter())).ends_with(s)
    }

    pub fn contains_str(self, s: &str) -> DictFilters<'_> {
        DictFilters::new(Box::new(self.inner.into_iter())).contains_str(s)
    }

    pub fn not_contains_str(self, s: &str) -> DictFilters<'_> {
        DictFilters::new(Box::new(self.inner.into_iter())).not_contains_str(s)
    }

    pub fn contains_chars(self, chars: &str) -> DictFilters<'_> {
        DictFilters::new(Box::new(self.inner.into_iter())).contains_chars(chars)
    }
    pub fn not_contains_chars(self, chars: &str) -> DictFilters<'_> {
        DictFilters::new(Box::new(self.inner.into_iter())).not_contains_chars(chars)
    }

    pub fn positional_contains_chars(self, chars: &[Option<char>]) -> DictFilters<'_> {
        DictFilters::new(Box::new(self.inner.into_iter())).positional_contains_chars(chars)
    }

    pub fn positional_not_contains_chars(self, chars: &[Option<char>]) -> DictFilters<'_> {
        DictFilters::new(Box::new(self.inner.into_iter())).positional_not_contains_chars(chars)
    }
}
