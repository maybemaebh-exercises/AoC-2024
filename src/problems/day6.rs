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
    let mut hash_set_for_treversal_loops = Vec::with_capacity(400);

    let guard_intial = grid.chars.iter().enumerate().find(|char| ['^','>','v','<'].contains(char.1)).unwrap();
    let mut guard_position = grid.vec_index_to_uquard(guard_intial.0);
    let initial_guard_position = guard_position;
    let mut guard_rotation = *guard_intial.1;

    loop {
        let next_guard_postion = get_next_guard(guard_position, guard_rotation, &grid);
        match next_guard_postion {
            None => break,
            Some(next_guard) => {
                if next_guard.1 == '#' {
                    guard_rotation = rotate_90_cw(guard_rotation)
                } else {
                    //find if an obstruction here whould cause a loop
                    let test_rotation = guard_rotation;
                    let test_position = guard_position;
                    let barrier_position = next_guard.0;
                    let barrier_char = grid.index_mut(barrier_position).unwrap();
                    if (!(barrier_position == initial_guard_position))&& barrier_char != &'O'{
                        let old_char = *barrier_char;
                        *barrier_char = '#';
                        if traversal_loops(test_position, test_rotation, &grid, &mut hash_set_for_treversal_loops) {
                            let barrier_char = grid.index_mut(barrier_position).unwrap();
                            *barrier_char = 'O';
                            running_count += 1;
                        } else {
                            let barrier_char = grid.index_mut(barrier_position).unwrap();
                            *barrier_char = old_char;
                        }
                    }
                    guard_position = next_guard.0;
                }
            }
        }
    }
    // assert_eq!(grid.chars.iter().filter(|x|x==&&'O').count(), running_count);
    // assert_eq!(grid.index(Uquard(grid.bounds[0]-1, grid.bounds[1]-1)), Some(grid.chars.last().unwrap()));

    grid.debug_print();
    running_count
}


fn traversal_loops(position: Uquard, rotation: char, char_grid: &CharGrid, hash_set: &mut Vec<(Uquard,char)>) -> bool {
    hash_set.clear();
    let mut current_position = position;
    // println!();
    // print!("initial:({rotation})");
    let mut current_rotation = rotate_90_cw(rotation);
    loop {
        match get_next_guard(current_position, current_rotation, char_grid) {
            None => { return false; },
            Some(next_guard) => {
                if next_guard.1 == '#' {
                    if !hash_set.contains(&(current_position, current_rotation)) {
                        return true;
                    } else{
                        hash_set.push((current_position,current_rotation))
                    }
                    current_rotation = rotate_90_cw(current_rotation);
                } else {
                    current_position = next_guard.0;
                }
            }
        }
    }
}

// #[allow(dead_code)]
// fn debug_print_traversal_loops(position: Uquard, rotation: char, char_grid: &mut CharGrid) -> bool {
//     let mut current_position = position;
//     // println!();
//     // print!("initial:({rotation})");
//     let mut current_rotation = rotate_90_cw(rotation);
//     loop {
//         let current_char = char_grid.index_mut(current_position).unwrap();
//         let old_char = *current_char;
//         *current_char = '%';
//         char_grid.debug_print();
//         let current_char = char_grid.index_mut(current_position).unwrap();
//         *current_char = old_char;
//
//         // print!("{current_rotation}{current_position:?}");
//         match get_next_guard_position(current_position, current_rotation) {
//             None => { return false; },
//             Some(next_guard_postion) => {
//                 match char_grid.index(next_guard_postion) {
//                     None => { return false; },
//                     Some(next_guard_char) => {
//                         if next_guard_char == &'#' {
//                             current_rotation = rotate_90_cw(current_rotation);
//                         } else {
//                             current_position = next_guard_postion;
//                         }
//                     }
//                 }
//             }
//         }
//         if current_position == position && current_rotation == rotation {
//             return true;
//         }
//     }
// }

fn rotate_90_cw(guard_rotation: char) -> char {
    match guard_rotation {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => unreachable!()
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