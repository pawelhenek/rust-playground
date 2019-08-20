fn words(phrase: &str) -> std::str::SplitWhitespace {
    return phrase.split_whitespace()
}

pub fn initials(phrase: &str) -> String {
    words(phrase).map(
        |word| word.chars().next().unwrap()
    ).collect::<String>().to_uppercase()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}