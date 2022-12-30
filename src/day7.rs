use aoc_2022_rust::Puzzle;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Dir {
    name: String,
}

#[derive(Debug, Clone)]
struct File {
    _name: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct Contents {
    dirs: Vec<Dir>,
    files: Vec<File>,
}

impl Contents {
    fn new() -> Contents {
        Contents {
            dirs: Vec::new(),
            files: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Day7 {
    input: HashMap<String, Contents>,
}

impl Day7 {
    pub fn new() -> Day7 {
        Day7 {
            input: HashMap::new(),
        }
    }

    pub fn _clear(&mut self) {
        self.input = HashMap::new();
    }
}

fn parse_file(line: &str) -> File {
    let size = line
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let name = line
        .chars()
        .skip_while(|c| c.is_digit(10) || c.is_whitespace())
        .collect::<String>();
    File { _name: name, size }
}

impl Puzzle for Day7 {
    fn load_input(&mut self) {
        const INPUT: &str = include_str!("../inputs/7.input");
        let mut tree = HashMap::from([("".to_string(), Contents::new())]);
        let mut current = String::new();
        for line in INPUT.lines() {
            if line.starts_with("$ cd /") {
                current = "".to_string();
            } else if line.starts_with("$ cd ..") {
                current = current.rsplit_once('/').unwrap().0.to_string();
            } else if line.starts_with("$ cd ") {
                let name = format!("/{:}", line.chars().skip(5).collect::<String>());
                current.push_str(&name);
            } else if line.starts_with("dir") {
                let name = format!("{:}/{:}", current, line.chars().skip(4).collect::<String>());
                if tree.get(&name).is_none() {
                    tree.insert(name.clone(), Contents::new());
                    if let Some(t) = tree.get_mut(&current) {
                        t.dirs.push(Dir { name });
                    }
                }
            } else if line.chars().nth(0).unwrap().is_digit(10) {
                let file = parse_file(&line);
                if let Some(t) = tree.get_mut(&current) {
                    t.files.push(file);
                }
            }
        }
        self.input = tree;
    }

    fn part1(&self) -> String {
        let mut dir_sizes = HashMap::<String, usize>::new();
        traverse("", &self.input, &mut dir_sizes);
        format!(
            "{:}",
            dir_sizes.values().filter(|&v| *v <= 100000).sum::<usize>()
        )
    }

    fn part2(&self) -> String {
        let size_of_device = 70000000usize;
        let mut dir_sizes = HashMap::<String, usize>::new();
        traverse("", &self.input, &mut dir_sizes);
        let available = size_of_device - dir_sizes[""];
        let needed = 30000000usize - available;
        let mut pick = dir_sizes[""];
        for (_, size) in dir_sizes {
            if size >= needed && size < pick {
                pick = size
            }
        }
        format!("{:}", pick)
    }
}

fn traverse(
    root: &str,
    data: &HashMap<String, Contents>,
    dir_sizes: &mut HashMap<String, usize>,
) -> usize {
    let mut total_size = data[root].files.iter().map(|f| f.size).sum::<usize>();
    for dir in &data[root].dirs {
        if let Some(size) = dir_sizes.get(&dir.name) {
            total_size += size;
        } else {
            let size = traverse(&dir.name, data, dir_sizes);
            dir_sizes.insert(dir.name.clone(), size);
            total_size += size;
        }
    }
    dir_sizes.insert(root.to_string(), total_size);
    total_size
}
