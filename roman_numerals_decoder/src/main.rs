use core::panic;
use std::char;

fn roman_to_decimal(roman: char) -> u64 {
    match roman {
        'I' => return 1,
        'V' => return 5,
        'X' => return 10,
        'L' => return 50,
        'C' => return 100,
        'D' => return 500,
        'M' => return 1000,
        _ => panic!("Invalid roman numeral"),
    }
}

fn roman_as_num(roman: &str) -> u64 {
    let chars: Vec<char> = roman.chars().collect();
    let mut result: u64 = 0;

    if chars.len() == 1 {
        return roman_to_decimal(chars[0]);
    }

    let mut i = 0;
    while i < chars.len() {
        let curr = roman_to_decimal(chars[i]);
        if i + 1 < chars.len() {
            let next = roman_to_decimal(chars[i + 1]);
            if next > curr {
                result += next - curr;
                i += 2;
                continue;
            }
        }
        result += curr;
        i += 1;
    }

    // for (curr_ch, next_ch) in chars.iter().zip(chars.iter().skip(1)) {
    //     let curr_val: u64 = roman_to_decimal(*curr_ch);
    //     let next_val: u64 = roman_to_decimal(*next_ch);

    //     if next_val > curr_val {
    //         result += next_val - curr_val;
    //     } else {
    //      result += curr_val;
    //     }
    // }
    result
}

fn main() {
    println!("CDXLIV = {}", roman_as_num("CDXLIV"));
}

#[cfg(test)]
mod roman_as_num_tests {
    use super::roman_as_num;

    fn test_equal(input: &str, actual: u64, expected: u64) {
        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right) for the input \"{}\"",
            input
        );
    }

    #[test]
    fn basic() {
        test_equal("", roman_as_num(""), 0);
        test_equal("I", roman_as_num("I"), 1);
        test_equal("XXI", roman_as_num("XXI"), 21);
        test_equal("MMVIII", roman_as_num("MMVIII"), 2008);
        test_equal("MDCLXVI", roman_as_num("MDCLXVI"), 1666);
    }

    #[test]
    fn one_through_ten() {
        test_equal("I", roman_as_num("I"), 1);
        test_equal("II", roman_as_num("II"), 2);
        test_equal("III", roman_as_num("III"), 3);
        test_equal("IV", roman_as_num("IV"), 4);
        test_equal("V", roman_as_num("V"), 5);
        test_equal("VI", roman_as_num("VI"), 6);
        test_equal("VII", roman_as_num("VII"), 7);
        test_equal("VIII", roman_as_num("VIII"), 8);
        test_equal("IX", roman_as_num("IX"), 9);
        test_equal("X", roman_as_num("X"), 10);
    }

    #[test]
    fn big_numbers() {
        test_equal("C", roman_as_num("C"), 100);
        test_equal("CDXLIV", roman_as_num("CDXLIV"), 444);
        test_equal("M", roman_as_num("M"), 1000);
        test_equal("MCMLIV", roman_as_num("MCMLIV"), 1954);
        test_equal("MCMXC", roman_as_num("MCMXC"), 1990);
        test_equal("MM", roman_as_num("MM"), 2000);
        test_equal("MMVIII", roman_as_num("MMVIII"), 2008);
        test_equal("MMM", roman_as_num("MMM"), 3000);
        test_equal("MMMCM", roman_as_num("MMMCM"), 3900);
        test_equal("MMMCMXIV", roman_as_num("MMMCMXIV"), 3914);
    }
}
