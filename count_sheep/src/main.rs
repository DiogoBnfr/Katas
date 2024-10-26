fn count_sheep(n: u32) -> String {
    let mut str: String = Default::default();
    for x in 1..=n {
        str.push_str(&format!("{x} sheep..."));
    }
    str
}

// Second solution
fn count_sheep_map(n: u32) -> String {
    (1..=n).map(|x| format!("{x} sheep...")).collect()
}

fn main() {
    println!("{}", count_sheep(3));
    println!("{}", count_sheep_map(3));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn returns_expected() {
    assert_eq!(count_sheep(0), "");
    assert_eq!(count_sheep(1), "1 sheep...");
    assert_eq!(count_sheep(2), "1 sheep...2 sheep...");
    assert_eq!(count_sheep(3), "1 sheep...2 sheep...3 sheep...");
  }

  #[test]
  fn returns_expected_map() {
    assert_eq!(count_sheep_map(0), "");
    assert_eq!(count_sheep_map(1), "1 sheep...");
    assert_eq!(count_sheep_map(2), "1 sheep...2 sheep...");
    assert_eq!(count_sheep_map(3), "1 sheep...2 sheep...3 sheep...");
  }
}
