pub mod dict;
pub mod dict_iter;
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
            .contains("ar")
            .not_contains("esioty")
            .apply()
            .most_common(100);

        println!("{:#?}", words)
    }
}
