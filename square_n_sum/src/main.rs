fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|x| x.pow(2)).sum()
}

fn main() {
    let _vec: Vec<i32> = vec![1, 2, 3];
    for x in _vec.iter() {
        print!("{}^2 ", x);
    }
    print!("= {}\n", square_sum(_vec));
}

#[test]
fn returns_expected() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
    assert_eq!(square_sum(vec![]), 0);
}
