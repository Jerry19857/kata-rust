fn digitize(n: u64) -> Vec<u8> {
    let n_list = n.to_string().chars().rev().map(|c| c.to_digit(10).unwrap() as u8).collect();
    n_list
}

fn main() {
    println!("Hello, digitize!");
    println!("{:?}", digitize(348597));
}