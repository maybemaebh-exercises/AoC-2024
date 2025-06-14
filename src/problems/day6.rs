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
    grid.debug_print();
    //assert_eq!(running_count, grid.chars.iter().filter(|c| **c == 'X').count());
    //println!("{:?}",grid.chars);
    running_count
}

pub fn part2(input: &str) -> usize {
    let mut grid = CharGrid::new(input);
    let mut running_count = 0;//starting position
    let mut current_position = grid.find_initial_guard_location();
    let mut current_rotation = Direction::DownwardsDownY;

    running_count
}

enum Direction{
    UpwardsPlusY,
    RightwardsPlusX,
    DownwardsDownY,
    LeftwardsDownY,
}

impl CharGrid {
    fn find_initial_guard_location(&self) -> Uquard {
        let index = self.chars.iter().enumerate().find(|x|*x.1 == '^').unwrap().0;
        self.vec_index_to_uquard(index)
    }
}

impl Direction {
    fn rotate_90cw(&mut self){
        *self = match self {
            Direction::UpwardsPlusY => Direction::RightwardsPlusX,
            Direction::RightwardsPlusX => Direction::DownwardsDownY,
            Direction::DownwardsDownY => Direction::LeftwardsDownY,
            Direction::LeftwardsDownY => Direction::UpwardsPlusY,
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