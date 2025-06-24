use std::str::Split;
use rayon::prelude::*;
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(evaluate_line)
        .sum()
}

pub fn part1_multithread(input: &str) -> usize {
    input
        .lines()
        .par_bridge()
        .into_par_iter()
        .filter_map(evaluate_line)
        .sum()
}

fn evaluate_line(line: &str) -> Option<usize> {
    let mut line = line.split(":");
    let sum = line.next()?.parse::<usize>().ok()?;
    let mut numbs = line.next()?.split(" ");
    numbs.next()?;
    let first_numb = numbs.next()?.parse::<usize>().ok()?;
    if can_reach_sum(&numbs, sum, first_numb) { Some(sum) } else { None }
}

fn can_reach_sum(remaining_terms: &Split<&str>, target_sum: usize, running_total: usize) -> bool {
    let mut remaining_terms = remaining_terms.clone();
    match remaining_terms.next() {
        None => target_sum == running_total,
        Some(next_term) => {
            let remaining_term = next_term.parse::<usize>().unwrap();
            can_reach_sum(&remaining_terms, target_sum, running_total + remaining_term)
                ||
                can_reach_sum(&remaining_terms, target_sum, running_total * remaining_term)
        }
    }
}

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::problems::day7::*;

    #[allow(unused)]
    const TEST_INPUT: &str = include_str!("day7_test.txt");

    #[test]
    fn day7_part1() {
        assert_eq!(part1(TEST_INPUT), 3749);
    }

    #[test]
    fn day7_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}