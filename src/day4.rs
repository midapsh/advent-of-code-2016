use std::collections::HashMap;

use itertools::Itertools;

const _DUMMY_INPUT: &str = include_str!("data/day4-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day4-real.txt");
const PART_2_MUST_FIND_THIS: &str = "northpoleobjectstorage";

fn decrypt(name: &str, id: u32) -> String {
    name.chars()
        .map(|c| {
            ((((c as u32 - 'a' as u32) + id) % ('z' as u32 - 'a' as u32 + 1)) + 'a' as u32) as u8
                as char
        })
        .collect()
}

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
            hash_vec.sort_unstable_by(|&a, &b| match b.1.cmp(a.1) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => a.0.cmp(b.0),
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            });
            let response = hash_vec
                .iter()
                .map(|(&a, _b)| a)
                .take(5)
                .collect::<String>();
            if checksum == response {
                sector_id
            } else {
                0
            }
        })
        .fold(0, |acc, x| acc + x)
        .to_string()
}

fn private_solve_part_2(values: &str) -> String {
    for line in values.lines() {
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
            .parse::<u32>()
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
        hash_vec.sort_unstable_by(|&a, &b| match b.1.cmp(a.1) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => a.0.cmp(b.0),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        });
        let response = hash_vec
            .iter()
            .map(|(&a, _b)| a)
            .take(5)
            .collect::<String>();
        if checksum == response && decrypt(&encrypted_name, sector_id) == PART_2_MUST_FIND_THIS {
            return sector_id.to_string();
        }
    }
    panic!("Sector id not found!")
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
    #[should_panic]
    fn test_part_2_dummy() {
        _solve_part_2_dummy();
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
