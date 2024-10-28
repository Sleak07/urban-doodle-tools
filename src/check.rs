// TODO : to create a sensitive and non sensitive palindrome
//

//case sensitive palindrome
pub fn is_case_palindrome(input: &str) -> bool {
    input.chars().eq(input.chars().rev())
}

//insensitive palindrome
pub fn is_not_case_palindrome(input: &str) -> bool {
    let clean_input: String = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    clean_input.chars().eq(clean_input.chars().rev())
}
