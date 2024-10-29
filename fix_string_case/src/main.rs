fn solve(s: &str) -> String {
    let mut lcase = 0;
    let mut ucase = 0;

    for c in s.chars() {
        if c.is_ascii_lowercase() {
            lcase += 1;
        }
        if c.is_ascii_uppercase() {
            ucase += 1;
        }
    }

    if lcase >= ucase {
        s.to_lowercase()
    } else {
        s.to_uppercase()
    }
}

fn main() {
    println!("{}", solve("HELLO world"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve("code"), "code");
        assert_eq!(solve("CODe"), "CODE");
        assert_eq!(solve("COde"), "code");
        assert_eq!(solve("Code"), "code");
    }
}
