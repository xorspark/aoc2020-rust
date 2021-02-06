use std::collections::HashSet;
use std::io::{self, Read};

// cat inputfile | aoc01
fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", do_part_1(&input));
    println!("Part 1: {}", do_part_2(&input));
    Ok(())
}

fn find_2sum(input_set: HashSet<i32>, sum: i32) -> (i32, i32) {
    for &set_val in input_set.iter() {
        let target_val = sum - set_val;
        if input_set.contains(&target_val) {
            return (set_val, target_val);
        }
    }
    return (0, 0);
}

fn find_3sum(input_array: &Vec<i32>, sum: i32) -> (i32, i32, i32) {
    let input_set: HashSet<i32> = input_array.iter().by_ref().cloned().collect();
    for (pos, &i) in input_array.iter().enumerate() {
        for &j in input_array.iter().skip(pos + 1) {
            let target_val = sum - i - j;
            if input_set.contains(&target_val) {
                return (i, j, target_val);
            }
        }
    }
    return (0, 0, 0);
}

fn do_part_1(input: &str) -> i32 {
    let input_set: HashSet<i32> = input
        .split("\n")
        .filter_map(|w| w.trim().parse().ok())
        .collect();
    let (lhs, rhs) = find_2sum(input_set, 2020);
    return lhs * rhs;
}

fn do_part_2(input: &str) -> i32 {
    let input_array: Vec<i32> = input
        .split("\n")
        .filter_map(|w| w.trim().parse().ok())
        .collect();
    let (val1, val2, val3) = find_3sum(&input_array, 2020);
    return val1 * val2 * val3;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample_input() {
        let result = do_part_1("1721\n979\n366\n299\n675\n1456");
        assert_eq!(514579, result);
    }

    #[test]
    fn test_part_2_sample_input() {
        let result = do_part_2("1721\n979\n366\n299\n675\n1456");
        assert_eq!(241861950, result);
    }
}
