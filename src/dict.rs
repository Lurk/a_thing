use crate::filters::Filters;

use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Error;

#[derive(Debug)]
pub struct Dict {
    inner: Vec<String>,
}

/// Here we are assuming that no word is longer than 64 chars
/// -   eng:    Pneumonoultramicroscopicsilicovolcanoconiosis
/// -   de:     Rindfleischetikettierungsüberwachungsaufgabenübertragungsgesetz
///
/// if this is a problem for you, please fill the issue
pub type CharPositionWeights = HashMap<char, [usize; 64]>;
pub type CharWeights = HashMap<char, usize>;

#[derive(Debug)]
pub enum WeghtsType {
    CharWeights(CharWeights),
    CharPositionWeights(CharPositionWeights),
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

    pub fn get_char_position_weights(&self) -> WeghtsType {
        let mut freq: CharPositionWeights = HashMap::new();
        for w in self.inner.iter() {
            for (i, char) in w.chars().enumerate() {
                if i > 63 {
                    // Check CharPositionFreq documentation
                    unreachable!()
                }
                let count = freq.entry(char).or_insert([0; 64]);
                count[i] += 1;
            }
        }
        WeghtsType::CharPositionWeights(freq)
    }

    pub fn get_char_weights(&self) -> WeghtsType {
        let mut freq: CharWeights = HashMap::new();
        for w in self.inner.iter() {
            for char in w.chars() {
                let count = freq.entry(char).or_insert(0);
                *count += 1;
            }
        }
        WeghtsType::CharWeights(freq)
    }

    pub fn most_common(&self, freq: &WeghtsType, count: usize) -> Self {
        let words_with_weight: HashMap<&String, usize> = match freq {
            WeghtsType::CharWeights(f) => self.most_common_by_char(f),
            WeghtsType::CharPositionWeights(f) => self.most_common_by_char_position(f),
        };

        let mut v: Vec<_> = words_with_weight.iter().collect();
        v.sort_by(|a, b| b.1.cmp(a.1));
        let n = if v.len() >= count { count } else { v.len() };
        Self::from_vec(v[0..n].iter().map(|(word, _)| word.to_string()).collect())
    }

    fn most_common_by_char(&self, freq: &CharWeights) -> HashMap<&String, usize> {
        let mut words_with_weight = HashMap::new();
        for word in &self.inner {
            let count = words_with_weight.entry(word).or_insert(0);
            let mut used_chars: Vec<char> = Vec::with_capacity(word.chars().count());
            for char in word.chars() {
                if !used_chars.contains(&char) {
                    if let Some(char_freq) = freq.get(&char) {
                        *count += char_freq;
                    }
                    used_chars.push(char);
                }
            }
        }
        words_with_weight
    }

    fn most_common_by_char_position(&self, freq: &CharPositionWeights) -> HashMap<&String, usize> {
        let mut words_with_weight = HashMap::new();
        for word in &self.inner {
            let count = words_with_weight.entry(word).or_insert(0);
            let mut used_chars: Vec<char> = Vec::with_capacity(word.chars().count());
            for (i, char) in word.chars().enumerate() {
                if !used_chars.contains(&char) {
                    if let Some(char_freq) = freq.get(&char) {
                        *count += char_freq[i];
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

    pub fn filter_by_length<'a>(self, len: usize) -> Filters<'a> {
        Filters::new(Box::new(self.inner.into_iter())).filter_by_length(len)
    }

    pub fn starts_with(self, s: &str) -> Filters<'_> {
        Filters::new(Box::new(self.inner.into_iter())).starts_with(s)
    }

    pub fn ends_with(self, s: &str) -> Filters<'_> {
        Filters::new(Box::new(self.inner.into_iter())).ends_with(s)
    }

    pub fn contains_str(self, s: &str) -> Filters<'_> {
        Filters::new(Box::new(self.inner.into_iter())).contains_str(s)
    }

    pub fn not_contains_str(self, s: &str) -> Filters<'_> {
        Filters::new(Box::new(self.inner.into_iter())).not_contains_str(s)
    }

    pub fn contains_chars(self, chars: &str) -> Filters<'_> {
        Filters::new(Box::new(self.inner.into_iter())).contains_chars(chars)
    }
    pub fn not_contains_chars(self, chars: &str) -> Filters<'_> {
        Filters::new(Box::new(self.inner.into_iter())).not_contains_chars(chars)
    }

    pub fn positional_contains_chars(self, chars: &[Option<char>]) -> Filters<'_> {
        Filters::new(Box::new(self.inner.into_iter())).positional_contains_chars(chars)
    }

    pub fn positional_not_contains_chars(self, chars: &[Option<char>]) -> Filters<'_> {
        Filters::new(Box::new(self.inner.into_iter())).positional_not_contains_chars(chars)
    }
}
