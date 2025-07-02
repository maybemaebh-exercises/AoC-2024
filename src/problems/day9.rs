use std::collections::VecDeque;

pub fn part1(input: &str) -> usize {
    PackedData::new(input)
        .flat_map(|block| (0..block.length).map(move |_|block.file_id))
        .enumerate()
        .map(|byte| byte.0 * byte.1)
        .sum()

}
pub fn part2(_input: &str) -> usize {
    0
}
struct PackedData {
    input: VecDeque<(usize, u8)>,
}
struct DataBlock {
    file_id: usize,
    length: u8,
}
impl Iterator for PackedData {
    type Item = DataBlock;
    fn next(&mut self) -> Option<Self::Item> {
        let front = self.input.front()?;
        match front.0 % 2 {
            0 => {
                let block = DataBlock{
                    file_id:front.0/2,
                    length: front.1
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
                assert_eq!(self.input.back()?.0 % 2, 0);
                assert!(self.input.len() > 1);

                let front = self.input.front()?;

                Some(DataBlock {
                    file_id: back.0/2,
                    length:                 if back.1 < front.1 {
                        self.input.front_mut()?.1 -=  back.1;
                        let back = self.input.pop_back().unwrap();
                        back.1
                    } else if back.1 == front.1 {
                        self.input.pop_front().unwrap();
                        let back = self.input.pop_back().unwrap();
                        back.1
                    } else { //back.1 > front.1
                        self.input.back_mut()?.1 -= front.1;
                        let front = self.input.pop_front().unwrap();
                        front.1
                    }
                })


            },
            _ => unreachable!(),
        }
    }
}
impl PackedData {
    fn new(input:&str) -> PackedData {
        let input_as_nums = input
            .chars()
            .filter_map(|x|x.to_digit(10))
            .map(|x| x as u8)
            .enumerate();
        let mut vec = VecDeque::with_capacity(input.len());
        vec.extend(input_as_nums);
        if vec.len() % 2 == 0 {
            vec.pop_back().unwrap();//ensure back is always data at end of function calls
        }
        PackedData {
            input: vec
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day9::*;

    #[allow(unused)]
    const TEST_INPUT: &str = include_str!("day9_test.txt");

    #[test]
    fn day8_part1() {
        assert_eq!(part1(TEST_INPUT), 1928);
    }
    //
    // #[test]
    // fn day7_part2() {
    //     assert_eq!(part2(TEST_INPUT), 34);
    // }
}