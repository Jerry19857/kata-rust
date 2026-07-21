// HackerRank - Mini-Max Sum (Easy)
// Given 5 integers, find the min and max sum possible from 4 of the 5 elements.
fn mini_max_sum(arr: &[i32; 5]) -> (i64, i64) {
    let max = *arr.iter().max().unwrap() as i64;
    let min = *arr.iter().min().unwrap() as i64;
    let sum: i64 = arr.iter().map(|&x| x as i64).sum();
    (sum - max, sum - min)
}

#[cfg(test)]
mod tests {
    use super::mini_max_sum;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(arr: [i32; 5], expected: (i64, i64)) {
        assert_eq!(mini_max_sum(&arr), expected, "{ERR_MSG} with arr = {arr:?}");
    }

    #[test]
    fn fixed_tests() {
        dotest([1, 3, 5, 7, 9], (16, 24));
        dotest([1, 2, 3, 4, 5], (10, 14));
        dotest([256741038, 623958417, 467905213, 714532089, 938071625], (2063136757, 2744467344));
    }
}

fn main() {
    let (min, max) = mini_max_sum(&[1, 3, 5, 7, 9]);
    println!("{} {}", min, max);
}
