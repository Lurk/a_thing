use crate::dict_iter::DictIter;
use crate::utils::read_lines;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Dict {
    iner: Vec<String>,
}

impl Dict {
    pub fn from_file(path: &str) -> Self {
        let i: Vec<String> = if let Ok(lines) = read_lines(path) {
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
        return Self { iner: i };
    }

    pub fn from_vec(v: Vec<String>) -> Self {
        Self { iner: v }
    }

    pub fn get_char_freq(&self) -> HashMap<char, usize> {
        let mut h: HashMap<char, usize> = HashMap::new();
        for w in &self.iner {
            for char in w.chars() {
                let count = h.entry(char).or_insert(0);
                *count += 1;
            }
        }
        h
    }

    pub fn most_common(&self, count: usize) -> Self {
        let freq = self.get_char_freq();
        let mut words_with_weight: HashMap<String, usize> = HashMap::new();
        for word in &self.iner {
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
        Self::from_vec(
            v[0..n]
                .into_iter()
                .map(|(word, _)| word.to_string())
                .collect(),
        )
    }

    pub fn filter_by_length<'a>(self, len: usize) -> DictIter<'a> {
        DictIter::new(Box::new(self.iner.into_iter())).filter_by_length(len)
    }

    pub fn starts_with<'a>(self, s: &'a str) -> DictIter<'a> {
        DictIter::new(Box::new(self.iner.into_iter())).starts_with(s)
    }

    pub fn ends_with<'a>(self, s: &'a str) -> DictIter<'a> {
        DictIter::new(Box::new(self.iner.into_iter())).ends_with(s)
    }

    pub fn contains<'a>(self, chars: &'a str) -> DictIter<'a> {
        DictIter::new(Box::new(self.iner.into_iter())).contains(chars)
    }
    pub fn not_contains<'a>(self, chars: &'a str) -> DictIter<'a> {
        DictIter::new(Box::new(self.iner.into_iter())).not_contains(chars)
    }
}
