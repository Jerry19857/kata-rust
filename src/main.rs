fn main() {
    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        eprintln!("Failed to read input");
        return;
    }

    let number: i64 = match input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Please provide a valid integer");
            return;
        }
    };

    if number % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }
}
