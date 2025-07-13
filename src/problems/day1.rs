pub struct Day1();
impl crate::Day for Day1 {
    fn part1(&self, input: &str) -> Option<usize> {
        Some(part1(input))
    }

    fn part2(&self, input: &str) -> Option<usize> {
        Some(part2(input))
    }

    fn full_input(&self) -> &'static str {
        include_str!("../../input/day1.txt")
    }

    fn problem_name(&self) -> &'static str {
        "Historian Hysteria"
    }
}

fn part1 (input:&str) -> usize {
    let values = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());

    let line_count: usize = input.chars().filter(|x| *x == 0xA as char).count();
    //println!("line count:{}",line_count);
    let mut lists:[Vec<usize>;2] = [Vec::with_capacity(line_count/2+1),Vec::with_capacity(line_count/2+1)];
    //Lerning point: how to run innitialisation twice for both lists without repeating memory

    let mut cur_column = 0;
    for value in values {
        lists[cur_column].push(value);
        cur_column = match cur_column {
            0 => 1,
            _ => 0
        }
    }
    //println!("list 0:{:?}",lists[0]);

    //MARK:COMPUTE
    let lists = lists.map(|mut x| {
        x.sort_unstable();
        x
    });
    //println!("sorted list 0:{:?}",sorted_lists[0]);
    
    let mut result_val:usize = 0;
    for i in 0..line_count {
        //println!("i:{},val1:{:?},val2:{:?},diff:{:?}",i,lists[0][i],lists[1][i],lists[0][i].abs_diff(lists[1][i]));
        result_val += lists[0][i].abs_diff(lists[1][i]);
    }
    result_val
}

fn part2 (input:&str) -> usize {
        let values = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());

    let line_count: usize = input.chars().filter(|x| *x == 0xA as char).count();
    //println!("line count:{}",line_count);
    let mut lists:[Vec<usize>;2] = [Vec::with_capacity(line_count/2+1),Vec::with_capacity(line_count/2+1)];
    //Lerning point: how to run innitialisation twice for both lists without repeating memory

    let mut cur_column = 0;
    for value in values {
        lists[cur_column].push(value);
        cur_column = match cur_column {
            0 => 1,
            _ => 0
        }
    }
    //println!("list 0:{:?}",lists[0]);

    //MARK:COMPUTE
    let lists = lists.map(|mut x| {
        x.sort_unstable();
        x
    });
    
    let mut current_count:usize = 0;
    let [first_list, second_list] = lists;
    let mut second_list = second_list.into_iter();
    
    let mut simularity_score:usize = 0;
    let mut comp_val = second_list.next();
    let mut last_val:usize = first_list[0]+1;
    for value in first_list{
        if value != last_val{
            current_count = 0;
            last_val = value;
            while match comp_val {Some(c) => c <= value, None => false} {
                if comp_val.expect("checked earlier") == value { current_count += 1; }
                comp_val = second_list.next();
            }
        }
        simularity_score += current_count * value;
    }
    simularity_score
}

#[allow(unused)]
const TEST_INPUT:&str =  include_str!("day1_test.txt");

#[cfg(test)]
mod tests {
    use crate::problems::day1::{part1, part2, TEST_INPUT};

    #[test]
    fn day1_part1() {
        assert_eq!(part1(TEST_INPUT), 11);
    }
    
    #[test]
    fn day1_part2() {
        assert_eq!(part2(TEST_INPUT), 31)
    }
}