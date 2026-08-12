pub fn reverse(input: &str) -> String {
    
    let char_vec: Vec<char> = input.chars().collect();
    let mut reversed_str = String::new();

    for letter in char_vec {
        reversed_str.push(letter);
    }
    
    reversed_str
}
