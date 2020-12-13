use std::cmp;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let first_fail_value = find_first_failing(&input, 25);
    println!("Part 1 solution: {}", first_fail_value);
    println!(
        "Part 2 solution: {}",
        find_encryption_weakness(&input, first_fail_value)
    );

    Ok(())
}

fn find_first_failing(input: &str, preamble_size: usize) -> i32 {
    let input_vector: Vec<i32> = convert_to_vector(input);

    for (idx, &val) in input_vector.iter().enumerate().skip(preamble_size) {
        if !is_valid(&input_vector, preamble_size, idx) {
            return val;
        }
    }

    return -1;
}

fn find_encryption_weakness(input: &str, target_val: i32) -> i32 {
    let input_vector: Vec<i32> = convert_to_vector(input);
    for (idx, _) in input_vector.iter().enumerate() {
        let mut sum: i32 = 0;
        let mut min: i32 = i32::MAX;
        let mut max: i32 = i32::MIN;
        let _ = input_vector.iter().skip(idx).try_for_each(|&v| {
            if (sum + v) <= target_val {
                sum += v;
                min = cmp::min(v, min);
                max = cmp::max(v, max);
                return Ok(());
            }
            return Err(());
        });
        if sum == target_val {
            return min + max;
        }
    }

    return 0;
}

fn convert_to_vector(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|v| v.parse().ok())
        .collect::<Vec<i32>>()
}

fn is_valid(input_list: &Vec<i32>, preamble_size: usize, check_position: usize) -> bool {
    let preamble_list: Vec<i32> = input_list
        .iter()
        .skip(check_position - preamble_size)
        .take(preamble_size)
        .cloned()
        .collect::<Vec<i32>>();

    for &preamble_value in preamble_list.iter() {
        let needle: i32 = cmp::max(input_list[check_position], preamble_value)
            - cmp::min(input_list[check_position], preamble_value);
        if needle != preamble_value && preamble_list.contains(&needle) {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn can_find_valid_values() {
        let input = indoc! {"
            35
            20
            15
            25
            47
            40
            62
            55
            65
            95
            102
            117
            150
            182
            127
            219
            299
            277
            309
            576
        "};
        let preamble = convert_to_vector(&input);
        assert_eq!(false, is_valid(&preamble, 5, 14)); // 127
        assert_eq!(true, is_valid(&preamble, 5, 10)); // 102
        assert_eq!(true, is_valid(&preamble, 5, 19)); // 576
    }

    #[test]
    fn can_find_first_failing() {
        assert_eq!(
            100,
            find_first_failing(
                indoc! {"
                    1
                    2
                    3
                    4
                    5
                    6
                    7
                    8
                    9
                    10
                    11
                    12
                    13
                    14
                    15
                    16
                    17
                    18
                    19
                    20
                    21
                    22
                    23
                    24
                    25
                    26
                    49
                    50
                    100
                "},
                25
            )
        );
        assert_eq!(
            127,
            find_first_failing(
                indoc! {"
                    35
                    20
                    15
                    25
                    47
                    40
                    62
                    55
                    65
                    95
                    102
                    117
                    150
                    182
                    127
                    219
                    299
                    277
                    309
                    576
                "},
                5
            )
        );
    }

    #[test]
    fn can_find_encryption_weakness() {
        assert_eq!(
            62,
            find_encryption_weakness(
                indoc! {"
                    35
                    20
                    15
                    25
                    47
                    40
                    62
                    55
                    65
                    95
                    102
                    117
                    150
                    182
                    127
                    219
                    299
                    277
                    309
                    576
                "},
                127
            )
        );
    }
}
