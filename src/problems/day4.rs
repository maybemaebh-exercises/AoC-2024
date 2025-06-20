use ascii::AsciiChar;
use crate::problems::commons::{CharGrid, Ucoord};

pub fn part1(input: &str) -> usize {
    //println!("{:?}",input.chars().filter(|x| !(x==&'\n'||x==&'\r')).collect::<Vec<_>>());
    let grid = CharGrid::new(input);
    let search_term = SearchTerm::new([AsciiChar::X,AsciiChar::M,AsciiChar::A,AsciiChar::S]);
    let mut running_total:usize = 0;

    //horisontal
    for y in 0..grid.bounds[1] {
        running_total += search_term.check_line_occorences_count(
            (0..grid.bounds[0]).map(|x| *grid.index(Ucoord(x, y)).expect("within bounds"))
        );
    }
    // println!("horizontal:{}", running_total);

    //vetical
    for x in 0..grid.bounds[0]{
        //println!("{}: {:?}", x,(0..grid.bounds[1]).map(|y| grid.index(x,y)).collect::<Vec<_>>());
        //println!("{:?}",(0..grid.bounds[1]).map(|y| *grid.index(Ucoord(x, y)).expect("within bounds")).collect::<Vec<_>>() );
        running_total += search_term.check_line_occorences_count(
            (0..grid.bounds[1]).map(|y| *grid.index(Ucoord(x, y)).expect("within bounds"))
        )
    }
    // println!("vertical:{}", running_total);

    //diagonal down
    for y in 0i32..(grid.bounds[1]+grid.bounds[0]-1) as i32{
        // println!("{}: {:?}", y,(0..grid.bounds[0] as i32).map(|x|if y-x>=0{grid.index(x as usize,(y-x)as usize)}else { None }).collect::<Vec<_>>());
        // println!("{}: {:?}", y,(0..grid.bounds[0] as i32).map(|x|[x,y-x]).collect::<Vec<_>>());
        //println!("{:?}",(0..grid.bounds[0] as i32).filter_map(|x|if y-x>=0{grid.index(Ucoord(x as usize, (y-x)as usize))}else { None }).copied().collect::<Vec<_>>() );
        running_total += search_term.check_line_occorences_count(
            (0..grid.bounds[0] as i32).filter_map(|x|if y-x>=0{grid.index(Ucoord(x as usize, (y-x)as usize))}else { None }).copied() //check cloned performance
        );
    }

    // println!("diagonal down:{}", running_total);

    //diagonal up
    for y in 1-grid.bounds[0] as i32..grid.bounds[1] as i32{
        //println!("{}: {:?}", y,(0..grid.bounds[0] as i32).map(|x|if y+x>=0{grid.index(x as usize,(y+x)as usize)}else { None }).collect::<Vec<_>>());
        // println!("{}: {:?}", y,(0..grid.bounds[0] as i32).map(|x|[x,y-x]).collect::<Vec<_>>());
        //println!("{:?}", (0..grid.bounds[0] as i32).filter_map(|x|if y+x>=0{grid.index(Ucoord(x as usize, (y+x)as usize))}else { None }).copied().collect::<Vec<_>>());
        running_total += search_term.check_line_occorences_count(
            (0..grid.bounds[0] as i32).filter_map(|x|if y+x>=0{grid.index(Ucoord(x as usize, (y+x)as usize))}else { None }).copied() //check cloned performance
        );
    }
    // println!("diagonal up:{}", running_total);

    running_total
}

#[derive(Debug, Clone, Copy)]
struct SearchTerm<const N: usize>([AsciiChar; N], usize);

impl<const N: usize> SearchTerm<{ N }> {
    fn new(chars: [AsciiChar; N]) -> Self {
        SearchTerm(chars, chars.len() - 1)
    }
    fn check_line_occorences_count(&self, line: impl Iterator<Item=AsciiChar>) -> usize {
        let mut running_total:usize = 0;
        let mut forward_index:usize = 0;
        let mut backward_index:usize = 0;
        //println!("");
        for char in line {
            //print!("{} ", char);
            if char == self.0[forward_index] {
                forward_index += 1;
                if forward_index > self.1 {forward_index = 0; running_total += 1;}
            } else if char == self.0[0] { forward_index = 1; } else { forward_index = 0;}
            if char == self.0[self.1-backward_index] {
                backward_index += 1;
                if backward_index > self.1 {backward_index = 0; running_total += 1;}
            } else if char == self.0[self.1] { backward_index = 1; } else { backward_index = 0; }
        }
        running_total
    }
}

struct OccorencesIterator<const N: usize, T: Iterator<Item=Option<AsciiChar>>>{line:T, search_indexs:[usize;2], search_term: SearchTerm<N>, position:usize}

// impl<const N: usize, T: Iterator<Item=char>> OccorencesIterator<N, T>{
//     fn new(a: &dyn Iterator<Item=char>, b: SearchTerm<N>) -> Self {
//         OccorencesIterator(a,0,b)
//     }
// }

impl<const N: usize, T: Iterator<Item=Option<AsciiChar>>> Iterator for OccorencesIterator<N, T>{
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        // println!("new line:");
        loop {
            match self.line.next(){
                None => { return None; },
                Some(char) => {
                    self.position += 1;
                    match char {
                        None => continue,
                        Some(char) => {
                            // print!("{},{},{},", self.position,self.search_indexs[0],self.search_indexs[1]);
                            // print!("{} ", char);
                            let mut is_middle = false;
                            if char == self.search_term.0[self.search_indexs[0]] {
                                self.search_indexs[0] += 1;
                                if self.search_indexs[0] > self.search_term.1 {self.search_indexs[0] = 0; is_middle = true;}
                            } else if char == self.search_term.0[0] { self.search_indexs[0] = 1; } else { self.search_indexs[0] = 0;}
                            if char == self.search_term.0[self.search_term.1-self.search_indexs[1]] {
                                self.search_indexs[1] += 1;
                                if self.search_indexs[1] > self.search_term.1 {self.search_indexs[1] = 0; is_middle = true;}
                            } else if char == self.search_term.0[self.search_term.1] { self.search_indexs[1] = 1; } else { self.search_indexs[1] = 0;
                            }
                            if is_middle {return Some(self.position-2)}
                    } }
                }
            }
        }
    }
}


pub fn part2(input: &str) -> usize {
    //println!("part 2:");
    let grid = CharGrid::new(input);
    let search_term = SearchTerm::new([AsciiChar::M,AsciiChar::A,AsciiChar::S]);
    let mut running_total = 0;

    // for y in 0..grid.bounds[1] {
    //     for x in 0..grid.bounds[0] {
    //         //print!("{}({},{}) ", grid.index(x, y).expect("within bounds"),x,y);
    //     }
    //     //println!("");
    // }

    //diagonal down
    for y in 0i32..(grid.bounds[1]+grid.bounds[0]-1) as i32{
        // println!("{}: {:?}", y,(0..grid.bounds[0] as i32).map(|x|if y-x>=0{grid.index(x as usize,(y-x)as usize)}else { None }).collect::<Vec<_>>());
        // println!("{}: {:?}", y,(0..grid.bounds[0] as i32).map(|x|[x,y-x]).collect::<Vec<_>>());

        // assert_eq!(OccorencesIterator {
        //     line: (0..grid.bounds[0] as i32).map(|x| if y - x >= 0 {grid.index(x as usize, (y - x) as usize) } else { None }).map(|x| match x {None => None, Some(x) => Some(*x)}), //check cloned performance
        //     search_indexs: [0, 0],
        //     search_term,
        //     position: 0
        // }.count(), search_term.check_line_occorences_count(
        //     (0..grid.bounds[0] as i32).filter_map(|x|if y-x>=0{grid.index(x as usize,(y-x)as usize)}else { None }).copied() //check cloned performance
        // ));

        for occorence in (OccorencesIterator {
            line: (0..grid.bounds[0] as i32).map(|x| if y - x >= 0 {grid.index(Ucoord(x as usize, (y - x) as usize)) } else { None }).map(|x| x.copied()), //check cloned performance
            search_indexs: [0, 0],
            search_term,
            position: 0
        }) {
            let (x,y) = (occorence, y as usize - occorence);
            //print!("Center:{},({},{}) ", grid.index(x as usize, y as usize).unwrap(),x,y);
            let top_right = grid.index(Ucoord(x+1, y+1));
            let bottom_left = grid.index(Ucoord(x-1, y-1));
            if (top_right == Some(&search_term.0[0]) && bottom_left == Some(&search_term.0[2])) || (top_right == Some(&search_term.0[2]) && bottom_left == Some(&search_term.0[0])) {
                //println!(",Passes");
                running_total += 1;
            }
        }
    }


    running_total
}

#[allow(unused)]
const TEST_INPUT:&str =  include_str!("day4_test.txt");

#[cfg(test)]
mod tests {
    use ascii::AsAsciiStr;
    use crate::problems::day4::*;

    #[test]
    fn day4_part1() {
        assert_eq!(part1(TEST_INPUT), 18);
    }

    #[test]
    fn day4_check_line() {
        assert_eq!(SearchTerm::new([AsciiChar::X,AsciiChar::M,AsciiChar::A,AsciiChar::S]).check_line_occorences_count("XMASAMXBAXMAS".as_ascii_str().unwrap().chars()), 3)
    }

    #[test]
    fn day4_index_test() {
        let grid = CharGrid::new(TEST_INPUT);
        assert_eq!(grid.bounds, [10,10]);
        assert_eq!(grid.newline_lengh, 2);
        assert_eq!(grid.index(Ucoord(0,0)), Some(&AsciiChar::M));
        assert_eq!(grid.index(Ucoord(9,9)), Some(&AsciiChar::X));
        assert!(grid.index(Ucoord(10,0)).is_none());
        assert!(grid.index(Ucoord(10,9)).is_none());
        assert!(grid.index(Ucoord(10,10)).is_none());
    }

    #[test]
    fn day4_part2() {
        assert_eq!(part2(TEST_INPUT), 9)
    }
}