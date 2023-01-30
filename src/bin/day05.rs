use regex::Regex;
use std::{collections::HashMap, fs, str::Lines};

type CraneStack = HashMap<usize, Vec<char>>;

const RULE_PATTERN: &str = r"move (\d+) from (\d+) to (\d+)";

fn main() {
    let input: String = fs::read_to_string("src/bin/day05.txt").expect("Could not read file");

    // Separate existing stack from operations
    let (stacks, rules_raw): (&str, &str) = input
        .split_once("\n\n")
        .expect("Could not parse input sections");

    let rules: Vec<&str> = rules_raw.lines().collect();

    let mut crane_stacks = build_stacks(stacks);

    process_rules(&mut crane_stacks, rules);

    println!("pick tops {}", pick_tops(&crane_stacks));
}

fn build_stacks(stacks: &str) -> CraneStack {
    let mut crane_stacks: CraneStack = HashMap::new();

    let stack_lines: Lines = stacks.lines();

    // Reverse iteration order so we get count first
    let mut rev_stack_lines = stack_lines.rev();

    let total_cols = rev_stack_lines
        .next()
        .expect("no number row")
        .trim()
        .split_ascii_whitespace()
        .last()
        .expect("no final column number")
        .parse::<usize>()
        .expect("could not parse col num");

    assert!(total_cols >= 1);

    for i in 1..=total_cols {
        crane_stacks.insert(i, Vec::new());
    }

    println!("stacks {:?}", crane_stacks);

    for stack_line in rev_stack_lines {
        stack_line
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .enumerate()
            .for_each(|(idx_raw, cell)| {
                let idx = idx_raw + 1;
                assert!(
                    crane_stacks.contains_key(&idx),
                    "size {} idx {}",
                    crane_stacks.len(),
                    idx
                );

                let cell_str: String = cell.iter().collect::<String>();
                let cell_str_trim: &str = cell_str.trim();

                if !cell_str_trim.is_empty() {
                    assert!(cell_str_trim.len() == 3);
                    let val = cell_str_trim
                        .chars()
                        .nth(1)
                        .expect("could not decode cell str");

                    crane_stacks.entry(idx).and_modify(|stack| stack.push(val));
                }
            });
    }

    crane_stacks
}

fn process_rules(crane_stacks: &mut CraneStack, rules: Vec<&str>) {
    let rule_regex: Regex = Regex::new(RULE_PATTERN).unwrap();

    for rule in rules {
        for cap in rule_regex.captures_iter(rule) {
            let (qty, start, end) = (
                cap[1].parse::<usize>().unwrap(),
                &cap[2].parse::<usize>().unwrap(),
                &cap[3].parse::<usize>().unwrap(),
            );

            // move_crates_p1(crane_stacks, qty, start, end);
            move_crates_p2(crane_stacks, qty, start, end);
        }
    }
}

fn _move_crates_p1(crane_stacks: &mut CraneStack, qty: usize, start: &usize, end: &usize) {
    for q in 0..qty {
        match crane_stacks.get_mut(start).unwrap().pop() {
            Some(i) => crane_stacks.get_mut(end).unwrap().push(i),
            None => panic!(
                "attempted to pull from a stack that was empty! qty {} q {} start {} end {}",
                qty, q, start, end
            ),
        }
    }
}

fn move_crates_p2(crane_stacks: &mut CraneStack, qty: usize, start: &usize, end: &usize) {
    let start_stack = crane_stacks.get_mut(start).unwrap();

    let mut buffer: Vec<char> = (0..qty)
        .into_iter()
        .map(|_| start_stack.pop().unwrap())
        .collect();
    buffer.reverse();

    let end_stack = crane_stacks.get_mut(end).unwrap();
    for b in buffer {
        end_stack.push(b);
    }
}

fn pick_tops(crane_stacks: &CraneStack) -> String {
    (1..=crane_stacks.len())
        .into_iter()
        .filter_map(|idx| crane_stacks.get(&idx).unwrap().last())
        .collect()
}

fn _print_crane(crane_stacks: &CraneStack) -> String {
    let mut buffer: String = String::new();
    buffer.push('{');

    for i in 1..=crane_stacks.len() {
        buffer.push_str(format!("{}: {:?}, ", i, crane_stacks.get(&i).unwrap()).as_str());
    }

    buffer.push('}');
    buffer
}
