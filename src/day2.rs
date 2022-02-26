const _DUMMY_INPUT: &str = include_str!("data/day2-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day2-real.txt");

const KEYPAD_PART_1: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
const KEYPAD_PART_2: [[char; 5]; 5] = [
    ['X', 'X', '1', 'X', 'X'],
    ['X', '2', '3', '4', 'X'],
    ['5', '6', '7', '8', '9'],
    ['X', 'A', 'B', 'C', 'X'],
    ['X', 'X', 'D', 'X', 'X'],
];

type Dimension = usize;

fn private_solve_part_1(values: &str) -> String {
    let mut x: Dimension = 1;
    let mut y: Dimension = 1;
    values
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

fn private_solve_part_2(values: &str) -> String {
    let mut y: Dimension = 2;
    let mut x: Dimension = 0;
    values
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
        assert_eq!("1985", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("5DB3", _solve_part_2_dummy());
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
