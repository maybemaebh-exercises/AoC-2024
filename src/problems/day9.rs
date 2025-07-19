use std::cmp::PartialEq;
use std::collections::VecDeque;
use crate::problems::commons::EnumeratedVecDeque;

pub struct Day9();
impl crate::Day for Day9 {
    fn part1(&self, input: &str) -> Option<usize> { Some(
        Part1PackedData::new(input)
        .flat_map(|block| (0..block.length).map(move |_|block.file_id))
        .enumerate()
        .map(|byte| byte.0 * byte.1)
        .sum()
    ) }

    fn part2(&self, input: &str) -> Option<usize> { Some(
        Part2PackedData::new(input)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .map(|block| match block {
                Block::Gap{length} => {(None,length)},
                Block::File{id,length} => {(Some(id),length as u16)}
            })
            .flat_map(|(id,length)| (0..length).map(move |_|id))
            .enumerate()
            .filter_map(|byte| Some(byte.0 * byte.1? as usize))
            .sum::<usize>()
    ) }

    fn full_input(&self) -> &'static str {
        include_str!("../../input/day9.txt")
    }

    fn problem_name(&self) -> &'static str {
        "Disk Fragmenter"
    }
}
struct Part1PackedData {
    input: EnumeratedVecDeque<u8>,
}
struct DataBlock {
    file_id: usize,
    length: u8,
}
impl Iterator for Part1PackedData {
    type Item = DataBlock;
    fn next(&mut self) -> Option<Self::Item> {
        let front = self.input.front()?;
        match front.0 % 2 {
            0 => {
                let block = DataBlock {
                    file_id:front.0/2,
                    length: *front.1
                };
                self.input.pop_front();
                Some(block)
            },
            1 => {
                let back = if self.input.back()?.0 % 2 == 0 {//back cannot = front
                    self.input.back()?
                } else {
                    self.input.pop_back().unwrap();
                    self.input.back()?
                };
                #[cfg(debug_assertions)]
                assert_eq!(self.input.back()?.0 % 2, 0);
                #[cfg(debug_assertions)]
                assert!(self.input.len() > 1);

                let front = self.input.front()?;

                Some(DataBlock {
                    file_id: back.0/2,
                    length: if back.1 < front.1 {
                        *self.input.front_mut()?.1 -=  *back.1;
                        let back = self.input.pop_back().unwrap();
                        back.1
                    } else if back.1 == front.1 {
                        self.input.pop_front().unwrap();
                        let back = self.input.pop_back().unwrap();
                        back.1
                    } else { //back.1 > front.1
                        *self.input.back_mut()?.1 -= *front.1;
                        let front = self.input.pop_front().unwrap();
                        front.1
                    }
                })
            },
            _ => unreachable!(),
        }
    }
}
impl Part1PackedData {
    fn new(input:&str) -> Part1PackedData {
        let input_as_nums = input
            .chars()
            .filter_map(|x|x.to_digit(10))
            .map(|x| x as u8);
        let mut vec = VecDeque::with_capacity(input.len());
        vec.extend(input_as_nums);
        if vec.len() % 2 == 0 {
            vec.pop_back().unwrap();//ensure back is always data at end of function calls
        }
        Part1PackedData {
            input: EnumeratedVecDeque::new(vec)
        }
    }
}

#[derive(Debug)]
enum Block {
    File{
        id: u16,
        length:u8
    },
    Gap{
        length:u16
    }
}
impl Block {
    #[allow(dead_code)]
    fn length(&self) -> u16 {
        match self {
            Block::Gap {length} => *length,
            Block::File {length, ..} => *length as u16
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum GapOccurrence {
    None,
    Invalid{last_valid_i:usize},
    Valid{i:usize}
}
struct Part2PackedData {
    data_layout: Vec<Block>,
    id_to_move:u16,
    first_gap_occurrence: [GapOccurrence;10]
    //offset to search always 0
}


impl Part2PackedData {
    fn new(input:&str) -> Part2PackedData {
        assert!((input.len() / 2) + 1 < u16::MAX as usize);
        let input_as_nums = input
            .chars()
            .filter_map(|x|x.to_digit(10))
            .map(|x| x as u8)
            .enumerate()
            .map(|(i,x)| if i % 2 == 0 { Block::File {
                id:(i/2) as u16,
                length: x
            }} else { Block::Gap{
                length: x as u16
            }});
        let mut data_layout = Vec::with_capacity(input.len());
        data_layout.extend(input_as_nums);

        let id_to_move:u16 = (data_layout.len()/2) as u16;
        Part2PackedData{
            data_layout,
            id_to_move,
            first_gap_occurrence: [GapOccurrence::Invalid {last_valid_i:0}; 10]
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for char in self.data_layout.iter()
            .map(|block| match block {
                Block::Gap{length} => {(None,*length)},
                Block::File{id,length} => {(Some(id),*length as u16)}
            })
            .flat_map(|(id,length)| (0..length).map(move |_|id))
            .map(|x| match x {Some(x) => (*x).to_string(), None => ".".to_string() }){
            print!("{char}");
        }
        println!();
    }

    fn attempt_to_move(&mut self) -> Option<()>{
        // println!("{:?}", self.first_gap_occurrence);
        // self.print();
        let length_to_move = *match self.data_layout.last().unwrap() {Block::File {length, ..} => length, _ => unreachable!()};
        let i = self.first_gap_occurrence(length_to_move)?;
        if i >= self.data_layout.len() {return None}
        let block = &self.data_layout[i];
        if let Block::Gap{length} = *block {
            assert!(length >= length_to_move as u16);
            // if length >= length_to_move as u16 {
            self.data_layout.swap_remove(i);// removes index to move, index to move - 1 new last

            let new_gap_length = match self.data_layout.last() {Some(Block::Gap {length}) => length, _ => &0} + (length_to_move as u16);
            if let Block::Gap{..} = self.data_layout.last().unwrap()  {
                self.data_layout.pop();
            }
            self.data_layout.push(Block::Gap {
                length: new_gap_length,
            });


            if length > length_to_move as u16 {
                self.data_layout.insert(i+1, Block::Gap {length: length - length_to_move as u16});
                self.shift_refs_from_insert(i+1);
            }
            self.set_invalid(length_to_move);
            Some(())
            // }
        } else {unreachable!()}
    }
    fn shift_refs_from_insert(&mut self, insert: usize) {
        self.first_gap_occurrence.iter_mut()
            .filter_map(|occurrence|
            match occurrence {
                GapOccurrence::Invalid{last_valid_i} => Some(last_valid_i),
                GapOccurrence::Valid {i} => Some(i),
                GapOccurrence::None => None,
            }
            )
            .filter(|i|**i>=insert)
            .for_each(|i|*i += 1);
    }
    fn first_gap_occurrence(&mut self, length_to_find:u8) -> Option<usize> {
        match self.first_gap_occurrence[length_to_find as usize] {
            GapOccurrence::Valid {i} => Some(i),
            GapOccurrence::None => None,
            GapOccurrence::Invalid{last_valid_i} => {
                let mut largest_gap_yet = 0;
                for i in last_valid_i..self.data_layout.len() {
                    if let Block::Gap{length} = &self.data_layout[i] {
                        if length > &largest_gap_yet {
                            self.first_gap_occurrence[(largest_gap_yet as usize) + 1..=(*length) as usize]
                                .iter_mut()
                                .for_each(|gap_occurrence|
                                    if let GapOccurrence::Invalid { .. } = *gap_occurrence {*gap_occurrence = GapOccurrence::Valid { i } }
                                );
                            if length >= &(length_to_find as u16) {
                                assert_eq!(self.first_gap_occurrence[length_to_find as usize], GapOccurrence::Valid{i});
                                return Some(i);
                            }
                            largest_gap_yet = *length;
                        }
                    }
                }
                self.first_gap_occurrence[(largest_gap_yet as usize) + 1..=9]
                    .iter_mut()
                    .for_each(|gap_occurrence| if let GapOccurrence::Invalid { .. } = *gap_occurrence { *gap_occurrence = GapOccurrence::None});
                None
            }
        }
    }
    fn set_invalid(&mut self, length_to_move:u8) {
        let old_gap = self.first_gap_occurrence[length_to_move as usize];
        let old_i = match old_gap {GapOccurrence::Valid {i} => i, _ => unreachable!()};
        self.first_gap_occurrence.iter_mut()
            .filter(|occurrence| **occurrence == old_gap)
            .for_each(|occurrence| *occurrence = GapOccurrence::Invalid { last_valid_i: old_i });
    }
}
impl Iterator for Part2PackedData {
    type Item = Block;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            //print_data(self.data_layout.as_slice());
            match self.data_layout.last()? {
                Block::Gap{..} => { return self.data_layout.pop() },
                Block::File{id, ..} => {
                    if *id == self.id_to_move {

                        let _ = self.attempt_to_move();
                        // offset_from_end_to_search = 0;
                        if self.id_to_move == 0 {
                            assert_eq!(self.data_layout.len(), 1);
                            return self.data_layout.pop();
                        } else {
                            self.id_to_move -= 1;
                        }
                    } else {// if not conditional doesn't account for
                        return self.data_layout.pop()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Day;
    use crate::problems::day9::*;

    #[allow(unused)]
    const TEST_INPUT: &str = include_str!("day9_test.txt");

    #[test]
    fn day9_part1() {
        assert_eq!(Day9().part1(TEST_INPUT), Some(1928));
    }

    #[test]
    fn day9_part2() {
        assert_eq!(Day9().part2(TEST_INPUT), Some(2858));
    }
}