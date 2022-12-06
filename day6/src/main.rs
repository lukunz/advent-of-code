use std::collections::HashSet;
use std::fs;

fn find_start(data: &str) -> Option<usize> {
    let mut char_set = HashSet::with_capacity(4);

    for index in 4..=data.len() {
        let window = data.get(index - 4..index).expect("parse error");

        for c in window.chars() {
            char_set.insert(c);
        }

        if char_set.len() == 4 {
            return Some(index);
        }

        char_set.clear();
    }

    None
}

fn main() {
    let data = fs::read_to_string("day6/input.txt").expect("Can't read input file");
    let result = find_start(data.as_str()).unwrap();

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(find_start("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(find_start("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 10);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(find_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 11);
    }
}
