fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    }
    &s[..]
}

fn main() {
    let mut str = String::from("hello");
    let mut word = first_word(&str);
    println!("{}", word);
    str.clear();
    println!("{}", word);
}
