pub mod dict;
pub mod dict_filters;
mod utils;

#[cfg(test)]
mod tests {
    use crate::dict::Dict;
    #[test]
    fn it_works() -> () {
        if let Ok(d) = Dict::from_file_with_word_len("./data/words_alpha.txt", 5) {
            let freq = d.get_char_freq();
            let words = d
                .contains_chars("")
                .not_contains_chars("")
                .positional_contains_chars(&[None, None, None, None, None])
                .positional_not_contains_chars(&[None, None, None, None, None])
                .apply()
                .most_common(&freq, 10);

            println!("{:#?}", words)
        }
    }
}
