fn past(h: i32, m: i32, s: i32) -> i32 {
    ((h * 3600) * 1000) + ((m * 60) * 1000) + (s * 1000)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(past(0, 1, 1), 61000);
        assert_eq!(past(1, 1, 1), 3661000);
        assert_eq!(past(0, 0, 0), 0);
        assert_eq!(past(1, 0, 1), 3601000);
        assert_eq!(past(1, 0, 0), 3600000);
    }
}
