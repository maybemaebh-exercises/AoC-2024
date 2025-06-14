use ahash::{HashSet, HashSetExt};
use crate::problems::commons::{CharGrid, Uquard};

pub fn part1(input: &str) -> usize {
    let mut grid = CharGrid::new(input);
    let mut running_count = 1;//starting position

    let guard_intial = grid.chars.iter().enumerate().find(|char| ['^','>','v','<'].contains(char.1)).unwrap();
    let mut guard_position = grid.vec_index_to_uquard(guard_intial.0);
    let mut guard_rotation = *guard_intial.1;
    let guard_index = guard_intial.0;
    grid.chars[guard_index] = 'X';

    'movemnet_loop:loop {
        let next_guard_postion = get_next_guard_position_mut(guard_position, guard_rotation, &mut grid);
        match next_guard_postion {
            None => break 'movemnet_loop,
            Some(next_guard) => {
                if next_guard.1 == &'#' {
                    guard_rotation = match guard_rotation {
                        '^' => '>',
                        '>' => 'v',
                        'v' => '<',
                        '<' => '^',
                        _ => unreachable!()
                    }
                } else {
                    guard_position = next_guard.0;
                    if next_guard.1 != &'X'{
                        *next_guard.1 = 'X';
                        running_count += 1;
                    }
                }
            }
        }
    }
    //grid.debug_print();
    //assert_eq!(running_count, grid.chars.iter().filter(|c| **c == 'X').count());
    //println!("{:?}",grid.chars);
    running_count
}

pub fn part2(input: &str) -> usize {
    let mut grid = CharGrid::new(input);
    let mut running_count = 0;//starting position
    let mut current_position = grid.find_initial_guard_location();
    let initial_guard_position = current_position;
    let mut current_direction = Direction::UpwardsDownY;
    let mut vec_for_loops_at = Vec::with_capacity(40);

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

#[derive(Debug,Clone,Copy)]
#[derive(PartialEq)]
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
    fn loops_at(&self, location: Uquard, direction: Direction, grid:&CharGrid, vec: &mut Vec<(Uquard, Direction)>) -> bool {
        vec.clear();
        let mut current_location = location;
        let mut current_direction = direction;
        let mut previuse_turns = vec;
        loop {
            match grid.next_guard(current_location, current_direction) {
                None => {return false},
                Some(next_guard) => {
                    if next_guard.0 == current_location {
                        if previuse_turns.contains(&(current_location,current_direction)) {return true;}
                        previuse_turns.push((current_location,current_direction));
                        current_direction = next_guard.1;
                    } else { current_location = next_guard.0; }
                }
            }
        }
    }
}

fn get_next_guard(guard_position: Uquard, guard_rotation: char, char_grid: &CharGrid) -> Option<(Uquard, char)> {
    let next_position = get_next_guard_position(guard_position, guard_rotation);
    match next_position {
        None => None,
        Some(next_position) => {
            (char_grid.index(next_position)).map(|char| (next_position, *char))
        }
    }
}
fn get_next_guard_position_mut(guard_position: Uquard, guard_rotation: char, char_grid: &mut CharGrid) -> Option<(Uquard, &mut char)> {
    let next_position = get_next_guard_position(guard_position, guard_rotation);
    match next_position {
        None => None,
        Some(next_position) => {
            (char_grid.index_mut(next_position)).map(|char| (next_position, char))
        }
    }
}

fn get_next_guard_position(guard_position: Uquard, guard_rotation: char) -> Option<Uquard> {
    match guard_rotation {
        '^' => guard_position - Uquard(0, 1),
        '>' => Some(guard_position + Uquard(1, 0)),
        'v' => Some(guard_position + Uquard(0, 1)),
        '<' => guard_position - Uquard(1, 0),
        _ => { unreachable!() },
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