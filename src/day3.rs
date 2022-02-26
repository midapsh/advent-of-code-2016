const _DUMMY_INPUT: &str = include_str!("data/day3-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day3-real.txt");

fn extract_columns(line: &str) -> impl Iterator<Item = i32> + '_ {
    line.trim()
        .split_ascii_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
}

fn private_solve_part_1(values: &str) -> String {
    values
        .lines()
        .map(|line| {
            let mut triangle_sides = extract_columns(line).collect::<Vec<_>>();

            triangle_sides.sort();
            match triangle_sides[..] {
                [x1, x2, x3] => x3 < x1 + x2,
                _ => false,
            }
        })
        .filter(|&x| x)
        .count()
        .to_string()
}

fn private_solve_part_2(values: &str) -> String {
    let mut iter_lines = values.lines();
    let mut n = 0;
    while let (Some(l1), Some(l2), Some(l3)) =
        (iter_lines.next(), iter_lines.next(), iter_lines.next())
    {
        let mut l1 = extract_columns(l1);
        let mut l2 = extract_columns(l2);
        let mut l3 = extract_columns(l3);

        while let (Some(c1), Some(c2), Some(c3)) = (l1.next(), l2.next(), l3.next()) {
            if (c3 < c1 + c2) && (c2 < c1 + c3) && (c1 < c2 + c3) {
                n += 1;
            }
        }
    }

    n.to_string()
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
        assert_eq!("0", _solve_part_1_dummy());
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
