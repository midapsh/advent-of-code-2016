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
        assert_eq!(
            "1985",
            solve(
                "ULL
        RRDDD
        LURDL
        UUUUD"
            )
        );
    }
}
