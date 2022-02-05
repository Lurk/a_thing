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
pub enum WeightsType {
    CharWeights(CharWeights),
    CharPositionWeights(CharPositionWeights),
}

/// Produces HashMap with count of every char from the dictionary
///
/// Basic usage:
/// ```
/// use a_thing::dict::get_char_weights;
/// let dict: [String;1] = [
///     "foo".to_string(),
/// ];
/// let weights = get_char_weights(&dict);
/// assert_eq!(weights.get(&'f'), Some(&1));
/// assert_eq!(weights.get(&'o'), Some(&2));
/// ```
pub fn get_char_weights(dict: &[String]) -> WeightsType {
    let mut freq: CharWeights = HashMap::new();
    for w in dict.iter() {
        for char in w.chars() {
            let count = freq.entry(char).or_insert(0);
            *count += 1;
        }
    }
    WeightsType::CharWeights(freq)
}

pub fn get_char_position_weights(dict: &[String]) -> WeightsType {
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
    WeightsType::CharPositionWeights(freq)
}

pub fn most_common(dict: &[String], freq: &WeightsType, count: usize) -> Vec<String> {
    let words_with_weight: HashMap<&String, usize> = match freq {
        WeightsType::CharWeights(f) => most_common_by_char(dict, f),
        WeightsType::CharPositionWeights(f) => most_common_by_char_position(dict, f),
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

#[cfg(test)]
mod tests {

    use crate::dict::{get_char_weights, WeightsType};

    fn test_dict() -> [String; 5] {
        [
            "foo".to_string(),
            "bfoo".to_string(),
            "foobar".to_string(),
            "foobarbaz".to_string(),
            "bfoobarbaz".to_string(),
        ]
    }
    #[test]
    fn get_char_weights_test() {
        if let WeightsType::CharWeights(weights) = get_char_weights(&test_dict()) {
            assert_eq!(weights.get(&'a'), Some(&5));
            assert_eq!(weights.get(&'b'), Some(&7));
            assert_eq!(weights.get(&'f'), Some(&5));
            assert_eq!(weights.get(&'r'), Some(&3));
            assert_eq!(weights.get(&'o'), Some(&10));
            assert_eq!(weights.get(&'z'), Some(&2));
        }
    }
}
