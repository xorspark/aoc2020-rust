use std::cmp;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Highest seat id in part 1: {}", do_part_one(&input));
    println!("Missing seat id in part 2: {}", do_part_two(&input));

    Ok(())
}

fn do_part_one(input: &str) -> i32 {
    let mut max_id = -1;

    for line in input.lines() {
        let seat_id = find_seat_id(line.trim());
        max_id = cmp::max(max_id, seat_id);
    }
    max_id
}

fn do_part_two(input: &str) -> i32 {
    let mut seats: Vec<i32> = Vec::new();
    let mut seen: i32 = 0;

    for line in input.lines() {
        seats.push(find_seat_id(line.trim()));
    }
    seats.sort();
    seats.dedup();
    for id in seats {
        if seen > 200 && (seen + 1) != id {
            return seen + 1;
        }
        seen = id;
    }
    0
}

fn find_seat_id(boarding_pass: &str) -> i32 {
    let (rows, cols): (Vec<char>, Vec<char>) =
        split_boarding_pass_into_rows_and_cols(boarding_pass);
    find_code(rows, 'F', 'B', 0, 127) * 8 + find_code(cols, 'L', 'R', 0, 7)
}

fn split_boarding_pass_into_rows_and_cols(boarding_pass: &str) -> (Vec<char>, Vec<char>) {
    let (rows, cols): (Vec<char>, Vec<char>) =
        boarding_pass.chars().partition(|&c| c != 'L' && c != 'R');

    (rows, cols)
}

fn find_code(
    code_sequence: Vec<char>,
    lower_bound_code: char,
    upper_bound_code: char,
    mut min_value: i32,
    mut max_value: i32,
) -> i32 {
    let mut last_seen: char = ' ';
    for ch in code_sequence.iter() {
        let difference = (max_value - min_value) / 2 + 1;
        if *ch == lower_bound_code {
            max_value -= difference;
        } else if *ch == upper_bound_code {
            min_value += difference;
        }
        last_seen = *ch;
    }

    if last_seen == lower_bound_code {
        min_value
    } else {
        max_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_split_boarding_pass() {
        let input = String::from("BFFFBBFRRR");
        let (rows, cols) = split_boarding_pass_into_rows_and_cols(&input);
        assert_eq!(vec!['B', 'F', 'F', 'F', 'B', 'B', 'F'], rows);
        assert_eq!(vec!['R', 'R', 'R'], cols);
    }

    #[test]
    fn can_find_code() {
        let mut input = String::from("FBFBBFFRLR");
        let (rows, cols) = split_boarding_pass_into_rows_and_cols(&input);
        assert_eq!(44, find_code(rows, 'F', 'B', 0, 127));
        assert_eq!(5, find_code(cols, 'L', 'R', 0, 7));
        input = String::from("BFFFBBFRRR");
        let (rows, cols) = split_boarding_pass_into_rows_and_cols(&input);
        assert_eq!(70, find_code(rows, 'F', 'B', 0, 127));
        assert_eq!(7, find_code(cols, 'L', 'R', 0, 7));
        input = String::from("BBFFBBFRLL");
        let (rows, cols) = split_boarding_pass_into_rows_and_cols(&input);
        assert_eq!(102, find_code(rows, 'F', 'B', 0, 127));
        assert_eq!(4, find_code(cols, 'L', 'R', 0, 7));
    }

    #[test]
    fn can_find_seat_id() {
        assert_eq!(357, find_seat_id("FBFBBFFRLR"));
        assert_eq!(567, find_seat_id("BFFFBBFRRR"));
        assert_eq!(119, find_seat_id("FFFBBBFRRR"));
        assert_eq!(820, find_seat_id("BBFFBBFRLL"));
    }
}
