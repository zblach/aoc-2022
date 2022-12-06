use std::collections::VecDeque;

use scanf::sscanf;

pub fn part_one(input: &str) -> Option<String> {
    let first_line = input.lines().next().unwrap();
    let num_stacks = (first_line.len() + 1) / 4;

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(VecDeque::new());
    }

    let mut line_iter = input.lines();

    for line in &mut line_iter {
        if line.starts_with(" 1 ") {
            break;
        }
        let chrs = line.as_bytes();
        for i in 0..num_stacks {
            let val = chrs[i * 4 + 1] as char;
            if val == ' ' {
                continue;
            }
            stacks[i].push_back(val)
        }
    }

    line_iter.next(); // skip empty line

    for line in line_iter {
        let (mut qty, mut src, mut dst) = (0, 0, 0);
        sscanf!(line, "move {} from {} to {}", qty, src, dst).unwrap();

        src -= 1;
        dst -= 1;

        for _ in 0..qty {
            let v = stacks[src].pop_front().unwrap();
            stacks[dst].push_front(v);
        }
    }

    let mut result = String::new();
    for mut stk in stacks {
        result.push(stk.pop_front().unwrap_or(' '))
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let first_line = input.lines().next().unwrap();
    let num_stacks = (first_line.len() + 1) / 4;

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(VecDeque::new());
    }

    let mut line_iter = input.lines();

    for line in &mut line_iter {
        if line.starts_with(" 1 ") {
            break;
        }
        let chrs = line.as_bytes();
        for i in 0..num_stacks {
            let val = chrs[i * 4 + 1] as char;
            if val == ' ' {
                continue;
            }
            stacks[i].push_back(val)
        }
    }

    line_iter.next(); // skip empty line

    for line in line_iter {
        let (mut qty, mut src, mut dst) = (0, 0, 0);
        sscanf!(line, "move {} from {} to {}", qty, src, dst).unwrap();

        src -= 1;
        dst -= 1;

        let mut tmp: Vec<char> = Vec::new();

        for _ in 0..qty {
            let t = stacks[src].pop_front().unwrap();
            tmp.push(t);
        }
        for _ in 0..qty {
            let t = tmp.pop().unwrap();
            stacks[dst].push_front(t);
        }
    }

    let mut result = String::new();
    for mut stk in stacks {
        result.push(stk.pop_front().unwrap_or(' '))
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
