use std::{collections::HashSet, fs};

fn main() {
    let input: String = fs::read_to_string("src/bin/day06.txt").expect("Could not read file");

    println!("part 1 = {:?}", find_idx(input.clone(), 4));
    println!("part 2 = {:?}", find_idx(input, 14));
}

fn find_idx(input: String, window_size: usize) -> Option<usize> {
    let char_vec: Vec<char> = input.chars().into_iter().collect();

    let mut i: Option<usize> = None;

    for (idx, window) in char_vec.windows(window_size).enumerate() {
        let mut char_set: HashSet<char> = HashSet::new();

        for c in window {
            char_set.insert(*c);
        }

        if char_set.len() == window_size {
            i = Some(idx + window_size);
            break;
        }
    }

    i
}

#[cfg(test)]
mod tests {
    use crate::find_idx;

    #[test]
    fn test() {
        let test_data: Vec<(String, Option<usize>, Option<usize>)> = vec![
            (
                String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
                Some(7),
                Some(19),
            ),
            (
                String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"),
                Some(5),
                Some(23),
            ),
            (
                String::from("nppdvjthqldpwncqszvftbrmjlhg"),
                Some(6),
                Some(23),
            ),
            (
                String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
                Some(10),
                Some(29),
            ),
            (
                String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
                Some(11),
                Some(26),
            ),
        ];

        for (test_str, expected_4, expected_14) in test_data {
            assert_eq!(find_idx(test_str.clone(), 4), expected_4);
            assert_eq!(find_idx(test_str, 14), expected_14);
        }
    }
}
