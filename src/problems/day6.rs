use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;
use std::thread;
use std::thread::JoinHandle;
use ahash::{HashSet, HashSetExt};
use ascii::{AsciiChar, AsciiString};
use crate::problems::commons::{CharGrid, Ucoord};
use rayon::prelude::*;
use tinyvec::*;

pub fn part1(input: &str) -> usize {
    let mut grid = CharGrid::<AsciiString>::new(input);
    let mut running_count = 1;//starting position

    let mut guard_position = grid.find_initial_guard_location();
    let mut guard_rotation = Direction::UpwardsDownY;
    *grid.index_mut(guard_position).unwrap() = AsciiChar::X;

    loop {
        let next_guard_postion = grid.next_guard(guard_position, guard_rotation, None);
        match next_guard_postion {
            None => return running_count,
            Some(next_guard) => {
                guard_position = next_guard.0;
                guard_rotation = next_guard.1;
                if next_guard.2 == '.'{
                    *grid.index_mut(next_guard.0).unwrap() = AsciiChar::X;
                    running_count += 1;
                }
            }
        }
    }
}


pub fn part2(input: &str) -> usize {
    let mut hashset_for_loops_at:HashSet<(Ucoord, Direction)> = HashSet::with_capacity(200);
    let grid = CharGrid::<AsciiString>::new(input);
    let initial_guard_position = grid.find_initial_guard_location();
    //atemt to estimate max length of loop turns
    //let mut vec_for_loops_at = HashSet::with_capacity((grid.chars.len().pow(2) as f32 * 1.103_368_7e-6) as usize);
    //println!("{}",grid.chars.len());
    let mut iter = GuardPermutationsToCheckForLoopsIter::new(initial_guard_position, grid);
    // for permutation in iter {
    //     println!("{permutation:?}")
    // }
    let mut running_count = 0usize;
    loop {
        let next = iter.next_with_current_grid();
        match next.0 {
            None => return running_count,
            Some(next_guard) => {
                if next.1.loops_at(next_guard.0, next_guard.1, Some(CharGrid::in_front_postion(next_guard.1,next_guard.0).expect("has been given as a permutation")), &mut hashset_for_loops_at) {
                    running_count += 1;
                }
            }
        }
    }
}


#[allow(dead_code)]
pub fn part2_multithread_rayon(input: &str) -> usize {
    let _pool = rayon::ThreadPoolBuilder::new().build().unwrap();//adds 33% to time but is only fare
    thread_local! {
    static HASHSET_FOR_LOOPS_AT:RefCell<HashSet<(Ucoord, Direction)>> = RefCell::new(HashSet::with_capacity(400))
    }
    let grid = CharGrid::<AsciiString>::new(input);
    let initial_guard_position = grid.find_initial_guard_location();
    //atemt to estimate max length of loop turns
    //let mut vec_for_loops_at = HashSet::with_capacity((grid.chars.len().pow(2) as f32 * 1.103_368_7e-6) as usize);
    let iter = GuardPermutationsToCheckForLoopsIter::new(initial_guard_position,grid.clone());
    // for permutation in iter {
    //     println!("{permutation:?}")
    // }
    iter.par_bridge().into_par_iter().filter(|x|
        HASHSET_FOR_LOOPS_AT.with_borrow_mut(|hash_set|
            grid.loops_at(x.0, x.1,  Some(CharGrid::in_front_postion(x.1,x.0).expect("has been given as a permutation")), hash_set)
        )
    ).count()
}

pub fn part2_multithread(input: &str) -> usize {
    // thread_local! {
    // static HASHSET_FOR_LOOPS_AT:RefCell<HashSet<(Ucoord, Direction)>> = RefCell::new(HashSet::with_capacity(400))
    // }
    let grid = Arc::new(CharGrid::<AsciiString>::new(input));
    let initial_guard_position = grid.find_initial_guard_location();
    let running_count = Arc::new(AtomicUsize::new(0));
    //atemt to estimate max length of loop turns
    //let mut vec_for_loops_at = HashSet::with_capacity((grid.chars.len().pow(2) as f32 * 1.103_368_7e-6) as usize);
    let iter = Arc::new(Mutex::new(GuardPermutationsToCheckForLoopsIter::new(initial_guard_position, grid.as_ref().clone())));
    // for permutation in iter {
    //     println!("{permutation:?}")
    // }
    let mut threads:TinyVec<[Option<JoinHandle<_>>; 64]> = TinyVec::new();
    for _ in 0..std::thread::available_parallelism().unwrap().get() {
        threads.push(
            Some({let (grid, running_count, iter) = (grid.clone(), running_count.clone(), iter.clone());
                thread::spawn(move ||
                    {
                        let mut hash_set = HashSet::with_capacity(400);//not counted by alloc_track
                        loop {
                            let next = iter.lock().unwrap().next();
                            match next {
                                None => { return; }
                                Some(next_guard) => {
                                    if grid.loops_at(next_guard.0, next_guard.1, Some(CharGrid::in_front_postion(next_guard.1, next_guard.0).expect("has been given as a permutation")),&mut hash_set)
                                    { running_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
                                }
                            }
                        }
                    })})
        );
    }
    //let threads = threads.into_iter().map(|x|x.unwrap().join().unwrap());
    for thread in threads {
        thread.unwrap().join().unwrap();
    }
    //println!("Running threads: {:?}", threads);
    running_count.load(std::sync::atomic::Ordering::SeqCst)
}

struct GuardPermutationsToCheckForLoopsIter  {
    grid: CharGrid<AsciiString>,
    current_position: Ucoord,
    initial_guard_position: Ucoord,
    current_direction: Direction,
}
impl GuardPermutationsToCheckForLoopsIter{
    fn new(initial_guard_position: Ucoord, char_grid: CharGrid<AsciiString>) -> Self {
        GuardPermutationsToCheckForLoopsIter {
            grid: char_grid,
            current_position: initial_guard_position,
            initial_guard_position,
            current_direction: Direction::UpwardsDownY,
        }
    }
    fn next_with_current_grid(&mut self) -> (Option<(Ucoord, Direction)>, &CharGrid<AsciiString>) {
        (self.next(), &self.grid)
    }
}

impl Iterator for GuardPermutationsToCheckForLoopsIter {
    type Item = (Ucoord, Direction);//Uquard is postion of the Guard not barrirer!!
    fn next(&mut self) -> Option<Self::Item> {
        // let current_position = self.current_position;
        // println!("{current_position:?}");
        let next_guard =  self.grid.next_guard(self.current_position, self.current_direction, None)?;
        let currenct_char = self.grid.index_mut(self.current_position).unwrap();
        *currenct_char = AsciiChar::V;
        let (last_position,last_diretion) = (self.current_position, self.current_direction);
        (self.current_position, self.current_direction, _) = next_guard;
        // println!("{next_guard:?}");
        if next_guard.0 != last_position && next_guard.2 != AsciiChar::V && next_guard.0 != self.initial_guard_position {
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

impl CharGrid<AsciiString> {
    fn find_initial_guard_location(&self) -> Ucoord {
        let index = self.chars.into_iter().enumerate().find(|x|*x.1 == '^').unwrap().0;
        self.vec_index_to_uquard(index)
    }
    fn in_front_postion(direction: Direction, position: Ucoord) -> Option<Ucoord> {
        match direction {
            Direction::UpwardsDownY => { position - Ucoord(0, 1)},
            Direction::RightwardsUpX => { Some(position + Ucoord(1, 0))},
            Direction::DownwardsUpY => { Some(position + Ucoord(0, 1))},
            Direction::LeftwardsDownX => { position - Ucoord(1, 0)}
        }
    }
    fn next_guard(&self, position: Ucoord, direction: Direction, barrier: Option<Ucoord>) -> Option<(Ucoord, Direction, AsciiChar)> {
        let in_front_positon = Self::in_front_postion(direction,position)?;
        if Some(in_front_positon) == barrier {return Some((position, direction.rotate_90cw(), AsciiChar::Hash))};
        match self.index(in_front_positon)? {
            AsciiChar::Hash => Some((position, direction.rotate_90cw(), AsciiChar::Hash)),
            char => Some((in_front_positon, direction, *char))
        }
    }

    fn loops_at(&self, location: Ucoord, direction: Direction, barrier: Option<Ucoord>, previus_turns: &mut HashSet<(Ucoord, Direction)>) -> bool {
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
    use crate::problems::day6::*;

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

    #[test]
    fn day6_part2_multithread_rayon() {
        assert_eq!(part2_multithread_rayon(TEST_INPUT), 6);
    }
}