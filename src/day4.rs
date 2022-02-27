const _DUMMY_INPUT: &str = include_str!("data/day4-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day4-real.txt");

use itertools::Itertools;
use std::collections::HashMap;

fn private_solve_part_1(values: &str) -> String {
    values
        .lines()
        .map(|line| {
            let mut counter: HashMap<char, i32> = HashMap::new();
            let mut room = line.chars().filter(|&c| c != '-').peekable();
            let encrypted_name = room
                .by_ref()
                .peeking_take_while(|&c| c.is_ascii_alphabetic())
                .collect::<String>();
            let sector_id = room
                .by_ref()
                // .skip_while(|&c| c.is_ascii_alphabetic())
                .take_while(|&c| c != '[')
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            let checksum = room
                .by_ref()
                // .skip_while(|&c| c != '[')
                .take_while(|&c| c != ']')
                .collect::<String>();
            for c in encrypted_name.chars() {
                *counter.entry(c).or_insert(0) += 1;
            }
            let mut hash_vec = counter.iter().collect::<Vec<(&char, &i32)>>();
            hash_vec.sort_by(|a, b| b.1.cmp(a.1));
            let response = hash_vec
                .iter()
                .group_by(|(_, &b)| b)
                .into_iter()
                .map(|(_, group)| group.sorted().map(|(&a, _)| a).collect::<Vec<_>>())
                .flatten()
                .take(5)
                .collect::<String>();
            // let response = hash_vec.iter().take(5).map(|(&a, _)| a).collect::<String>();
            if checksum.eq(&response) {
                sector_id
            } else {
                0
            }
        })
        .fold(0, |acc, x| acc + x)
        .to_string()
}

fn private_solve_part_2(_values: &str) -> String {
    unimplemented!()
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
        assert_eq!((123 + 987 + 404).to_string(), _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("", _solve_part_2_dummy());
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
