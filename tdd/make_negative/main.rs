fn main() {
    let a = make_negative(3);
    let b = make_negative(-5);
    let c = make_negative(0);
    println!("make_negative(5) = {}", a);
    println!("make_negative(-5) = {}", b);
    println!("make_negative(0) = {}", c);
}



fn make_negative(n: i32) -> i32 {
    match n.is_positive() {
        true => -n,
        false => n,
    }
} 