use std::cmp::PartialEq;
use std::mem::swap;
use ahash::{HashSet, HashSetExt, RandomState};
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
        //grid.debug_print();
        let next_guard_postion = get_next_guard_position(guard_position, guard_rotation);
        //println!("{guard_rotation}->{next_guard_postion:?}");
        match next_guard_postion {
            None => break 'movemnet_loop,
            Some(next_guard_postion) => {
                if let Some(next_guard_char) = grid.index_mut(next_guard_postion) {
                    if next_guard_char == &'#' {
                        guard_rotation = match guard_rotation {
                            '^' => '>',
                            '>' => 'v',
                            'v' => '<',
                            '<' => '^',
                            _ => unreachable!()
                        }
                    } else {
                        guard_position = next_guard_postion;
                        if next_guard_char != &'X'{
                            *next_guard_char = 'X';
                            running_count += 1;
                        }
                    }
                } else { break 'movemnet_loop; }
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
    let mut hash_set_for_treversal_loops = HashSet::with_capacity(400);

    let guard_intial = grid.chars.iter().enumerate().find(|char| ['^','>','v','<'].contains(char.1)).unwrap();
    let mut guard_position = grid.vec_index_to_uquard(guard_intial.0);
    let initial_guard_position = guard_position;
    let mut guard_rotation = *guard_intial.1;
    let guard_index = guard_intial.0;
    grid.chars[guard_index] = 'X';

    'movemnet_loop:loop {
        //grid.debug_print();
        let next_guard_postion = get_next_guard_position(guard_position, guard_rotation);
        //println!("{guard_rotation}->{next_guard_postion:?}");
        match next_guard_postion {
            None => break 'movemnet_loop,
            Some(next_guard_position) => {
                if let Some(next_guard_char) = grid.index_mut(next_guard_position) {
                    if next_guard_char == &'#' {
                        guard_rotation = rotate_90_cw(guard_rotation)
                    } else {
                        //find if an obstruction here whould cause a loop
                        let test_rotation = guard_rotation;
                        let test_position = guard_position;
                        let barrier_char = grid.index_mut(next_guard_position).unwrap();
                        if !(next_guard_position == initial_guard_position) && barrier_char != &'O'{
                            let old_char = *barrier_char;
                            *barrier_char = '#';
                            if traversal_loops(test_position, test_rotation, &grid, &mut hash_set_for_treversal_loops) {
                                let barrier_char = grid.index_mut(next_guard_position).unwrap();
                                *barrier_char = 'O';
                                running_count += 1;
                            } else {
                                let barrier_char = grid.index_mut(next_guard_position).unwrap();
                                *barrier_char = old_char;
                            }
                        }
                        if !(next_guard_position == initial_guard_position) && traversal_loops(test_position, test_rotation, &grid, &mut hash_set_for_treversal_loops) {
                            let barrier_char = grid.index_mut(next_guard_position).unwrap();
                            if barrier_char != &'O'{
                                *barrier_char = 'O';
                                running_count += 1;
                            }
                        }
                        guard_position = next_guard_position;
                    }
                } else { break 'movemnet_loop; }
            }
        }
    }
    //assert_eq!(grid.chars.iter().filter(|x|x==&&'O').count(), running_count);
    grid.debug_print();
    running_count
}


fn traversal_loops(position: Uquard, rotation: char, char_grid: &CharGrid, hash_set: &mut HashSet<(Uquard,char)>) -> bool {
    hash_set.clear();
    let mut current_position = position;
    // println!();
    // print!("initial:({rotation})");
    let mut current_rotation = rotate_90_cw(rotation);
    loop {
        // print!("{current_rotation}{current_position:?}");
        match get_next_guard_position(current_position, current_rotation) {
            None => { return false; },
            Some(next_guard_postion) => {
                match char_grid.index(next_guard_postion) {
                    None => { return false; },
                    Some(next_guard_char) => {
                        if next_guard_char == &'#' {
                            current_rotation = rotate_90_cw(current_rotation);
                        } else {
                            hash_set.insert((current_position,current_rotation));
                            current_position = next_guard_postion;
                        }
                    }
                }
            }
        }
        if hash_set.contains(&(current_position,current_rotation)) {
            return true;
        }
    }
}
fn debug_print_traversal_loops(position: Uquard, rotation: char, char_grid: &mut CharGrid) -> bool {
    let mut current_position = position;
    // println!();
    // print!("initial:({rotation})");
    let mut current_rotation = rotate_90_cw(rotation);
    loop {
        let current_char = char_grid.index_mut(current_position).unwrap();
        let old_char = *current_char;
        *current_char = '%';
        char_grid.debug_print();
        let current_char = char_grid.index_mut(current_position).unwrap();
        *current_char = old_char;

        // print!("{current_rotation}{current_position:?}");
        match get_next_guard_position(current_position, current_rotation) {
            None => { return false; },
            Some(next_guard_postion) => {
                match char_grid.index(next_guard_postion) {
                    None => { return false; },
                    Some(next_guard_char) => {
                        if next_guard_char == &'#' {
                            current_rotation = rotate_90_cw(current_rotation);
                        } else {
                            current_position = next_guard_postion;
                        }
                    }
                }
            }
        }
        if current_position == position && current_rotation == rotation {
            return true;
        }
    }
}

fn rotate_90_cw(guard_rotation: char) -> char {
    match guard_rotation {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => unreachable!()
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