fn minimum(arr: &[i32]) -> i32 {
    // let mut minimum: i32 = arr[0];
    // for n in arr.iter() {
    //     if n < &minimum {
    //         minimum = *n
    //     }
    // }
    // minimum
    // refactor to this
    *arr.iter().min().unwrap()
}

fn maximum(arr: &[i32]) -> i32 {
    // let mut maximum: i32 = arr[0];
    // for n in arr.iter() {
    //     if n > &maximum {
    //         maximum = *n
    //     }   
    // }
    // maximum
    // refactor to this 
    *arr.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{minimum, maximum};
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(arr: &[i32], expected_min: i32, expected_max: i32) {
        assert_eq!(minimum(arr), expected_min, "{ERR_MSG} with function minimum and arr = {arr:?}");
        assert_eq!(maximum(arr), expected_max, "{ERR_MSG} with function maximum and arr = {arr:?}");
    }

    #[test]
    fn fixed_tests() {
        dotest(&[-52, 56, 30, 29, -54, 0, -110], -110, 56);
        dotest(&[42, 54, 65, 87, 0], 0, 87);
        dotest(&[1, 2, 3, 4, 5, 10], 1, 10);
        dotest(&[-1, -2, -3, -4, -5, -10, 534, 43, 2, 1, 3, 4, 5, 5, 443, 443, 555, 555], -10, 555);
        dotest(&[9], 9, 9);
        dotest(&[4,6,2,1,9,63,-134,566], -134, 566);
    }
}

fn main() {
    let result = minimum(&[-52, 56, 30, 29, -54, 0, -110]);
    println!("result >>>>>>>>>>>>>>>>>>>>>: {}", result)
}