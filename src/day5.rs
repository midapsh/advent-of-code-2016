const _DUMMY_INPUT: &str = include_str!("data/day5-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day5-real.txt");
const PASSWORD_LENGTH: usize = 8;
const HASH_MATCH: &str = "00000";

fn private_solve_part_1(values: &str) -> String {
    let mut output = String::new();
    let mut index = 0;
    while output.len() < PASSWORD_LENGTH {
        let data = values.to_string() + &index.to_string();
        let hash = format!("{:x}", md5::compute(data.as_bytes()));
        if let Some(suffix) = hash.strip_prefix(HASH_MATCH) {
            output.push(suffix.chars().next().unwrap());
        }
        index += 1;
    }

    output
}

fn private_solve_part_2(values: &str) -> String {
    let mut output = ['-'; PASSWORD_LENGTH];
    let mut index: i32 = 0;
    while output.iter().any(|&c| c == '-') {
        let data = values.to_string() + &index.to_string();
        let hash = format!("{:x}", md5::compute(data.as_bytes()));
        if let Some(suffix) = hash.strip_prefix(HASH_MATCH) {
            let mut chars = suffix.chars();
            let word_index = (chars.next().unwrap() as u32 - '0' as u32) as usize;
            let word = chars.next().unwrap();
            if word_index < output.len() && output[word_index] == '-' {
                output[word_index] = word;
            }
        }
        index += 1;
    }

    output.iter().collect()
}

fn _solve_part_1_dummy() -> String {
    private_solve_part_1(_DUMMY_INPUT)
}

pub fn solve_part_1_real() -> String {
    private_solve_part_1(REAL_INPUT)
}

fn _solve_part_2_dummy() -> String {
    private_solve_part_2(_DUMMY_INPUT)
}

pub fn solve_part_2_real() -> String {
    private_solve_part_2(REAL_INPUT)
}

#[cfg(test)]
mod tests {
    use super::{_solve_part_1_dummy, _solve_part_2_dummy, solve_part_1_real, solve_part_2_real};

    #[test]
    fn test_part_1_dummy() {
        assert_eq!("18f47a30", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("05ace8e3", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real());
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real());
    }
}
