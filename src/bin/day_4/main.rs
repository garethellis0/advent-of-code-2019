#[macro_use] extern crate itertools;
use itertools::{izip, multizip};

fn convert_int_to_vec(i: i32) -> Vec<i32> {
    /// Convert the given integer into an vector of integers
    /// Example: `1234` -> `{1, 2, 3, 4}`

    let mut result = Vec::new();

    let mut f = i as f32;

    while f >= (1 as f32) {
        // Save the lowest order digit
        result.push(f.round() as i32 % 10);

        // Shift the decimal
        f /= 10.0;
        f = f.trunc();
    }

    result.reverse();

    return result;
}

#[cfg(test)]
mod convert_vec_to_int_tests {
    use crate::convert_int_to_vec;

    fn test_1() {
        assert_eq!(vec!(1, 2, 3, 4), convert_int_to_vec(1234))
    }

    fn test_2() {
        assert_eq!(vec!(9, 7, 0, 4), convert_int_to_vec(9704))
    }
}


fn digits_never_decrease(i: i32) -> bool {
    let digit_vec = convert_int_to_vec(i);

    let digit_pairs = digit_vec[1..].iter().zip(digit_vec.clone());

    digit_pairs.fold(true, |no_digits_increase_so_far, digit_pair| {
        return no_digits_increase_so_far && (digit_pair.0 >= &digit_pair.1);
    })
}

fn at_least_two_adjacent_values_eq(i: i32) -> bool {
    let mut result = false;

    let digit_vec = convert_int_to_vec(i);

    let digit_pairs = digit_vec[1..].iter().zip(digit_vec.clone());

    digit_pairs.fold(false, |seen_eq_digits, digit_pair| {
        return seen_eq_digits || (digit_pair.0 == &digit_pair.1);
    })
}

fn at_least_two_adjacent_values_not_part_of_larger_group_eq(i: i32) -> bool {
    let mut result = false;

    let digit_vec = convert_int_to_vec(i);

    let quads = digit_vec[1..].iter().zip(digit_vec.clone());

    let mut quads = izip!(
        digit_vec[..].iter(), digit_vec[1..].iter(), digit_vec[2..].iter(), digit_vec[3..].iter()
    );

    let interior_pair_eq = quads.fold(false, |seen_pair, quad| {
        return seen_pair || (quad.1 == quad.2 && quad.0 != quad.1 && quad.2 != quad.3);
    });

    let start_pair_eq = match digit_vec.get(0..3) {
        Some(slice) => slice[0] == slice[1] && slice[1] != slice[2],
        None => false,
    };

    let end_pair_eq = match digit_vec.get(digit_vec.len()-3..) {
        Some(slice) => slice[0] != slice[1] && slice[1] == slice[2],
        None => false,
    };

    return interior_pair_eq || start_pair_eq || end_pair_eq;
}

fn is_six_digit(i: i32) -> bool {
    return i > 99999 && i < 1000000;
}

fn part1(range_start: i32, range_end: i32) -> usize {
    let mut matching_numbers = Vec::new();

    for i in range_start..range_end {
        println!("{}: {}, {}, {}", i, is_six_digit(i), at_least_two_adjacent_values_not_part_of_larger_group_eq(i), digits_never_decrease(i));
        if is_six_digit(i) && at_least_two_adjacent_values_not_part_of_larger_group_eq(i) && digits_never_decrease(i) {
            matching_numbers.push(i);
        }
    }

    return matching_numbers.len();
}

fn main() {
    let range_start = 108457;
    let range_end = 562041;
    println!("Found this many matching passwords: {}", part1(range_start, range_end));
}