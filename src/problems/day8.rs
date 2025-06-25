use ascii::{AsciiChar, AsciiStr};
use tinyvec::TinyVec;
use crate::problems::commons::*;

pub fn part1(input: &str) -> usize {
    let antenna_grid = CharGrid::<&AsciiStr>::new(input);
    let mut antinode_grid:VecGrid<bool> = VecGrid::new(antenna_grid.bounds);
    let mut frequencies: TinyVec<[AsciiChar;64]> = TinyVec::new();

    for char in  antenna_grid.chars.into_iter().filter(|char| ![AsciiChar::Dot, AsciiChar::LineFeed, AsciiChar::CarriageReturn].contains(char)) {
        if !frequencies.contains(char) { frequencies.push(*char) }
    }

    frequencies
        .iter()
        .map(|frequency| antenna_grid.add_antinodes_for_frequency_part1(&mut antinode_grid, *frequency))
        .sum()
}
pub fn part2(input: &str) -> usize {
    0
}
impl CharGrid<&AsciiStr> {
    fn add_antinodes_for_frequency_part1(
        &self,
        antinode_grid: &mut VecGrid<bool>,
        frequency: AsciiChar,
    ) -> usize {
        let antenna_locations = self.chars
            .into_iter()
            .enumerate()
            .filter(|char| *(*char).1 == frequency)
            .map(|char| self.vec_index_to_uquard(char.0))
            .collect::<TinyVec<[Ucoord;128]>>();
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

    // #[test]
    // fn day7_part2() {
    //     assert_eq!(part2(TEST_INPUT), 11387);
    // }
}