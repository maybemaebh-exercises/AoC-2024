use ascii::{AsciiChar, AsciiStr};
use num_integer::gcd;
use tinyvec::TinyVec;
use crate::problems::commons::*;

pub fn part1(input: &str) -> usize {
    Part::Part1.calculate(input)
}
pub fn part2(input: &str) -> usize {
    Part::Part2.calculate(input)
}
enum Part{
    Part1,
    Part2
}
impl Part {
    fn calculate(&self, input: &str) -> usize{
        let antenna_grid = CharGrid::<&AsciiStr>::new(input);
        let mut antinode_grid:VecGrid<bool> = VecGrid::new(antenna_grid.bounds);
        let mut frequencies: TinyVec<[AsciiChar;64]> = TinyVec::new();

        for char in  antenna_grid.chars.into_iter().filter(|char| ![AsciiChar::Dot, AsciiChar::LineFeed, AsciiChar::CarriageReturn].contains(char)) {
            if !frequencies.contains(char) { frequencies.push(*char) }
        }

        frequencies
            .iter()
            .map(|frequency| self.add_antinodes_for_frequency(&antenna_grid, &mut antinode_grid, *frequency))
            .sum()
    }
    fn add_antinodes_for_frequency(
        &self,
        antenna_grid: & CharGrid<&AsciiStr>,
        antinode_grid: &mut VecGrid<bool>,
        frequency: AsciiChar,
    ) -> usize {
        let antenna_locations = antenna_grid.chars
            .into_iter()
            .enumerate()
            .filter(|char| *(*char).1 == frequency)
            .map(|char| antenna_grid.vec_index_to_uquard(char.0))
            .collect::<TinyVec<[Ucoord;128]>>();
        match self {
            Part::Part1 => {
                antenna_locations.iter()
                    .map(
                        |antenna_location|
                            antenna_locations.iter()
                                .filter(|second_location|  **second_location != *antenna_location)
                                .filter_map(|second_antenna_location| {
                                    (*antenna_location * 2) - *second_antenna_location
                                })
                                .filter_map(|antinode_coord|
                                    { let antinode = antinode_grid.index_mut(antinode_coord)?; if !(*antinode) {*antinode = true; Some(())} else {None}}
                                )
                                .count()
                    )
                    .sum()
            },
            Part::Part2 => {
                antenna_locations.iter()
                    .enumerate()
                    .map(
                        |antenna_location|
                            antenna_locations
                                .iter()
                                .enumerate()
                                .filter(|second_location|  second_location.0 < antenna_location.0)
                                .map(|second_antenna_location| {
                                    antinode_grid.set_positions_on_line(antenna_location.1.into(),second_antenna_location.1.into())
                                })
                                .sum::<usize>()
                    )
                    .sum()
            }
        }
    }
}
impl VecGrid<bool> {
    fn set_positions_on_line (
        &mut self,
        p1: Icoord,
        p2: Icoord,
    ) -> usize {
        let difference = p2 - p1;
        let minmum_step = difference / gcd(difference.0, difference.1);
        let mut current_position = p1;
        let mut running_total = 0;
        while let Some(ucoord) = current_position.into() {
            if let Some(antinode) = self.index_mut(ucoord) {
                if !(*antinode) {*antinode = true; running_total+=1;}
                current_position = current_position + minmum_step;
            } else {
                break
            }
        }
        current_position = p1;
        while let Some(ucoord) = current_position.into() {
            if let Some(antinode) = self.index_mut(ucoord) {
                if !(*antinode) {*antinode = true; running_total+=1;}
                current_position = current_position - minmum_step;
            } else {
                break
            }
        }
        running_total
    }
}
#[cfg(test)]
mod tests {
    use crate::problems::day8::*;

    #[allow(unused)]
    const TEST_INPUT: &str = include_str!("day8_test.txt");

    #[test]
    fn day8_part1() {
        assert_eq!(part1(TEST_INPUT), 14);
    }

    #[test]
    fn day7_part2() {
        assert_eq!(part2(TEST_INPUT), 34);
    }
}