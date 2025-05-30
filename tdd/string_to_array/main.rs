fn string_to_array(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(|word| word.to_string())
        .collect::<Vec<String>>()
}


fn main() {
    let s = "hello world";
    let result = string_to_array(s);
    println!("{:?}", result);
}