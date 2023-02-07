use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input: String = fs::read_to_string("src/bin/day03.txt").expect("Could not read file");
    let mut count: usize = 0;
    let mut priority_map: HashMap<char, usize> = HashMap::new();
    for (idx, letter) in (b'a'..=b'z').chain(b'A'..=b'Z').enumerate() {
        priority_map.insert(
            char::from_u32(letter.into()).expect("could not parse byte into char"),
            idx + 1,
        );
    }

    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let left_set: HashSet<char> = left.chars().collect();
        let right_set: HashSet<char> = right.chars().collect();

        let intersection: Vec<&char> = left_set.intersection(&right_set).collect();
        assert!(
            intersection.len() == 1,
            "not exactly one thing intersected {:?} : left {:?} right {:?}",
            intersection,
            left_set,
            right_set
        );
        let item = intersection[0];
        count += priority_map
            .get(item)
            .expect("could not find {} in priority map")
    }

    println!("part 1 count {}", count);

    let mut count2: usize = 0;
    for set in input.lines().collect::<Vec<&str>>().chunks(3) {
        let first: HashSet<char> = set[0].chars().collect();
        let second: HashSet<char> = set[1].chars().collect();
        let third: HashSet<char> = set[2].chars().collect();

        let fs: HashSet<char> = first.intersection(&second).copied().collect();

        let fst: HashSet<char> = fs.intersection(&third).copied().collect();
        assert!(fst.len() == 1, "intersection contained more than 1");

        let item = fst.iter().next().expect("no item found");

        count2 += priority_map.get(item).expect("no value found");
    }

    println!("part 2 count {}", count2);
}
