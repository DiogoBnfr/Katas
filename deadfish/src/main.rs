fn parse(code: &str) -> Vec<i32> {
    let mut value: i32 = 0;
    let mut vector: Vec<i32> = vec![];
    for ch in code.chars() {
        match ch {
            'i' => value += 1,
            'd' => value -= 1,
            's' => value *= value,
            'o' => vector.push(value),
            _ => continue
        }
    }
    vector
}

fn main() {
    let vector: Vec<i32> = parse("iiisdoso");
    println!("{:?}", vector);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"),
            vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"),
            vec![8, 64, 3600]);
    }
}
