use std::fs;

fn main() {
    let input: String = fs::read_to_string("src/bin/day01.txt").expect("Could not read file");

    let cal_sums: Vec<i32> = input
        .split("\n\n")
        .map(|s| -> Vec<i32> {
            s.lines()
                .map(|f| -> i32 { f.parse::<i32>().unwrap() })
                .collect()
        })
        .map(|f| -> i32 {
            f.into_iter()
                .reduce(|a, b| -> i32 { a + b })
                .expect("could not reduce array")
        })
        .collect();

    // part 1
    let max_cal: &i32 = cal_sums.iter().max().expect("could not find max");

    println!("{:?}", max_cal);

    // part 2
    let mut sorted_cals: Vec<i32> = cal_sums.clone();
    sorted_cals.sort();
    sorted_cals.reverse();

    let max_three_cal: i32 = sorted_cals.into_iter().take(3).sum();

    println!("{:?}", max_three_cal);
}
