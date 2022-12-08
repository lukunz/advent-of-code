use std::fs;
use std::slice::Iter;

#[derive(Debug, PartialEq)]
enum Statement<'a> {
    ChangeDir(&'a str),
    List,
    Directory(&'a str),
    File(&'a str, usize),
}

struct File<'a> {
    _name: &'a str,
    size: usize,
}

struct Directory<'a> {
    name: &'a str,
    files: Vec<File<'a>>,
    directories: Vec<Directory<'a>>,
}

impl<'a> Directory<'a> {
    fn with_name(name: &'a str) -> Self {
        Directory {
            name,
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    fn find_directory_index(&mut self, dir_name: &str) -> Option<usize> {
        self.directories.iter().position(|dir| dir.name == dir_name)
    }

    fn add_directory(&mut self, directory: Directory<'a>) -> usize {
        self.directories.push(directory);
        self.directories.len() - 1
    }

    fn add_file(&mut self, file: File<'a>) {
        self.files.push(file);
    }

    fn find_small_directories(
        &'a self,
        small_directories: &mut Vec<(&'a Directory<'a>, usize)>,
        limit: usize,
    ) -> usize {
        let size = self.files.iter().map(|file| file.size).sum::<usize>()
            + self
                .directories
                .iter()
                .map(|directory| directory.find_small_directories(small_directories, limit))
                .sum::<usize>();

        if size <= limit {
            small_directories.push((&self, size));
        }

        size
    }
}

fn parse_line(line: &str) -> Result<Statement, &'static str> {
    let (first, rest) = line.split_once(' ').ok_or("Unknown line format")?;

    match first {
        "$" => match rest.split_once(' ') {
            Some(("cd", dir_name)) => Ok(Statement::ChangeDir(dir_name)),
            None => Ok(Statement::List),
            Some((&_, _)) => Err("Unknown command"),
        },
        "dir" => Ok(Statement::Directory(rest)),
        size_string => match size_string.parse() {
            Ok(size) => Ok(Statement::File(rest, size)),
            Err(_) => Err("Expected number"),
        },
    }
}

fn parse_input(input: &str) -> Result<Vec<Statement>, &'static str> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn execute_statements_on_dir<'a>(
    directory: &mut Directory<'a>,
    statement_iter: &mut Iter<Statement<'a>>,
) {
    while let Some(statement) = statement_iter.next() {
        match statement {
            Statement::List => {}
            Statement::ChangeDir("..") => return,
            Statement::ChangeDir(dir_name) => {
                match directory.find_directory_index(dir_name) {
                    Some(index) => {
                        execute_statements_on_dir(&mut directory.directories[index], statement_iter)
                    }
                    None => {}
                };
            }
            Statement::Directory(name) => {
                directory.add_directory(Directory::with_name(name));
            }
            Statement::File(name, size) => directory.add_file(File {
                _name: name,
                size: *size,
            }),
        }
    }
}

fn create_filesystem(statements: Vec<Statement>) -> Directory {
    let mut statement_iter = statements.iter();

    statement_iter.next();

    let mut root = Directory::with_name("/");

    execute_statements_on_dir(&mut root, &mut statement_iter);

    root
}

fn size_of_small_directories(directory: &Directory, limit: usize) -> Result<usize, &'static str> {
    let mut small_directories = Vec::new();
    directory.find_small_directories(&mut small_directories, limit);

    Ok(small_directories.iter().map(|(_, size)| size).sum())
}

fn find_smallest_directory_to_delete<'a>(
    directory: &'a Directory<'a>,
    target: usize,
) -> Option<usize> {
    let mut directories = Vec::new();
    directory.find_small_directories(&mut directories, usize::MAX);

    directories.sort_by(|(_, a), (_, b)| a.cmp(b));
    let root = directories.pop().unwrap();

    match directories.iter().find(|(_, size)| root.1 - size <= target) {
        Some((_, size)) => Some(*size),
        None => None,
    }
}

fn main() {
    let data = fs::read_to_string("day7/input.txt").expect("Can't read input file");
    let statements = parse_input(&data).unwrap();
    let directory = create_filesystem(statements);

    let result_part_one = size_of_small_directories(&directory, 100000).unwrap();
    let result_part_two = find_smallest_directory_to_delete(&directory, 40000000).unwrap();

    println!("Part one: {result_part_one}");
    println!("Part two: {}", result_part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ls_command() {
        assert_eq!(Ok(Statement::List), parse_line("$ ls"));
    }

    #[test]
    fn test_parse_cd_command() {
        assert_eq!(Ok(Statement::ChangeDir("a")), parse_line("$ cd a"));
        assert_eq!(Ok(Statement::ChangeDir("test")), parse_line("$ cd test"));
        assert_eq!(Ok(Statement::ChangeDir("/")), parse_line("$ cd /"));
        assert_eq!(Ok(Statement::ChangeDir("..")), parse_line("$ cd .."));
    }

    #[test]
    fn test_directory_line() {
        assert_eq!(Ok(Statement::Directory("a")), parse_line("dir a"));
        assert_eq!(Ok(Statement::Directory("test")), parse_line("dir test"));
    }

    #[test]
    fn test_file_line() {
        assert_eq!(Ok(Statement::File("a", 100)), parse_line("100 a"));
        assert_eq!(Ok(Statement::File("test", 39584)), parse_line("39584 test"));
    }

    #[test]
    fn test_small_input() {
        let data = fs::read_to_string("input-small.txt").expect("Can't read input file");
        let statements = parse_input(&data).unwrap();
        let directory = create_filesystem(statements);
        let result = size_of_small_directories(&directory, 100000);

        assert_eq!(Ok(95437), result);
    }

    #[test]
    fn test_small_input2() {
        let data = fs::read_to_string("input-small.txt").expect("Can't read input file");
        let statements = parse_input(&data).unwrap();
        let directory = create_filesystem(statements);
        let result = find_smallest_directory_to_delete(&directory, 40000000).unwrap();

        assert_eq!(24933642, result);
    }
}
