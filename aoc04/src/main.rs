use regex::Captures;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let passports = parse_passport_batch_file(&input);
    println!(
        "Part 1 valid passports: {}",
        count_valid_passports_part_1(&passports)
    );
    println!(
        "Part 2 valid passports: {}",
        count_valid_passports_part_2(&passports)
    );

    Ok(())
}

fn parse_passport_batch_file(input: &str) -> Vec<HashMap<String, String>> {
    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let re: Regex = Regex::new(r"(?P<field>[^:]*):(?P<value>[^ |\n]*)").unwrap();

    for line in input.split("\n\n") {
        let mut passport_entry: HashMap<String, String> = HashMap::new();
        for caps in re.captures_iter(&line) {
            passport_entry.insert(
                caps["field"].trim().to_string(),
                caps["value"].trim().to_string(),
            );
        }
        passports.push(passport_entry);
    }

    passports
}

fn count_valid_passports_part_1(passports: &Vec<HashMap<String, String>>) -> i32 {
    passports.iter().fold(0, |mut acc, passport| {
        if has_passport_fields(&passport) {
            acc += 1;
        }
        acc
    })
}

fn count_valid_passports_part_2(passports: &Vec<HashMap<String, String>>) -> i32 {
    passports.iter().fold(0, |mut acc, passport| {
        if has_valid_passport(&passport) {
            acc += 1;
        }
        acc
    })
}

fn has_passport_fields(passport: &HashMap<String, String>) -> bool {
    passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid")
}

fn has_valid_passport(passport: &HashMap<String, String>) -> bool {
    has_passport_fields(&passport)
        && has_valid_birth_year(passport.get("byr").unwrap().to_string())
        && has_valid_issue_year(passport.get("iyr").unwrap().to_string())
        && has_valid_expiration_year(passport.get("eyr").unwrap().to_string())
        && has_valid_height(passport.get("hgt").unwrap().to_string())
        && has_valid_eye_color(passport.get("ecl").unwrap().to_string())
        && has_valid_hair_color(passport.get("hcl").unwrap().to_string())
        && has_valid_passport_id(passport.get("pid").unwrap().to_string())
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
fn has_valid_birth_year(birth_year: String) -> bool {
    (1920..=2002).contains(&birth_year.parse::<i32>().unwrap())
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
fn has_valid_issue_year(issue_year: String) -> bool {
    (2010..=2020).contains(&issue_year.parse::<i32>().unwrap())
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn has_valid_expiration_year(expiration_year: String) -> bool {
    (2020..=2030).contains(&expiration_year.parse::<i32>().unwrap())
}

/*
hgt (Height) - a number followed by either cm or in:

    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
*/
fn has_valid_height(height: String) -> bool {
    let re: Regex = Regex::new(r"(\d+)((in|cm)?)").unwrap();
    let c: Captures = re.captures(&height).unwrap();
    let height_value: i32 = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let unit: &str = c.get(2).unwrap().as_str();

    if unit == "cm" {
        (59..=76).contains(&height_value)
    } else if unit == "in" {
        (150..=193).contains(&height_value)
    } else {
        false
    }
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn has_valid_eye_color(eye_color: String) -> bool {
    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .any(|&eye| eye == eye_color)
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn has_valid_hair_color(hair_color: String) -> bool {
    Regex::new(r"#[0-9a-f]{6}").unwrap().is_match(&hair_color)
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn has_valid_passport_id(passport_id: String) -> bool {
    passport_id.chars().count() == 9
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_batch_file() {
        let input = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let passport_list = parse_passport_batch_file(&input);
        assert_eq!(4, passport_list.len());

        let passport = &passport_list[2];

        assert_eq!("2024", passport.get("eyr").unwrap());
        assert_eq!(None, passport.get("cid"));
    }

    #[test]
    fn can_check_passport_fields() {
        let input = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm";
        let passport_list = parse_passport_batch_file(&input);
        assert_eq!(true, has_passport_fields(&passport_list[0]));
        assert_eq!(false, has_passport_fields(&passport_list[1]));
        assert_eq!(true, has_passport_fields(&passport_list[2]));
    }

    #[test]
    fn can_validate_birth_year() {
        let valid_birth_year = &parse_passport_batch_file("byr:1937")[0];
        assert_eq!(
            true,
            has_valid_birth_year(valid_birth_year.get("byr").unwrap().to_string())
        );
        let invalid_birth_year = &parse_passport_batch_file("byr:1900")[0];
        assert_eq!(
            false,
            has_valid_birth_year(invalid_birth_year.get("byr").unwrap().to_string())
        );
    }

    #[test]
    fn can_validate_issue_year() {
        let valid_issue_year = &parse_passport_batch_file("iyr:2015")[0];
        assert_eq!(
            true,
            has_valid_issue_year(valid_issue_year.get("iyr").unwrap().to_string())
        );
        let invalid_issue_year = &parse_passport_batch_file("iyr:2001")[0];
        assert_eq!(
            false,
            has_valid_issue_year(invalid_issue_year.get("iyr").unwrap().to_string())
        );
    }

    #[test]
    fn can_validate_expiration_year() {
        let valid_expiration_year = &parse_passport_batch_file("eyr:2022")[0];
        assert_eq!(
            true,
            has_valid_expiration_year(valid_expiration_year.get("eyr").unwrap().to_string())
        );
        let invalid_expiration_year = &parse_passport_batch_file("eyr:2049")[0];
        assert_eq!(
            false,
            has_valid_expiration_year(invalid_expiration_year.get("eyr").unwrap().to_string())
        );
    }

    #[test]
    fn can_validate_valid_height() {
        let valid_height_inches = &parse_passport_batch_file("hgt:159in")[0];
        assert_eq!(
            true,
            has_valid_height(valid_height_inches.get("hgt").unwrap().to_string())
        );
        let invalid_height_inches = &parse_passport_batch_file("hgt:100in")[0];
        assert_eq!(
            false,
            has_valid_height(invalid_height_inches.get("hgt").unwrap().to_string())
        );
        let valid_height_centimeters = &parse_passport_batch_file("hgt:65cm")[0];
        assert_eq!(
            true,
            has_valid_height(valid_height_centimeters.get("hgt").unwrap().to_string())
        );
        let invalid_height_centimeters = &parse_passport_batch_file("hgt:80cm")[0];
        assert_eq!(
            false,
            has_valid_height(invalid_height_centimeters.get("hgt").unwrap().to_string())
        );
        let invalid_height_no_units = &parse_passport_batch_file("hgt:999")[0];
        assert_eq!(
            false,
            has_valid_height(invalid_height_no_units.get("hgt").unwrap().to_string())
        );
    }

    #[test]
    fn can_validate_eye_color() {
        let valid_eye_color = &parse_passport_batch_file("ecl:brn")[0];
        assert_eq!(
            true,
            has_valid_eye_color(valid_eye_color.get("ecl").unwrap().to_string())
        );
        let invalid_eye_color = &parse_passport_batch_file("ecl:red")[0];
        assert_eq!(
            false,
            has_valid_eye_color(invalid_eye_color.get("ecl").unwrap().to_string())
        );
    }

    #[test]
    fn can_validate_hair_color() {
        let valid_hair_color = &parse_passport_batch_file("hcl:#112233")[0];
        assert_eq!(
            true,
            has_valid_hair_color(valid_hair_color.get("hcl").unwrap().to_string())
        );
        let invalid_hair_color = &parse_passport_batch_file("hcl:rgb(50,100,150)")[0];
        assert_eq!(
            false,
            has_valid_hair_color(invalid_hair_color.get("hcl").unwrap().to_string())
        );
    }

    #[test]
    fn can_validate_passport_id() {
        let valid_passport_id = &parse_passport_batch_file("pid:012345678")[0];
        assert_eq!(
            true,
            has_valid_passport_id(valid_passport_id.get("pid").unwrap().to_string())
        );
        let invalid_passport_id = &parse_passport_batch_file("pid:393939")[0];
        assert_eq!(
            false,
            has_valid_passport_id(invalid_passport_id.get("pid").unwrap().to_string())
        );
    }
}
