pub mod dict;
pub mod dict_filters;
mod utils;

#[cfg(test)]
mod tests {
    use crate::dict::Dict;
    #[test]
    fn it_works() -> () {
        let d = Dict::from_file("./data/words_alpha.txt");
        let words = d.filter_by_length(5).apply();
        let freq = words.get_char_freq();
        let words = words
            .contains_chars("")
            .not_contains_chars("")
            .positional_contains_chars(&[None, None, None, None, None])
            .positional_not_contains_chars(&[None, None, None, None, None])
            .apply()
            .most_common(&freq, 10);

        println!("{:#?}", words)
    }
}
