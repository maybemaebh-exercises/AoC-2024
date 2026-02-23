use rayon::prelude::*;
use std::cmp::PartialEq;
use std::num::ParseIntError;
use std::thread;

pub struct Day7();
impl crate::Day for Day7 {
    fn part1(&self, input: &str) -> Option<usize> { Some(
        input
        .lines()
        .filter_map(|x|Part::Part1.evaluate_line(x))
        .sum()
    ) }

    fn part2(&self, input: &str) -> Option<usize> { Some(
        input
            .lines()
            .filter_map(|x|Part::Part2.evaluate_line(x))
            .sum()
    ) }

    fn part1_multithreaded(&self, input: &str) -> Option<usize> {
        Some(Part::Part1.evaluate_input_multithread(input))
    }

    fn part2_multithreaded(&self, input: &str) -> Option<usize> {
        Some(Part::Part2.evaluate_input_multithread(input))
    }

    fn full_input(&self) -> &'static str {
        include_str!("../../input/day7.txt")
    }

    fn problem_name(&self) -> &'static str {
        "Bridge Repair"
    }
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
        let parameter_section = line.next()?;
        let mut numbs = parameter_section.split(" ").map(|num| num.parse::<usize>());
        _ = numbs.next()?;
        let first_numb = numbs.next()?.ok()?;
        // let mut reverse_numbs_vec = numbs.clone().collect::<Vec<_>>();
        // reverse_numbs_vec.reverse();
        // let reverse_numbs_vec = reverse_numbs_vec.into_iter();
        let reverse_numbs = parameter_section
            .rsplit(" ")
            .map(|num| num.parse::<usize>())
            .enumerate()
            .filter_map(|x|if x.0 <= parameter_section.split(" ").count()-3 {Some(x.1)} else {None});
        // assert!(itertools::equal(
        //     reverse_numbs.clone(),
        //     reverse_numbs_vec.clone()
        // ));
        // for x in reverse_numbs.clone() {
        //     print!("{x:?}");
        // }
        // println!(".");
        // for x in reverse_numbs_vec.clone() {
        //     print!("{x:?}");
        // }
        // println!("next:");

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
                self.can_reach_sum_forward(&remaining_terms, target_sum, running_total * next_term)
                ||
                self.can_reach_sum_forward(&remaining_terms, target_sum, running_total + next_term)
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
                    running_total.is_multiple_of(next_term)
                        &&
                        self.can_reach_sum_backward(&remaining_terms, initial_value, running_total / next_term)
                )
                ||
                (
                    running_total >= next_term
                    &&
                    self.can_reach_sum_backward(&remaining_terms, initial_value, running_total - next_term)
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

#[cfg(test)]
mod tests {
    use crate::Day;
    use crate::problems::day7::*;

    #[allow(unused)]
    const TEST_INPUT: &str = include_str!("day7_test.txt");

    #[test]
    fn day7_part1() {
        assert_eq!(Day7().part1(TEST_INPUT).unwrap(), 3749);
    }

    #[test]
    fn day7_part2() {
        assert_eq!(Day7().part2(TEST_INPUT).unwrap(), 11387);
    }
}