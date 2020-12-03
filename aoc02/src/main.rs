use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    validate_passwords(&input);
    Ok(())
}

fn validate_passwords(input: &str) {
    let mut validator_v1: PasswordValidatorV1 = PasswordValidatorV1::new();
    let mut validator_v2: PasswordValidatorV2 = PasswordValidatorV2::new();
    let mut total_v1: i32 = 0;
    let mut total_v2: i32 = 0;
    for line in input.lines() {
        validator_v1.parse_policy_and_pw(line.to_string());
        validator_v2.parse_policy_and_pw(line.to_string());
        if validator_v1.is_password_valid() {
            total_v1 += 1;
        }
        if validator_v2.is_password_valid() {
            total_v2 += 1;
        }
    }
    println!("Part 1 valid passwords: {}", total_v1);
    println!("Part 2 valid passwords: {}", total_v2);
}

pub struct PasswordValidatorV1 {
    min_length: usize,
    max_length: usize,
    required_char: char,
    password: String,
}

impl PasswordValidatorV1 {
    pub fn new() -> PasswordValidatorV1 {
        PasswordValidatorV1 {
            min_length: 0,
            max_length: 0,
            required_char: ' ',
            password: String::new(),
        }
    }

    pub fn parse_policy_and_pw(&mut self, policy_and_pw: String) {
        let arr: Vec<&str> = policy_and_pw
            .split(|c| c == '-' || c == ' ' || c == ':')
            .map(|w| w.trim())
            .filter(|&w| w != "")
            .collect();

        self.min_length = arr[0].parse::<usize>().unwrap();
        self.max_length = arr[1].parse::<usize>().unwrap();
        self.required_char = arr[2].chars().next().unwrap();
        self.password = arr[3].to_string();
    }

    pub fn is_password_valid(&mut self) -> bool {
        (self.min_length..=self.max_length)
            .contains(&self.password.matches(self.required_char).count())
    }
}

pub struct PasswordValidatorV2 {
    first_test_index: usize,
    second_test_index: usize,
    target_char: char,
    password: String,
}

impl PasswordValidatorV2 {
    pub fn new() -> PasswordValidatorV2 {
        PasswordValidatorV2 {
            first_test_index: 0,
            second_test_index: 0,
            target_char: ' ',
            password: String::new(),
        }
    }

    pub fn parse_policy_and_pw(&mut self, policy_and_pw: String) {
        let arr: Vec<&str> = policy_and_pw
            .split(|c| c == '-' || c == ' ' || c == ':')
            .map(|w| w.trim())
            .filter(|&w| w != "")
            .collect();

        self.first_test_index = arr[0].parse::<usize>().unwrap() - 1;
        self.second_test_index = arr[1].parse::<usize>().unwrap() - 1;
        self.target_char = arr[2].chars().next().unwrap();
        self.password = arr[3].to_string();
    }

    pub fn is_password_valid(&mut self) -> bool {
        let password_chars: Vec<char> = self.password.chars().collect();

        (password_chars[self.first_test_index] == self.target_char
            && password_chars[self.second_test_index] != self.target_char)
            || (password_chars[self.first_test_index] != self.target_char
                && password_chars[self.second_test_index] == self.target_char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_policy_and_pw_part_1() {
        let mut validator: PasswordValidatorV1 = PasswordValidatorV1::new();
        validator.parse_policy_and_pw("1-3 a: aabbcc".to_string());

        assert_eq!(1, validator.min_length);
        assert_eq!(3, validator.max_length);
        assert_eq!('a', validator.required_char);
        assert_eq!("aabbcc", validator.password);
    }

    #[test]
    fn can_validate_password_part_1() {
        let mut validator: PasswordValidatorV1 = PasswordValidatorV1::new();
        validator.parse_policy_and_pw("1-3 a: aabbcc".to_string());
        assert_eq!(true, validator.is_password_valid());
        validator.parse_policy_and_pw("1-3 b: cdefg".to_string());
        assert_eq!(false, validator.is_password_valid());
        validator.parse_policy_and_pw("2-9 c: ccccccccc".to_string());
        assert_eq!(true, validator.is_password_valid());
    }

    #[test]
    fn can_parse_policy_and_pw_part_2() {
        let mut validator: PasswordValidatorV2 = PasswordValidatorV2::new();
        validator.parse_policy_and_pw("1-3 a: aabbcc".to_string());

        assert_eq!(0, validator.first_test_index);
        assert_eq!(2, validator.second_test_index);
        assert_eq!('a', validator.target_char);
        assert_eq!("aabbcc", validator.password);
    }

    #[test]
    fn can_validate_password_part_2() {
        let mut validator: PasswordValidatorV2 = PasswordValidatorV2::new();
        validator.parse_policy_and_pw("1-3 a: aabbcc".to_string());
        assert_eq!(true, validator.is_password_valid());
        validator.parse_policy_and_pw("1-3 b: cdefg".to_string());
        assert_eq!(false, validator.is_password_valid());
        validator.parse_policy_and_pw("2-9 c: ccccccccc".to_string());
        assert_eq!(false, validator.is_password_valid());
    }
}
