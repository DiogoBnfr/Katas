fn fix_string_case(s: &str) -> String {
    if s.chars().filter(|c| c.is_uppercase()).count() > s.len() / 2 {
        return s.to_uppercase()
    }
    s.to_lowercase()
}

fn main() {
        println!("{} expected: {}", fix_string_case("code"), "code");
        println!("{} expected: {}", fix_string_case("CODe"), "CODE");
        println!("{} expected: {}", fix_string_case("COde"), "code");
        println!("{} expected: {}", fix_string_case("Code"), "code");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(fix_string_case("code"), "code");
        assert_eq!(fix_string_case("CODe"), "CODE");
        assert_eq!(fix_string_case("COde"), "code");
        assert_eq!(fix_string_case("Code"), "code");
    }
}
