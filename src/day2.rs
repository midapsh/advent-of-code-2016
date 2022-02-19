const DUMMY_INPUT: &str = include_str!("data/day2-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day2-real.txt");

pub fn solve(value: &str) -> &str {
    unimplemented!()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_part1() {
        assert_eq!("1985", solve("ULL\nRRDDD\nLURDL\nUUUUD"));
    }
}
