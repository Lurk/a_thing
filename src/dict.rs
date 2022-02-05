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

/// Counts of every char from the dictionary
///
/// Basic usage:
/// ```
/// use a_thing::dict::{get_char_weights, WeightsType};
/// let dict: [String;1] = [
///     "foo".to_string(),
/// ];
/// if let WeightsType::CharWeights(weights) = get_char_weights(&dict){
///     assert_eq!(weights.get(&'f'), Some(&1));
///     assert_eq!(weights.get(&'o'), Some(&2));
/// }
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

/// Counts every char in dictionary with respect to char position
/// Basic usage:
/// ```
/// use a_thing::dict::{get_char_position_weights, WeightsType};
/// let dict: [String;1] = [
///     "foo".to_string(),
/// ];
/// if let WeightsType::CharPositionWeights(weights) = get_char_position_weights(&dict){
///     assert_eq!(weights.get(&'f').unwrap()[0], 1);
///     assert_eq!(weights.get(&'o').unwrap()[1], 1);
///     assert_eq!(weights.get(&'o').unwrap()[2], 1);
/// }
/// ```
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

    use crate::dict::{get_char_position_weights, get_char_weights, WeightsType};

    fn test_dict() -> [String; 3] {
        ["foo".to_string(), "bar".to_string(), "baz".to_string()]
    }
    #[test]
    fn get_char_weights_test() {
        if let WeightsType::CharWeights(weights) = get_char_weights(&test_dict()) {
            assert_eq!(weights.get(&'a'), Some(&2));
            assert_eq!(weights.get(&'b'), Some(&2));
            assert_eq!(weights.get(&'f'), Some(&1));
            assert_eq!(weights.get(&'r'), Some(&1));
            assert_eq!(weights.get(&'o'), Some(&2));
            assert_eq!(weights.get(&'z'), Some(&1));
        }
    }

    #[test]
    fn get_char_position_weights_test() {
        if let WeightsType::CharPositionWeights(weights) = get_char_position_weights(&test_dict()) {
            assert_eq!(weights.get(&'f').unwrap()[0], 1);
            assert_eq!(weights.get(&'b').unwrap()[0], 2);
            assert_eq!(weights.get(&'o').unwrap()[1], 1);
            assert_eq!(weights.get(&'o').unwrap()[2], 1);
            assert_eq!(weights.get(&'a').unwrap()[1], 2);
            assert_eq!(weights.get(&'r').unwrap()[2], 1);
            assert_eq!(weights.get(&'z').unwrap()[2], 1);
        }
    }
}
