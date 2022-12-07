use std::collections::HashMap;

use itertools::Itertools;
/*
trait VSize {
    fn siz(&self) -> u32;
}
struct VDir{
    parent: Option<*mut VDir>,
    ls: HashMap<String, Box<dyn VSize>>,
}
struct VFile(u32);

impl VSize for VFile {
    fn siz(&self) -> u32 {
        return self.0;
    }
}

impl VSize for VDir {
    fn siz(&self) -> u32 {
        let mut ret:u32 = 0;
        for v in self.ls.values() {
            ret += v.siz()
        }
        ret
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut root = VDir{parent: None, ls: HashMap::new()};
    root.parent = Some(&mut root);

    let mut dir = &mut root;

    for line in input.lines() {
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks[0] != "$" {
            // continue adding file
            let name = String::from(toks[1]);
            if toks[0] == "dir" {
                dir.ls.insert(name, Box::new(VDir{parent: Some(*mut), ls: HashMap::new()}));
            } else {
                dir.ls.insert(name, Box::new(VFile(toks[0].parse::<u32>().unwrap())));
            }

        } else {
            match &toks[1..] {
                ["cd", "/"] => dir = &mut root,
                ["cd", ".."] => dir = unsafe {dir.parent.unwrap().as_mut().unwrap()}, // TODO. get parent
                ["cd", _] => {},
                ["ls"] => {},

                _ => panic!("{:?}", toks)

            }
            // parse command
        }
    }

    None
}
*/

fn get_dir_sizes(input: &str) -> HashMap<String, u32> {
    let mut file_table = HashMap::<String, u32>::new();
    let mut folders: Vec<String> = Vec::new();
    let mut path: Vec<&str> = Vec::new();

    for line in input.lines() {
        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks[0] == "$" {
            match &toks[1..] {
                ["cd", "/"] => path.clear(),
                ["cd", ".."] => {
                    let _x = path.pop();
                }
                ["cd", _] => path.push(toks[2]),
                ["ls"] => {}

                _ => panic!(),
            }
        } else {
            let mut s = path.clone();
            s.push(toks[1]);

            let v = s.join("/");

            if toks[0] == "dir" {
                folders.push(v);
            } else {
                file_table.insert(v, toks[0].parse::<u32>().unwrap());
            }
        }
    }

    // find dir sizes
    let mut dir_sizes = HashMap::<String, u32>::new();
    for dir in folders.clone() {
        let mut sum: u32 = 0;
        for (file, size) in file_table.clone().iter() {
            if file.starts_with(&dir) {
                sum += size;
            }
        }
        dir_sizes.insert(dir, sum);
    }

    // finally, total
    let mut sum = 0;
    for (_, size) in file_table.clone() {
        sum += size;
    }
    dir_sizes.insert(String::from(""), sum);

    dir_sizes
}

pub fn part_one(input: &str) -> Option<u32> {
    let dir_sizes = get_dir_sizes(input);

    // filter sum for 100000
    let mut sum = 0;
    for (_, size) in dir_sizes {
        if size <= 100000 {
            sum += size;
        }
    }

    Some(sum)
}
pub fn part_two(input: &str) -> Option<u32> {
    let dir_sizes = get_dir_sizes(input);

    let disk_total: u32 = 70000000;
    let target_free: u32 = 30000000;
    let total_used = dir_sizes.get("").unwrap();

    let unused = disk_total - total_used;
    let to_free = target_free - unused;

    let mut sorted = dir_sizes
        .iter()
        .sorted_unstable_by(|a, b| Ord::cmp(a.1, b.1));

    let targ = sorted.find(|&x| x.1 > &to_free).unwrap();

    Some(*targ.1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
