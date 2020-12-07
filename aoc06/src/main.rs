use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!(
        "Part 1 total yes responses: {}",
        count_any_yes_responses(&input)
    );
    println!(
        "Part 2 total yes responses: {}",
        count_universal_yes_responses(&input)
    );
    Ok(())
}

fn count_any_yes_responses(survey_responses: &str) -> i32 {
    let mut total_responses: i32 = 0;

    for group_responses in survey_responses.split("\n\n") {
        let mut yes_answers: HashSet<char> = HashSet::new();
        for person_responses in group_responses.lines() {
            for response in person_responses.chars() {
                yes_answers.insert(response);
            }
        }
        total_responses += yes_answers.len() as i32;
    }
    total_responses
}

fn count_universal_yes_responses(survey_responses: &str) -> i32 {
    let mut total_responses: i32 = 0;

    for group_responses in survey_responses.split("\n\n") {
        let mut yes_answers: HashMap<char, i32> = HashMap::new();
        let mut num_people: i32 = 0;
        for person_responses in group_responses.lines() {
            num_people += 1;
            for response in person_responses.chars() {
                let counter = yes_answers.entry(response).or_insert(0);
                *counter += 1;
            }
        }
        total_responses += yes_answers
            .into_iter()
            .filter(|(_k, v)| *v == num_people)
            .map(|(_k, v)| v)
            .collect::<Vec<i32>>()
            .len() as i32;
    }
    total_responses
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn can_count_any_yes_responses() {
        assert_eq!(
            6,
            count_any_yes_responses(indoc! {"
                abcx
                abcy
                abcz
            "})
        );
        assert_eq!(
            11,
            count_any_yes_responses(indoc! {"
                abc

                a
                b
                c

                ab
                ac

                a
                a
                a
                a

                b
            "})
        );
    }

    #[test]
    fn can_count_universal_yes_responses() {
        assert_eq!(
            3,
            count_universal_yes_responses(indoc! {"
                abcx
                abcy
                abcz
            "})
        );
        assert_eq!(
            6,
            count_universal_yes_responses(indoc! {"
                abc

                a
                b
                c

                ab
                ac

                a
                a
                a
                a

                b
            "})
        );
    }
}
