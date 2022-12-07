use std::{fs::File, io::{BufReader, BufRead}, slice::Iter};

#[derive(PartialEq, Debug)]
enum Input {
    Command(Command),
    LSContentFile((String, u32)),
    LSContentDir(String),
}

#[derive(PartialEq, Debug)]
enum Command {
    LS,
    CD(String),
    CDBack,
}

fn parse_command(cmd: &str) -> Command {
    if !cmd.starts_with("$ ") {
        panic!("{} is not a valid command", cmd);
    }
    if &cmd[2..4] == "cd" {
        let path = &cmd[5..];
        if path == ".." {
            Command::CDBack
        } else {
            Command::CD(path.to_string())
        }
    } else {
        Command::LS
    }
}

fn parse_line(line: &str) -> Input {
    if line.starts_with("$") {
        Input::Command(parse_command(line))
    } else {
        let split = line.split(" ").collect::<Vec<&str>>();
        if split[0] == "dir" {
            Input::LSContentDir(split[1].to_string())
        } else {
            Input::LSContentFile((split[1].to_string(), split[0].parse::<u32>().unwrap()))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct DirItem {
    name: String,
    dirs: Vec<DirItem>,
    files: Vec<FileItem>,
}

impl DirItem {
    fn size(&self) -> u32 {
        let files_size = self.files.iter().map(|file| file.size).sum::<u32>();
        let dirs_size = self.dirs.iter().map(|dir| dir.size()).sum::<u32>();
        files_size + dirs_size
    }

    fn size_and_name(&self) -> Vec<(String, u32)> {
        let size = self.size();
        let mut children_size_and_names = self.dirs.iter().map(|dir| dir.size_and_name()).flatten().collect::<Vec<(String, u32)>>();
        children_size_and_names.push((self.name.clone(), size));
        children_size_and_names
    }
}

#[derive(PartialEq, Debug, Clone)]
struct FileItem {
    name: String,
    size: u32,
}

fn create_file_hierarchy_rec<'a>(iter: &mut Iter<Input>, mut current_item: DirItem) -> DirItem {
    match iter.next() {
        None | Some(Input::Command(Command::CDBack)) => {}, // do nothing
        Some(Input::Command(Command::LS)) => current_item = create_file_hierarchy_rec(iter, current_item),
        Some(Input::Command(Command::CD(new_dir_name))) => {
            let new_dir = DirItem { name: new_dir_name.to_string(), dirs: vec![], files: vec![]};
            let new_dir_visited = create_file_hierarchy_rec(iter, new_dir);
            current_item.dirs.push(new_dir_visited.to_owned());
            current_item = create_file_hierarchy_rec(iter, current_item);
        },
        Some(Input::LSContentFile((name, size))) => {
            current_item.files.push(FileItem {name: name.to_string(), size: size.to_owned()});
            current_item = create_file_hierarchy_rec(iter, current_item);
        },
        Some(Input::LSContentDir(_)) => {
            current_item = create_file_hierarchy_rec(iter, current_item);
        }
    }
    current_item
}

fn create_file_hierarchy(mut iter: Iter<Input>) -> DirItem {
    let mut root = DirItem{ name: "/".to_string(), dirs: vec![], files: vec![] };
    if let Some(_cmd) = iter.next() {
        root = create_file_hierarchy_rec(&mut iter, root);
    }
    root
}

fn main() {
    let file_name = "input/day-07";
    let file = File::open(file_name).expect("oops");
    let reader = BufReader::new(file);
    
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    let input = lines.iter().map(|line| parse_line(line)).collect::<Vec<Input>>();
    
    let root = create_file_hierarchy(input.iter());
    let used_space = root.size();
    println!("root size: {}", &used_space);

    let sizes_and_names = root.size_and_name();
    let sum_under_100000 = sizes_and_names.iter().filter(|(_, size)| size < &100_000).map(|(_, size)| size).sum::<u32>();
    println!("Sum of sizes under 100_000: {}", sum_under_100000);

    let total_space = 70_000_000;
    let free_space = total_space - used_space;
    let required_space = 30_000_000 - free_space;
    println!("required space: {}", required_space);

    let smallest_dir_size_to_remove = sizes_and_names.iter().filter(|(_, size)| size > &required_space).map(|(_, size)| size).min().unwrap();
    println!("Size of the dir to remove: {}", smallest_dir_size_to_remove);
}

#[cfg(test)]
mod tests {
    use crate::{parse_command, Command};

    #[test]
    fn parse_cd_root() {
        let cmd = "$ cd /";
        assert_eq!(parse_command(cmd), Command::CD("/".to_string()));
    }

    #[test]
    fn parse_cd_a() {
        let cmd = "$ cd a";
        assert_eq!(parse_command(cmd), Command::CD("a".to_string()));
    }
    
    #[test]
    fn parse_cd_ab() {
        let cmd = "$ cd ab";
        assert_eq!(parse_command(cmd), Command::CD("ab".to_string()));
    }
    
    #[test]
    fn parse_cd_dot_dot() {
        let cmd = "$ cd ..";
        assert_eq!(parse_command(cmd), Command::CDBack);
    }

    #[test]
    fn parse_ls() {
        let cmd = "$ ls";
        assert_eq!(parse_command(cmd), Command::LS);
    }
}