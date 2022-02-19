pub const DUMMY_INPUT: &str = include_str!("data/day2-dummy.txt");
pub const REAL_INPUT: &str = include_str!("data/day2-real.txt");

pub const KEYPAD_PART_1: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
pub const KEYPAD_PART_2: [[char; 5]; 5] = [
    ['X', 'X', '1', 'X', 'X'],
    ['X', '2', '3', '4', 'X'],
    ['5', '6', '7', '8', '9'],
    ['X', 'A', 'B', 'C', 'X'],
    ['X', 'X', 'D', 'X', 'X'],
];

type Dimension = usize;

pub fn solve_part_1(input: &str) -> String {
    let mut x: Dimension = 1;
    let mut y: Dimension = 1;
    input
        .lines()
        .map(|line| {
            for step in line.chars() {
                match step {
                    'U' => {
                        y = y.saturating_sub(1);
                    }
                    'D' => {
                        y = Dimension::min(2, y + 1);
                        // y = (y + 1).clamp(0, 2);
                        // y = 2.min(y + 1);
                    }
                    'L' => {
                        x = x.saturating_sub(1);
                    }
                    'R' => {
                        x = Dimension::min(2, x + 1);
                        // x = (x + 1).clamp(0, 2);
                        // x = 2.min(x + 1);
                    }
                    _ => panic!("Input not allowed"),
                }
            }
            KEYPAD_PART_1[y][x]
        })
        .collect()
}

pub fn solve_part_2(input: &str) -> String {
    let mut y: Dimension = 2;
    let mut x: Dimension = 0;
    input
        .lines()
        .map(|line| {
            for step in line.chars() {
                match step {
                    'U' => {
                        if KEYPAD_PART_2[y.saturating_sub(1)][x] != 'X' {
                            y = y.saturating_sub(1);
                        }
                    }
                    'D' => {
                        if KEYPAD_PART_2[Dimension::min(4, y + 1)][x] != 'X' {
                            y = Dimension::min(4, y + 1);
                        }
                        // y = (y + 1).clamp(0, 2);
                        // y = 2.min(y + 1);
                    }
                    'L' => {
                        if KEYPAD_PART_2[y][x.saturating_sub(1)] != 'X' {
                            x = x.saturating_sub(1);
                        }
                    }
                    'R' => {
                        if KEYPAD_PART_2[y][Dimension::min(4, x + 1)] != 'X' {
                            x = Dimension::min(4, x + 1);
                        }
                        // x = (x + 1).clamp(0, 2);
                        // x = 2.min(x + 1);
                    }
                    _ => panic!("Input not allowed"),
                }
            }
            KEYPAD_PART_2[y][x]
        })
        .collect()
}

fn main() {
    println!("{}", solve_part_1(REAL_INPUT));
    println!("{}", solve_part_2(REAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2, DUMMY_INPUT};

    #[test]
    fn test_part1() {
        assert_eq!("1985", solve_part_1(DUMMY_INPUT));
    }
    #[test]
    fn test_part2() {
        assert_eq!("5DB3", solve_part_2(DUMMY_INPUT));
    }
}
