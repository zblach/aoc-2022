pub fn part_one(input: &str) -> Option<u64> {
    let mut max_total: u64 = 0;
    let mut local_total: u64 = 0;

    for inp in input.lines() {
        if inp.is_empty() {
            if local_total > max_total {
                max_total = local_total;
            }
            local_total = 0;
            continue;
        };
        local_total += inp.parse::<u64>().unwrap()
    }
    if local_total != 0 && local_total > max_total {
        max_total = local_total
    }

    Some(max_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut totals: Vec<u64> = Vec::new();
    let mut local_total: u64 = 0;
    let elf_count = 3;

    for inp in input.lines() {
        if inp.is_empty() {
            totals.push(local_total);
            local_total = 0;
            continue;
        }
        local_total += inp.parse::<u64>().unwrap()
    }
    if local_total != 0 {
        totals.push(local_total)
    }

    totals.sort_by(|a, b| b.cmp(a));

    Some(totals.into_iter().take(elf_count).sum())
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
