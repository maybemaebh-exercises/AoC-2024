use ahash::{HashSet, HashSetExt};
use crate::problems::commons::{CharGrid, Uquard};

pub fn part1(input: &str) -> usize {
    let mut grid = CharGrid::new(input);
    let mut running_count = 1;//starting position

    let mut guard_position = grid.find_initial_guard_location();
    let mut guard_rotation = Direction::UpwardsDownY;
    *grid.index_mut(guard_position).unwrap() = 'X';

    loop {
        let next_guard_postion = grid.next_guard(guard_position, guard_rotation);
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
    let mut grid = CharGrid::new(input);
    let mut running_count = 0;//starting position
    let mut current_position = grid.find_initial_guard_location();
    let initial_guard_position = current_position;
    let mut current_direction = Direction::UpwardsDownY;
    //atemt to estimate max length of loop turns
    let mut vec_for_loops_at = HashSet::with_capacity((grid.chars.len().pow(2) as f32 * 1.1033687572171522810942245886417e-6) as usize);

    loop{
        match grid.next_guard(current_position, current_direction) {
            None => {return running_count;},
            Some(next_guard) => {
                let currenct_char = grid.index_mut(current_position).unwrap();
                if currenct_char != &'O' {*currenct_char = 'V'}
                if next_guard.0 != current_position && next_guard.2 != 'V' && next_guard.2 != 'O'&& next_guard.0 != initial_guard_position {
                    //assert_eq!(next_guard.2,'.');
                    *grid.index_mut(next_guard.0).unwrap() = '#';
                    if grid.loops_at(current_position, current_direction, &grid, &mut vec_for_loops_at) {
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
    fn next_guard(&self, position: Uquard, direction: Direction) -> Option<(Uquard, Direction, char)> {
        let in_front_positon = match direction {
            Direction::UpwardsDownY => { position - Uquard(0, 1)},
            Direction::RightwardsUpX => { Some(position + Uquard(1, 0))},
            Direction::DownwardsUpY => { Some(position + Uquard(0, 1))},
            Direction::LeftwardsDownX => { position - Uquard(1, 0)}
        }?;
        match self.index(in_front_positon)? {
            '#' => Some((position, direction.rotate_90cw(), '#')),
            x => Some((in_front_positon, direction, *x))
        }
    }
    fn loops_at(&self, location: Uquard, direction: Direction, grid:&CharGrid, vec: &mut HashSet<(Uquard, Direction)>) -> bool {
        vec.clear();
        let mut current_location = location;
        let mut current_direction = direction;
        let previuse_turns = vec;
        loop {
            match grid.next_guard(current_location, current_direction) {
                None => {return false},
                Some(next_guard) => {
                    if next_guard.0 == current_location {
                        if previuse_turns.contains(&(current_location,current_direction)) {return true;}
                        previuse_turns.insert((current_location,current_direction));
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
    use crate::problems::day6::{part1, part2, TEST_INPUT};

    #[test]
    fn day6_part1() {
        assert_eq!(part1(TEST_INPUT), 41);
    }

    #[test]
    fn day6_part2() {
        assert_eq!(part2(TEST_INPUT), 6);
    }
}