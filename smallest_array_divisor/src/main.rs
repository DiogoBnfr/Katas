fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut divisor: i32 = 1;
    let mut result: i32;
    loop {
        result = nums.iter().map(|x| (x + divisor - 1) / divisor).sum();
        if result <= threshold {
            return divisor
        }
        divisor += 1;
    }
}

fn main() {
    let arr: Vec<i32> = vec![1, 2, 5, 9];
    assert_eq!(smallest_divisor(arr, 6), 5);
}
