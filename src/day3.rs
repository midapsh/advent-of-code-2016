pub const DUMMY_INPUT: &str = include_str!("data/day3-dummy.txt");
pub const REAL_INPUT: &str = include_str!("data/day3-real.txt");

type Dimension = usize;

fn extract_columns(line: &str) -> impl Iterator<Item = i32> + '_ {
    line.trim()
        .split_ascii_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
}

pub fn solve_part_1(input: &str) -> String {
    input
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

pub fn solve_part_2(input: &str) -> String {
    let mut iter_lines = input.lines();
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

fn main() {
    println!("{}", solve_part_1(REAL_INPUT));
    println!("{}", solve_part_2(REAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2, DUMMY_INPUT};

    #[test]
    fn test_part_1() {
        assert_eq!("0", solve_part_1(DUMMY_INPUT));
    }
    #[test]
    fn test_part_2() {
        // assert_eq!("5DB3", solve_part_2(DUMMY_INPUT));
    }
}
