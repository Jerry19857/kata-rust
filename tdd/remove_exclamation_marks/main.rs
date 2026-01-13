fn main() {
    let input = "Hello World!";
    let output = remove_exclamation_marks(input);
    println!("{}", output);
}

fn remove_exclamation_marks(input: &str) -> String {
    input.chars().filter(|c| *c != '!').collect()
}