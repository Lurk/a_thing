use crate::dict_filters::DictFilters;
use crate::utils::read_lines;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Dict {
    inner: Vec<String>,
}

impl Dict {
    pub fn from_file(path: &str) -> Self {
        let v: Vec<String> = if let Ok(lines) = read_lines(path) {
            lines
                .filter_map(|line| {
                    if let Ok(word) = line {
                        return Some(word);
                    }
                    None
                })
                .collect()
        } else {
            vec![]
        };
        Self::from_vec(v)
    }

    pub fn from_vec(v: Vec<String>) -> Self {
        Self { inner: v }
    }

    pub fn get_char_freq(&self) -> HashMap<char, usize> {
        let mut freq: HashMap<char, usize> = HashMap::new();
        for w in &self.inner {
            for char in w.chars() {
                let count = freq.entry(char).or_insert(0);
                *count += 1;
            }
        }
        freq
    }

    pub fn most_common(&self, count: usize) -> Self {
        let freq = self.get_char_freq();
        let mut words_with_weight: HashMap<String, usize> = HashMap::new();
        for word in &self.inner {
            let count = words_with_weight.entry(word.to_string()).or_insert(0);
            let mut chars: Vec<char> = vec![];
            for char in word.chars() {
                if !chars.contains(&char) {
                    *count += freq.get(&char).unwrap_or(&0);
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

    pub fn contains(self, chars: &str) -> DictFilters<'_> {
        DictFilters::new(Box::new(self.inner.into_iter())).contains(chars)
    }
    pub fn not_contains(self, chars: &str) -> DictFilters<'_> {
        DictFilters::new(Box::new(self.inner.into_iter())).not_contains(chars)
    }
}
