fn main() {
    let age = "25 years old";
    let age_value = get_age(age);
    println!("The age is: {}", age_value);
}

fn get_age(age: &str) -> u32 {
    age.chars().next().unwrap().to_digit(10).unwrap()
}