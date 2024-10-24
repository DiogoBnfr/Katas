fn century(year: u32) -> u32 {
    let mut c: u32 = (year as f32 / 100 as f32 + 1 as f32).floor() as u32;
    if year % 10 == 0 {
        c -= 1;
    }
    c
    // Could have done just that:
    // 1 + (year - 1) / 100
}

fn main() {
    let year: u32 = 1705;
    println!("{}", year % 10);
    println!("{}", century(year));
}

#[test]
fn basic_tests() {
    assert_eq!(century(1705), 18);
    assert_eq!(century(1900), 19);
    assert_eq!(century(1601), 17);
    assert_eq!(century(2000), 20);
    assert_eq!(century(89), 1);
}
