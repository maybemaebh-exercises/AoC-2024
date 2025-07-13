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
#[allow(dead_code)]
fn print_data(data_layout: &[Block]) {
    for char in data_layout.iter()
        .map(|block| match block {
            Block::Gap{length} => {(None,*length)},
            Block::File{id,length} => {(Some(id),*length as u16)}
        })
        .flat_map(|(id,length)| (0u16..length).map(move |_|id))
        .map(|id| match id {None => ".".to_string(), Some(id) => id.to_string() }) {
        print!("{char}");
    }
    println!();
}
struct Part2PackedData {
    data_layout: Vec<Block>,
    id_to_move:u16,
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
            id_to_move
        }
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

                        let _ = attempt_to_move(&mut self.data_layout);
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
fn attempt_to_move(data_layout: &mut Vec<Block>) -> Option<()>{
    let length_to_move = *match data_layout.last().unwrap() {Block::File {length, ..} => length, _ => unreachable!()};
    for (i, block) in data_layout.iter().enumerate() {
        if let Block::Gap{length} = *block {
            if length >= length_to_move as u16 {
                data_layout.swap_remove(i);// removes index to move, index to move - 1 new last

                let new_gap_length = match data_layout.last() {Some(Block::Gap {length}) => length, _ => &0} + (length_to_move as u16);
                if let Block::Gap{..} = data_layout.last().unwrap()  {
                    data_layout.pop();
                }
                data_layout.push(Block::Gap {
                    length: new_gap_length,
                });


                if length > length_to_move as u16 {
                    data_layout.insert(i+1, Block::Gap {length: length - length_to_move as u16});
                }
                return Some(())
            }
        }
    }
    None
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