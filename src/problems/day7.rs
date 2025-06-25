use rayon::prelude::*;
use std::cmp::PartialEq;
use std::num::ParseIntError;
use std::thread;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|x|Part::Part1.evaluate_line(x))
        .sum()
}

pub fn part1_multithread(input: &str) -> usize {
    Part::Part1.evaluate_input_multithread(input)
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
        let mut reverse_numbs = numbs.clone().collect::<Vec<_>>();
        reverse_numbs.reverse();
        let reverse_numbs = reverse_numbs.into_iter();
        //let last_numb = reverse_numbs.next()?.ok()?;
        // assert_eq!(
        //     self.can_reach_sum_forward(&numbs, sum, first_numb),
        //     self.can_reach_sum_backward(&reverse_numbs, first_numb, sum)
        // );
        if self.can_reach_sum_backward(&reverse_numbs, first_numb, sum) { Some(sum) } else { None }
    }
    #[allow(dead_code)]
    fn can_reach_sum_forward<I: Iterator<Item = Result<usize, ParseIntError>> + Clone>(
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
                self.can_reach_sum_forward(&remaining_terms, target_sum, running_total + next_term)
                ||
                self.can_reach_sum_forward(&remaining_terms, target_sum, running_total * next_term)
                ||
                (self == &Part::Part2 && self.can_reach_sum_forward(&remaining_terms, target_sum, running_total*(10usize.pow(next_term.ilog10() + 1)) + next_term)),
            _ => target_sum == running_total,
        }
    }
    fn can_reach_sum_backward<I: Iterator<Item = Result<usize, ParseIntError>> + Clone>(
        &self,
        remaining_terms: &I,
        initial_value: usize,
        running_total: usize
    ) -> bool {
        if running_total < initial_value { return false }
        let mut remaining_terms = remaining_terms.clone();
        match remaining_terms.next() {
            Some(Ok(next_term)) =>
                (
                    running_total >= next_term
                    &&
                    self.can_reach_sum_backward(&remaining_terms, initial_value, running_total - next_term)
                )
                ||
                (
                    running_total % next_term == 0
                    &&
                    self.can_reach_sum_backward(&remaining_terms, initial_value, running_total / next_term)
                )
                ||
                (
                    self == &Part::Part2
                    &&
                    running_total % 10usize.pow(next_term.ilog10()+1) == next_term
                    &&
                    self.can_reach_sum_backward(&remaining_terms, initial_value, (running_total - next_term)/10usize.pow(next_term.ilog10()+1))
                ),
            _ => initial_value == running_total,
        }
    }
    fn evaluate_input_multithread(&self, input: &str) -> usize {
        let _pool = rayon::ThreadPoolBuilder::new()
            .num_threads(thread::available_parallelism().unwrap().get()/2)
            .build()
            .unwrap();
        input
            .lines()
            .par_bridge()
            .into_par_iter()
            .filter_map(|x|self.evaluate_line(x))
            .sum()
    }
}


pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|x|Part::Part2.evaluate_line(x))
        .sum()
}

pub fn part2_multithread(input: &str) -> usize {
    Part::Part2.evaluate_input_multithread(input)
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