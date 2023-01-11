use std::{fs, ops::RangeInclusive};

fn main() {
    let input: String = fs::read_to_string("src/bin/day04.txt").expect("Could not read file");

    let assignments: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> = input
        .lines()
        .into_iter()
        .map(|row| {
            let (left, right) = row.split_once(",").expect("could not split pair");
            (get_range_from_str(left), get_range_from_str(right))
        })
        .collect();

    let count_superset: usize = assignments
        .iter()
        .filter(|row| range_contains_range(&row.0, &row.1) || range_contains_range(&row.1, &row.0))
        .count();

    println!("{} are in superset", count_superset);

    let count_overlap: usize = assignments
        .iter()
        .filter(|row| range_overlaps_range(&row.0, &row.1))
        .count();

    println!("{} are in overlap", count_overlap);
}

fn get_range_from_str(from: &str) -> RangeInclusive<usize> {
    let (low, high) = from.split_once("-").expect("could not split range");
    RangeInclusive::new(
        usize::from_str_radix(low, 10).expect("could not process low"),
        usize::from_str_radix(high, 10).expect("could not process high"),
    )
}

fn range_contains_range(left: &RangeInclusive<usize>, right: &RangeInclusive<usize>) -> bool {
    left.start() <= right.start() && left.end() >= right.end()
}

fn range_overlaps_range(left: &RangeInclusive<usize>, right: &RangeInclusive<usize>) -> bool {
    left.start() <= right.end() && left.end() >= right.start()
}
