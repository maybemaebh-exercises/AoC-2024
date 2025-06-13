pub fn part1(input: &str) -> usize {
    //println!("{:?}",input.chars().filter(|x| !(x==&'\n'||x==&'\r')).collect::<Vec<_>>());
    let grid = CharGrid::new(input);
    let search_term = SearchTerm::new(['X','M','A','S']);
    let mut running_total:usize = 0;

    //horisontal
    for line in input.lines().map(|x|x.chars()) {
        running_total += search_term.check_line_occorences_count(line);
    }
    // println!("horizontal:{}", running_total);

    //vetical
    for x in 0..grid.bounds[0]{
        //println!("{}: {:?}", x,(0..grid.bounds[1]).map(|y| grid.index(x,y)).collect::<Vec<_>>());
        running_total += search_term.check_line_occorences_count(
            (0..grid.bounds[1]).map(|y| *grid.index(x, y).expect("within bounds"))
        )
    }
    // println!("vertical:{}", running_total);

    //diagonal down
    for y in 0i32..(grid.bounds[1]+grid.bounds[0]-1) as i32{
        // println!("{}: {:?}", y,(0..grid.bounds[0] as i32).map(|x|if y-x>=0{grid.index(x as usize,(y-x)as usize)}else { None }).collect::<Vec<_>>());
        // println!("{}: {:?}", y,(0..grid.bounds[0] as i32).map(|x|[x,y-x]).collect::<Vec<_>>());
        running_total += search_term.check_line_occorences_count(
            (0..grid.bounds[0] as i32).filter_map(|x|if y-x>=0{grid.index(x as usize,(y-x)as usize)}else { None }).copied() //check cloned performance
        );
    }

    // println!("diagonal down:{}", running_total);

    //diagonal up
    for y in 1-grid.bounds[0] as i32..grid.bounds[1] as i32{
        //println!("{}: {:?}", y,(0..grid.bounds[0] as i32).map(|x|if y+x>=0{grid.index(x as usize,(y+x)as usize)}else { None }).collect::<Vec<_>>());
        // println!("{}: {:?}", y,(0..grid.bounds[0] as i32).map(|x|[x,y-x]).collect::<Vec<_>>());
        running_total += search_term.check_line_occorences_count(
            (0..grid.bounds[0] as i32).filter_map(|x|if y+x>=0{grid.index(x as usize,(y+x)as usize)}else { None }).copied() //check cloned performance
        );
    }
    // println!("diagonal up:{}", running_total);

    running_total
}

struct CharGrid {
    chars: Vec<char>,
    bounds: [usize; 2]
}
impl CharGrid {
    fn index(&self, x: usize, y: usize) -> Option<&char> {
        self.chars.get(y*(self.bounds[0]) + x)//rember 2 char return character
    }

    fn new(input: &str) -> CharGrid {
        let mut chars: Vec<char> = Vec::with_capacity(input.len());
        chars.extend(input.chars().filter(|x| !(x==&'\n'||x==&'\r')));
        CharGrid {
            bounds: [input.lines().next().unwrap().chars().filter(|x| !(x==&'\n'||x==&'\r')).count(), input.lines().count()],
            chars
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SearchTerm<const N: usize>([char; N], usize);

impl<const N: usize> SearchTerm<{ N }> {
    fn new(chars: [char; N]) -> Self {
        SearchTerm(chars, chars.len() - 1)
    }
    fn check_line_occorences_count(&self, line: impl Iterator<Item=char>) -> usize {
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

struct OccorencesIterator<const N: usize, T: Iterator<Item=Option<char>>>{line:T, search_indexs:[usize;2], search_term: SearchTerm<N>, position:usize}

// impl<const N: usize, T: Iterator<Item=char>> OccorencesIterator<N, T>{
//     fn new(a: &dyn Iterator<Item=char>, b: SearchTerm<N>) -> Self {
//         OccorencesIterator(a,0,b)
//     }
// }

impl<const N: usize, T: Iterator<Item=Option<char>>> Iterator for OccorencesIterator<N, T>{
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
    let search_term = SearchTerm::new(['M','A','S']);
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
            line: (0..grid.bounds[0] as i32).map(|x| if y - x >= 0 {grid.index(x as usize, (y - x) as usize) } else { None }).map(|x| x.copied()), //check cloned performance
            search_indexs: [0, 0],
            search_term,
            position: 0
        }) {
            let (x,y) = (occorence, y as usize - occorence);
            //print!("Center:{},({},{}) ", grid.index(x as usize, y as usize).unwrap(),x,y);
            let top_right = grid.index(x+1, y+1);
            let bottom_left = grid.index(x-1, y-1);
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
    use crate::problems::day4::{part1, part2, TEST_INPUT, SearchTerm};

    #[test]
    fn day4_part1() {
        assert_eq!(part1(TEST_INPUT), 18);
    }

    #[test]
    fn day4_check_line() {
        assert_eq!(SearchTerm::new(['X','M','A','S']).check_line_occorences_count("XMASAMXBAXMAS".chars()), 3)
    }

    #[test]
    fn day4_part2() {
        assert_eq!(part2(TEST_INPUT), 9)
    }
}