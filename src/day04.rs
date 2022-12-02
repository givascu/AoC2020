pub fn solve_1() -> usize {
    include_str!("../input/day04.txt")
        .split("\n\n")
        .filter(|x| {
            x.contains("byr:")
                && x.contains("iyr:")
                && x.contains("eyr:")
                && x.contains("hgt:")
                && x.contains("hcl:")
                && x.contains("ecl:")
                && x.contains("pid:")
        })
        .count()
}

pub fn solve_2() -> usize {
    include_str!("../input/day04.txt")
        .split("\n\n")
        .filter(|x| {
            x.contains("byr:")
                && x.contains("iyr:")
                && x.contains("eyr:")
                && x.contains("hgt:")
                && x.contains("hcl:")
                && x.contains("ecl:")
                && x.contains("pid:")
        })
        .filter(|x| {
            let passport = x.replace('\n', " ");
            for token in passport.split(' ') {
                let mut parts = token.split(':');
                let (key, value) = (parts.next().unwrap(), parts.next().unwrap());
                if key == "byr" {
                    if let Ok(value) = value.parse::<i32>() {
                        if !(1920..=2002).contains(&value) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else if key == "iyr" {
                    if let Ok(value) = value.parse::<i32>() {
                        if !(2010..=2020).contains(&value) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else if key == "eyr" {
                    if let Ok(value) = value.parse::<i32>() {
                        if !(2020..=2030).contains(&value) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else if key == "hgt" {
                    if value.ends_with("cm") {
                        let value = value.trim_end_matches("cm");
                        if let Ok(value) = value.parse::<i32>() {
                            if !(150..=193).contains(&value) {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    } else if value.ends_with("in") {
                        let value = value.trim_end_matches("in");
                        if let Ok(value) = value.parse::<i32>() {
                            if !(59..=76).contains(&value) {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else if key == "hcl" {
                    if !value.starts_with('#') {
                        return false;
                    }
                    let value = value.trim_start_matches('#');
                    if value.len() != 6 || !value.chars().all(|c| c.is_ascii_hexdigit()) {
                        return false;
                    }
                } else if key == "ecl" {
                    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
                        return false;
                    }
                } else if key == "pid"
                    && (value.len() != 9 || !value.chars().all(|c| c.is_ascii_digit()))
                {
                    return false;
                }
            }
            true
        })
        .count()
}
