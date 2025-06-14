fn rps(p1: &str, p2: &str) -> &'static str  {
    match (p1, p2) {
        ("rock", "scissors") | ("scissors", "paper") | ("paper", "rock") => "Player 1 won!",
        ("rock", "paper") | ("paper", "scissors") | ("scissors", "rock") => "Player 2 won!",
        _ => "Draw!"
    }
}

#[cfg(test)]
mod tests {
    use super::rps;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(p1: &str, p2: &str, expected: &str) {
        assert_eq!(rps(p1, p2), expected, "{ERR_MSG} with p1 = \"{p1}\", p2 = \"{p2}\"")   
    }

    #[test]
    fn fixed_tests() {
        dotest("rock", "scissors", "Player 1 won!");
        dotest("scissors", "rock", "Player 2 won!");
        dotest("rock", "rock", "Draw!");
    }
}


fn main() {
    let result = rps("rock", "scissors");
    println!("result : {}", result);
}