use rayon::prelude::*;
use std::cmp::PartialEq;
use std::num::ParseIntError;
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|x|Part::Part1.evaluate_line(x))
        .sum()
}

pub fn part1_multithread(input: &str) -> usize {
    input
        .lines()
        .par_bridge()
        .into_par_iter()
        .filter_map(|x|Part::Part1.evaluate_line(x))
        .sum()
}

#[derive(Debug,PartialEq)]
enum Part {
    Part1,
    Part2,
}

impl Part {
    fn evaluate_line(&self, line: &str) -> Option<usize> {
        let mut line = line.split(":");
        let sum = line.next()?.parse::<usize>().ok()?;
        let mut numbs = line.next()?.split(" ").map(|num| num.parse::<usize>());
        _ = numbs.next()?;
        let first_numb = numbs.next()?.ok()?;
        if self.can_reach_sum(&numbs, sum, first_numb) { Some(sum) } else { None }
    }
    fn can_reach_sum<I: Iterator<Item = Result<usize, ParseIntError>> + Clone>(
        &self,
        remaining_terms: &I,
        target_sum: usize,
        running_total: usize
    ) -> bool
    {
        if running_total > target_sum { return false }
        let mut remaining_terms = remaining_terms.clone();
        match remaining_terms.next() {
            Some(Ok(next_term)) =>
                self.can_reach_sum(&remaining_terms, target_sum, running_total + next_term)
                ||
                self.can_reach_sum(&remaining_terms, target_sum, running_total * next_term)
                ||
                (self == &Part::Part2 && self.can_reach_sum(&remaining_terms, target_sum, running_total*(10usize.pow(next_term.ilog10() + 1)) + next_term)),
            _ => target_sum == running_total,
        }
    }
}


pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|x|Part::Part2.evaluate_line(x))
        .sum()
}

pub fn part2_multithread(input: &str) -> usize {
    input
        .lines()
        .par_bridge()
        .into_par_iter()
        .filter_map(|x|Part::Part2.evaluate_line(x))
        .sum()
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
        assert_eq!(part2(TEST_INPUT), 11387);
    }
}