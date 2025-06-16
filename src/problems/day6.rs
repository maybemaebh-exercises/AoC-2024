use std::cell::RefCell;
use std::ptr::hash;
use ahash::{HashSet, HashSetExt};
use crate::problems::commons::{CharGrid, Uquard};
use rayon::prelude::*;

pub fn part1(input: &str) -> usize {
    let mut grid = CharGrid::new(input);
    let mut running_count = 1;//starting position

    let mut guard_position = grid.find_initial_guard_location();
    let mut guard_rotation = Direction::UpwardsDownY;
    *grid.index_mut(guard_position).unwrap() = 'X';

    loop {
        let next_guard_postion = grid.next_guard(guard_position, guard_rotation, None);
        match next_guard_postion {
            None => return running_count,
            Some(next_guard) => {
                guard_position = next_guard.0;
                guard_rotation = next_guard.1;
                if next_guard.2 == '.'{
                    *grid.index_mut(next_guard.0).unwrap() = 'X';
                    running_count += 1;
                }
            }
        }
    }
}


pub fn part2(input: &str) -> usize {
    let mut hashset_for_loops_at:HashSet<(Uquard, Direction)> = HashSet::with_capacity(400);
    let mut grid = CharGrid::new(input);
    let mut running_count = 0;//starting position
    let mut current_position = grid.find_initial_guard_location();
    let initial_guard_position = current_position;
    let mut current_direction = Direction::UpwardsDownY;
    //atemt to estimate max length of loop turns

    loop{
        match grid.next_guard(current_position, current_direction, None) {
            None => {return running_count;},
            Some(next_guard) => {
                let currenct_char = grid.index_mut(current_position).unwrap();
                if currenct_char != &'O' {*currenct_char = 'V'}
                if next_guard.0 != current_position && next_guard.2 != 'V' && next_guard.2 != 'O'&& next_guard.0 != initial_guard_position {
                    //assert_eq!(next_guard.2,'.');
                    *grid.index_mut(next_guard.0).unwrap() = '#';
                    if grid.loops_at(current_position, current_direction, None, &mut hashset_for_loops_at) {
                        *grid.index_mut(next_guard.0).unwrap() = 'O';
                        running_count += 1;
                    } else {
                        *grid.index_mut(next_guard.0).unwrap() = '.';
                    }
                }
                current_position = next_guard.0;
                current_direction = next_guard.1;
            }
        }
    }
}



pub fn part2_multithread(input: &str) -> usize {
    thread_local! {
    static HASHSET_FOR_LOOPS_AT:RefCell<HashSet<(Uquard, Direction)>> = RefCell::new(HashSet::with_capacity(400))
    }
    let grid = CharGrid::new(input);
    let initial_guard_position = grid.find_initial_guard_location();
    //atemt to estimate max length of loop turns
    //let mut vec_for_loops_at = HashSet::with_capacity((grid.chars.len().pow(2) as f32 * 1.103_368_7e-6) as usize);
    let iter = GuardPermutationsToCheckForLoopsIter::new(initial_guard_position,&grid);
    // for permutation in iter {
    //     println!("{permutation:?}")
    // }
    iter.par_bridge().into_par_iter().filter(|x|
        HASHSET_FOR_LOOPS_AT.with_borrow_mut(|hash_set|
            grid.loops_at(x.0, x.1,  Some(CharGrid::in_front_postion(x.1,x.0).expect("has been given as a permutation")), hash_set)
        )
    ).count()
}


struct GuardPermutationsToCheckForLoopsIter  {
    grid: CharGrid,
    current_position: Uquard,
    initial_guard_position: Uquard,
    current_direction: Direction,
}
impl GuardPermutationsToCheckForLoopsIter{
    fn new(initial_guard_position:Uquard, char_grid: &CharGrid) -> Self {
        GuardPermutationsToCheckForLoopsIter {
            grid: char_grid.clone(),
            current_position: initial_guard_position,
            initial_guard_position,
            current_direction: Direction::UpwardsDownY,
        }
    }
}

impl Iterator for GuardPermutationsToCheckForLoopsIter {
    type Item = (Uquard, Direction);//Uquard is postion of the Guard not barrirer!!
    fn next(&mut self) -> Option<Self::Item> {
        // let current_position = self.current_position;
        // println!("{current_position:?}");
        let next_guard =  self.grid.next_guard(self.current_position, self.current_direction, None)?;
        let currenct_char = self.grid.index_mut(self.current_position).unwrap();
        *currenct_char = 'V';
        let (last_position,last_diretion) = (self.current_position, self.current_direction);
        (self.current_position, self.current_direction, _) = next_guard;
        // println!("{next_guard:?}");
        if next_guard.0 != last_position && next_guard.2 != 'V' && next_guard.0 != self.initial_guard_position {

            return Some((last_position, last_diretion));
        }
        self.next()
    }
}

#[derive(Debug,Clone,Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction{
    UpwardsDownY,
    RightwardsUpX,
    DownwardsUpY,
    LeftwardsDownX,
}

impl Direction {
    fn rotate_90cw(&self) -> Self{
        match self {
            Direction::UpwardsDownY => Direction::RightwardsUpX,
            Direction::RightwardsUpX => Direction::DownwardsUpY,
            Direction::DownwardsUpY => Direction::LeftwardsDownX,
            Direction::LeftwardsDownX => Direction::UpwardsDownY,
        }
    }
}

impl CharGrid {
    fn find_initial_guard_location(&self) -> Uquard {
        let index = self.chars.iter().enumerate().find(|x|*x.1 == '^').unwrap().0;
        self.vec_index_to_uquard(index)
    }
    fn in_front_postion(direction: Direction, position:Uquard) -> Option<Uquard> {
        match direction {
            Direction::UpwardsDownY => { position - Uquard(0, 1)},
            Direction::RightwardsUpX => { Some(position + Uquard(1, 0))},
            Direction::DownwardsUpY => { Some(position + Uquard(0, 1))},
            Direction::LeftwardsDownX => { position - Uquard(1, 0)}
        }
    }
    fn next_guard(&self, position: Uquard, direction: Direction, barrier: Option<Uquard>) -> Option<(Uquard, Direction, char)> {
        let in_front_positon = Self::in_front_postion(direction,position)?;
        if Some(in_front_positon) == barrier {return Some((position, direction.rotate_90cw(), '#'))};
        match self.index(in_front_positon)? {
            '#' => Some((position, direction.rotate_90cw(), '#')),
            x => Some((in_front_positon, direction, *x))
        }
    }

    fn loops_at(&self, location: Uquard, direction: Direction, barrier: Option<Uquard>, previus_turns: &mut HashSet<(Uquard, Direction)>) -> bool {
        previus_turns.clear();
        let mut current_location = location;
        let mut current_direction = direction;
        loop {
            match self.next_guard(current_location, current_direction, barrier) {
                None => {return false},
                Some(next_guard) => {
                    if next_guard.0 == current_location {
                        if previus_turns.contains(&(current_location,current_direction)) {return true;}
                        previus_turns.insert((current_location,current_direction));
                        current_direction = next_guard.1;
                    } else { current_location = next_guard.0; }
                }
            }
        }
    }
}

#[allow(unused)]
const TEST_INPUT: &str = include_str!("day6_test.txt");
#[cfg(test)]
mod tests {
    use crate::problems::day6::{part1, part2, part2_multithread, TEST_INPUT};

    #[test]
    fn day6_part1() {
        assert_eq!(part1(TEST_INPUT), 41);
    }

    #[test]
    fn day6_part2() {
        assert_eq!(part2(TEST_INPUT), 6);
    }

    #[test]
    fn day6_part2_multithread() {
        assert_eq!(part2_multithread(TEST_INPUT), 6);
    }
}