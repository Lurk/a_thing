use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Types {
    ALL,
    ALPHA,
}

struct Dict {
    iner: Vec<String>,
}

struct DictIter<'a> {
    inner: Box<dyn Iterator<Item = String> + 'a>,
}

impl<'a> DictIter<'a> {
    pub fn starts_with(mut self, s: &'a str) -> Self {
        self.inner = Box::new(self.inner.filter(move |word| word.starts_with(s)));
        self
    }

    pub fn ends_with(mut self, s: &'a str) -> Self {
        self.inner = Box::new(self.inner.filter(move |word| word.ends_with(s)));
        self
    }

    pub fn contains(mut self, chars: &'a str) -> Self {
        self.inner = Box::new(
            self.inner
                .filter(move |word| chars.chars().all(|char| word.contains(char))),
        );
        self
    }

    pub fn not_contains(mut self, chars: &'a str) -> Self {
        self.inner = Box::new(
            self.inner
                .filter(move |word| chars.chars().all(|char| !word.contains(char))),
        );
        self
    }

    pub fn apply(self) -> Dict {
        Dict {
            iner: self.inner.collect(),
        }
    }
}

impl Dict {
    pub fn new(dict_type: Types) -> Self {
        let path = match dict_type {
            Types::ALL => "./data/words.txt",
            Types::ALPHA => "./data/words_alpha.txt",
        };
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

    pub fn filter_by_length(self, len: usize) -> DictIter<'static> {
        DictIter {
            inner: Box::new(
                self.iner
                    .into_iter()
                    .filter(move |word| word.chars().count() == len),
            ),
        }
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

    pub fn starts_with<'a>(&self, s: &'a str) -> DictIter<'a> {
        DictIter {
            inner: Box::new(
                self.iner
                    .clone()
                    .into_iter()
                    .filter(move |word| word.starts_with(s)),
            ),
        }
    }

    pub fn ends_with<'a>(&self, s: &'a str) -> DictIter<'a> {
        DictIter {
            inner: Box::new(
                self.iner
                    .clone()
                    .into_iter()
                    .filter(move |word| word.ends_with(s)),
            ),
        }
    }

    pub fn contains<'a>(&self, chars: &'a str) -> DictIter<'a> {
        DictIter {
            inner: Box::new(
                self.iner
                    .clone()
                    .into_iter()
                    .filter(move |word| chars.chars().all(|char| word.contains(char))),
            ),
        }
    }
    pub fn not_contains<'a>(&self, chars: &'a str) -> DictIter<'a> {
        DictIter {
            inner: Box::new(
                self.iner
                    .clone()
                    .into_iter()
                    .filter(move |word| chars.chars().all(|char| !word.contains(char))),
            ),
        }
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
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::{Dict, Types};
    #[test]
    fn it_works() -> () {
        let d = Dict::new(Types::ALPHA);
        let words = d
            .filter_by_length(5)
            .starts_with("")
            .ends_with("")
            .contains("")
            .not_contains("")
            .apply()
            .most_common(10);

        println!("{:#?}", words.iner)
    }
}
