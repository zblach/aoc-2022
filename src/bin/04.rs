use scanf::sscanf;

struct pair(i32, i32);

impl pair {
    fn contains(&self, other: &pair) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &pair) -> bool {
        self.0 <= other.0 && other.0 <= self.1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut overlap_count = 0;

    for line in input.lines() {
        let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
        sscanf!(line, "{}-{},{}-{}", a, b, c, d).unwrap();

        let (r1, r2) = (pair(a, b), pair(c, d));

        if r1.contains(&r2) || r2.contains(&r1) {
            overlap_count += 1;
        }
    }
    Some(overlap_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut overlap_count = 0;

    for line in input.lines() {
        let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
        sscanf!(line, "{}-{},{}-{}", a, b, c, d).unwrap();

        let (r1, r2) = (pair(a, b), pair(c, d));

        if r1.overlaps(&r2) || r2.overlaps(&r1) {
            overlap_count += 1;
        }
    }
    Some(overlap_count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
