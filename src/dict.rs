use crate::dict_filters::DictFilters;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Error;

#[derive(Debug)]
pub struct Dict {
    inner: Vec<String>,
}

type CharPositionFreq = HashMap<char, HashMap<usize, usize>>;
type CharFreq = HashMap<char, usize>;

#[derive(Debug)]
pub enum FreqType {
    CharFreq(CharFreq),
    CharPositionFreq(CharPositionFreq),
}

impl Dict {
    pub fn from_file(path: &str) -> Result<Self, Error> {
        let contents = read_to_string(path)?;

        Ok(Self::from_vec(
            contents.rsplit('\n').map(|w| w.to_string()).collect(),
        ))
    }

    pub fn from_vec(v: Vec<String>) -> Self {
        Self { inner: v }
    }

    pub fn get_char_position_freq(&self) -> FreqType {
        let mut freq: CharPositionFreq = HashMap::new();
        for w in self.inner.iter() {
            for (i, char) in w.chars().enumerate() {
                let count = freq.entry(char).or_insert_with(HashMap::new);
                let positional_count = count.entry(i).or_insert(0);
                *positional_count += 1;
            }
        }
        FreqType::CharPositionFreq(freq)
    }

    pub fn get_char_freq(&self) -> FreqType {
        let mut freq: CharFreq = HashMap::new();
        for w in self.inner.iter() {
            for char in w.chars() {
                let count = freq.entry(char).or_insert(0);
                *count += 1;
            }
        }
        FreqType::CharFreq(freq)
    }

    pub fn most_common(&self, freq: &FreqType, count: usize) -> Self {
        let words_with_weight: HashMap<&String, usize> = match freq {
            FreqType::CharFreq(f) => self.most_common_by_char(f),
            FreqType::CharPositionFreq(f) => self.most_common_by_char_position(f),
        };

        let mut v: Vec<_> = words_with_weight.iter().collect();
        v.sort_by(|a, b| b.1.cmp(a.1));
        let n = if v.len() >= count { count } else { v.len() };
        Self::from_vec(v[0..n].iter().map(|(word, _)| word.to_string()).collect())
    }

    fn most_common_by_char(&self, freq: &CharFreq) -> HashMap<&String, usize> {
        let mut words_with_weight = HashMap::new();
        for word in &self.inner {
            let count = words_with_weight.entry(word).or_insert(0);
            let mut chars: Vec<char> = Vec::with_capacity(word.chars().count());
            for char in word.chars() {
                if !chars.contains(&char) {
                    if let Some(char_freq) = freq.get(&char) {
                        *count += char_freq;
                    }
                }

                chars.push(char);
            }
        }
        words_with_weight
    }

    fn most_common_by_char_position(&self, freq: &CharPositionFreq) -> HashMap<&String, usize> {
        let mut words_with_weight = HashMap::new();
        for word in &self.inner {
            let count = words_with_weight.entry(word).or_insert(0);
            let mut used_chars: Vec<char> = Vec::with_capacity(word.chars().count());
            for (i, char) in word.chars().enumerate() {
                if !used_chars.contains(&char) {
                    if let Some(char_freq) = freq.get(&char) {
                        *count += char_freq.get(&i).unwrap_or(&0);
                    }

                    used_chars.push(char);
                }
            }
        }
        words_with_weight
    }

    pub fn at(&self, index: usize) -> &String {
        &self.inner[index]
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }

    pub fn to_vec(self) -> Vec<String> {
        self.inner
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
