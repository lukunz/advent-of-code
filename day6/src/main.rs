use std::collections::HashSet;
use std::fs;

fn find_start(data: &str, marker_size: usize) -> Option<usize> {
    let mut char_set = HashSet::with_capacity(marker_size);

    for index in marker_size..=data.len() {
        let window = data.get(index - marker_size..index).expect("parse error");

        for c in window.chars() {
            char_set.insert(c);
        }

        if char_set.len() == marker_size {
            return Some(index);
        }

        char_set.clear();
    }

    None
}

fn main() {
    let data = fs::read_to_string("day6/input.txt").expect("Can't read input file");
    let part_one = find_start(data.as_str(), 4).unwrap();
    let part_two = find_start(data.as_str(), 14).unwrap();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(find_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap(), 5);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(find_start("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap(), 6);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap(), 10);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(find_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap(), 11);
    }

    #[test]
    fn test_example_5() {
        assert_eq!(find_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap(), 19);
    }

    #[test]
    fn test_example_6() {
        assert_eq!(find_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap(), 23);
    }

    #[test]
    fn test_example_7() {
        assert_eq!(find_start("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap(), 23);
    }

    #[test]
    fn test_example_8() {
        assert_eq!(find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap(), 29);
    }

    #[test]
    fn test_example_9() {
        assert_eq!(find_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap(), 26);
    }
}
