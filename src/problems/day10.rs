use crate::problems::commons::{Ucoord, VecGrid};
use crate::Day;
use tinyvec::TinyVec;

pub struct Day10();

impl Day for Day10 {
    fn part1(&self, input: &str) -> Option<usize> {
        let grid = VecGrid::from_iter(
            input.lines().next().unwrap().len(),
            input.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap() as u8),
            input.len()
        );
        let mut bool_vec = VecGrid::<bool>::new(grid.bounds);
        Some(grid.vec.iter()
            .enumerate()
            .filter(|num| *num.1== 0)
            .map(|num| grid.find_trailhead_score(&mut bool_vec, grid.vec_index_to_uquard(num.0)))
            .sum())
    }
    fn full_input(&self) -> &'static str {
        include_str!("../../input/day10.txt")
    }

    fn problem_name(&self) -> &'static str {
        "Hoof It"
    }
}
struct TrailheadScoreFinder<'a>(& 'a VecGrid<u8>,&'a mut VecGrid<bool>);
impl TrailheadScoreFinder<'_> {
    pub fn find_trailhead_score(&mut self, position:Ucoord) -> usize
    {
        let height = *self.0.index(position).unwrap();
        [
            position - Ucoord(1,0),
            position - Ucoord(0,1),
            Some(position + Ucoord(0,1)),
            Some(position + Ucoord(1,0)),
        ].into_iter()
            .filter_map(|neighboring_position|
                Some((neighboring_position?,*self.0.index(neighboring_position?)?))
            )
            .filter(|neighboring_position| neighboring_position.1 == height + 1)
            .filter(|neighboring_position| {
                let visited = self.1.index_mut(neighboring_position.0).unwrap();
                if !*visited {*visited = true; true}
                else {false}
            })
            .collect::<TinyVec<[(Ucoord, u8); 4]>>()
            .into_iter().map(|neighboring_position| {
            if neighboring_position.1 == 9 {1}
            else { self.find_trailhead_score(neighboring_position.0) }
        }).sum::<usize>()
    }
}
impl VecGrid<u8> {
    fn find_trailhead_score(&self, bool_vec: &mut VecGrid<bool>, position:Ucoord) -> usize {
        bool_vec.vec.fill(false);
        TrailheadScoreFinder(self, bool_vec).find_trailhead_score(position)
    }
}
#[cfg(test)]
mod tests {
    use crate::problems::day10::*;
    use crate::Day;

    #[allow(unused)]
    const TEST_INPUT: &str = include_str    !("day10_test.txt");

    #[test]
    fn day10_part1() {
        assert_eq!(Day10().part1(TEST_INPUT), Some(36));
    }

    // #[test]
    // fn day9_part2() {
    //     assert_eq!(Day9().part2(TEST_INPUT), Some(2858));
    //     assert_eq!(Day9().part2(Day9().full_input()), Some(6423258376982));
    // }
}