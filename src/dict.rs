use std::collections::HashMap;

/// Structure for holding char weights
pub type CharWeights = HashMap<char, usize>;

/// Structure for holding char weights with positions
///
/// Here we are assuming that no word is longer than 64 chars
/// -   eng:    Pneumonoultramicroscopicsilicovolcanoconiosis
/// -   de:     Rindfleischetikettierungsüberwachungsaufgabenübertragungsgesetz
///
/// if this is a problem for you, please fill the issue
pub type CharPositionWeights = HashMap<char, [usize; 64]>;

#[derive(Debug)]
pub enum WeghtsType {
    CharWeights(CharWeights),
    CharPositionWeights(CharPositionWeights),
}

pub fn get_char_position_weights(dict: &[String]) -> WeghtsType {
    let mut freq: CharPositionWeights = HashMap::new();
    for w in dict.iter() {
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

pub fn get_char_weights(dict: &[String]) -> WeghtsType {
    let mut freq: CharWeights = HashMap::new();
    for w in dict.iter() {
        for char in w.chars() {
            let count = freq.entry(char).or_insert(0);
            *count += 1;
        }
    }
    WeghtsType::CharWeights(freq)
}

pub fn most_common(dict: &[String], freq: &WeghtsType, count: usize) -> Vec<String> {
    let words_with_weight: HashMap<&String, usize> = match freq {
        WeghtsType::CharWeights(f) => most_common_by_char(dict, f),
        WeghtsType::CharPositionWeights(f) => most_common_by_char_position(dict, f),
    };

    let mut v: Vec<_> = words_with_weight.iter().collect();
    v.sort_by(|a, b| b.1.cmp(a.1));
    let n = if v.len() >= count { count } else { v.len() };
    v[0..n].iter().map(|(word, _)| word.to_string()).collect()
}

fn most_common_by_char<'a>(
    dict: &'a [String],
    freq: &'a CharWeights,
) -> HashMap<&'a String, usize> {
    let mut words_with_weight = HashMap::new();
    for word in dict {
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

fn most_common_by_char_position<'a>(
    dict: &'a [String],
    freq: &CharPositionWeights,
) -> HashMap<&'a String, usize> {
    let mut words_with_weight = HashMap::new();
    for word in dict {
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
