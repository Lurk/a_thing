pub mod dict;
pub mod dict_filters;
mod utils;

#[cfg(test)]
mod tests {
    use crate::dict::Dict;
    #[test]
    fn it_works() -> () {
        let d = Dict::from_file("./data/words_alpha.txt");
        let words = d
            .filter_by_length(5)
            .starts_with("")
            .ends_with("")
            .contains("")
            .not_contains("")
            .apply()
            .most_common(10);

        println!("{:#?}", words)
    }
}
