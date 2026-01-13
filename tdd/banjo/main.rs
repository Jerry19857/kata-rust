fn main() {
    println!("Hello, Banjo!");
    let result = are_you_playing_banjo("Martin");
    println!("{}", result);
    println!("{}", are_you_playing_banjo("Rikke"));
    println!("{}", are_you_playing_banjo("bravo"));
    println!("{}", are_you_playing_banjo("rolf"));
}

fn are_you_playing_banjo(name: &str) -> String {
    let first_chars = name.chars().next().unwrap();
    if first_chars == 'R' || first_chars == 'r' {
        format!("{} plays banjo", name)
    } else {
        format!("{} does not play banjo", name)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_you_playing_banjo() {
        assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
        assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
        assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
    }
}