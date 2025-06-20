use std::cmp::Ordering;
use std::fmt::Debug;
use ahash::{HashMap};
use ahash::RandomState;

pub fn part1(input: &str) -> usize {
    let mut running_total = 0;
    let mut lines = input.lines();
    let mut rules:HashMap<[usize;2],bool> = HashMap::with_capacity_and_hasher(input.lines().count(), RandomState::default());

    loop{
        let line = lines.next().unwrap();
        if line.is_empty() {break}
        let mut  rule_values = line.split("|").map(|x| x.parse::<usize>().unwrap());
        let rule_values = [rule_values.next().unwrap(), rule_values.next().unwrap()];
        let compare =rule_values[1]>rule_values[0];
        rules.insert(match compare{
            true => {rule_values},
            false => {[rule_values[1],rule_values[0]]},
        }, !compare);
    }
    //println!("{:?}", rules);

    let mut page_list:Vec<PageNum> = Vec::with_capacity(50);
    for line in lines{
        page_list.clear();
        page_list.extend(line.split(",").map(|x| PageNum(x.parse::<usize>().unwrap(), &rules)));
        //let sorted_page_list = insertion_sort(&page_list);
        //println!("{:?} -> {:?}", page_list, check_valid(&page_list));
        //println!("{:?}",sorted_page_list);
        if check_valid(&page_list){
            running_total += page_list[page_list.len()/2].0;
        }

    }
    running_total
}

pub fn part2(input: &str) -> usize {
    let mut running_total = 0;
    let mut lines = input.lines();
    let mut rules:HashMap<[usize;2],bool> = HashMap::with_capacity_and_hasher(input.lines().count(), RandomState::default());

    loop{
        let line = lines.next().unwrap();
        if line.is_empty() {break}
        let mut  rule_values = line.split("|").map(|x| x.parse::<usize>().unwrap());
        let rule_values = [rule_values.next().unwrap(), rule_values.next().unwrap()];
        let compare =rule_values[1]>rule_values[0];
        rules.insert(match compare{
            true => {rule_values},
            false => {[rule_values[1],rule_values[0]]},
        }, !compare);
    }
    //println!("{:?}", rules);

    let mut page_list:Vec<PageNum> = Vec::with_capacity(50);
    for line in lines{
        page_list.clear();
        page_list.extend(line.split(",").map(|x| PageNum(x.parse::<usize>().unwrap(), &rules)));
        //let sorted_page_list = insertion_sort(&page_list);
        //println!("{:?} -> {:?}", page_list, sorted_page_list);
        //println!("{:?}",sorted_page_list);
        if !check_valid(&page_list){
            sort_with_partialord(&mut page_list);
            running_total += page_list[page_list.len()/2].0;
        }

    }
    running_total
}

fn check_valid(page_list:&Vec<PageNum>) -> bool {
    for page in page_list.iter().enumerate(){
        for compare_page in page_list.iter().enumerate(){
            //print!("{}<{}: {:?},",page.1.0, compare_page.1.0, page.1<compare_page.1);
            if
                (page.0>compare_page.0 && page.1<compare_page.1)
                || (page.0<compare_page.0 && page.1>compare_page.1)
            {return false;}
        }
    }
    true
}

//Slightly (50 micro-seconds) faster than algo off internet
#[allow(dead_code)]
pub fn sort_with_partialord<T: PartialOrd + Debug>(items: &mut [T]) { 
    //println!("start:");
    'next_value: for sorted_len in 1..items.len(){
        //println!("{:?}",items);
        for i in 0..sorted_len {
            //println!("{:?}>{:?}:{}",items[i], items[0],items[i] > items[sorted_len]);
            if items[i] > items[sorted_len] {
                //println!("{i}&{sorted_len} rotate:{:?}",items);
                items[i..=sorted_len].rotate_right(1);
                continue 'next_value;
            }
        }
    }
    //println!("{:?}",items);
}

//source:https://stackoverflow.com/questions/78588965/how-to-sort-a-vector-in-rust-that-only-has-partial-ordering
#[allow(dead_code)]
pub fn partialordsort<T: PartialOrd>(mut items: &mut [T]) {
    let mut presorted = 1;

    while items.len() > presorted {
        'make_start_min: loop {
            #[allow(clippy::mut_range_bound)]
            for i in presorted..items.len() {
                let ordering = T::partial_cmp(&items[0], &items[i]);
                if let Some(Ordering::Greater) = ordering {
                    items.swap(presorted, i);
                    items[0..presorted+1].rotate_right(1);
                    presorted += 1;
                    continue 'make_start_min;
                }
            }

            break;
        }

        presorted = usize::max(presorted - 1, 1);
        items = &mut items[1..];
    }
}

#[derive(Copy, Clone)]
struct PageNum<'a>(usize, & 'a HashMap<[usize;2],bool>);

impl<'a> Debug for PageNum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("page_num")
            .field(&self.0)
            .finish()
    }
}

impl<'a> PageNum<'a> {
    fn compare_bool(&self, other:&Self) -> Option<bool> {
        let compare = self.0>other.0;
        let stored_val = self.1.get(&match compare {true => { [other.0, self.0] }, false => {[self.0,other.0]}}).copied();
        match stored_val {
            None => None,
            Some(val) => {
                if compare {Some(!val)} else {Some(val)}
            },
        }

    }
}
impl PartialEq for PageNum<'_> {
    fn eq(&self, _other: &Self) -> bool {
        false//can't have multiple copies of pages
    }
}

impl PartialOrd for PageNum<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        //println!("{:?}<{:?}:<{:?}", self, other, self.compare_bool(other));
        match self.compare_bool(other) {
            None => None,
            Some(false) => {Some(Ordering::Less)},
            Some(true) => {Some(Ordering::Greater)},
        }
    }
}


#[allow(unused)]
const TEST_INPUT:&str =  include_str!("day5_test.txt");

#[cfg(test)]
mod tests {
    use crate::problems::day5::{part1, part2, TEST_INPUT, sort_with_partialord};

    #[test]
    fn day5_part1() {
        assert_eq!(part1(TEST_INPUT), 143);
    }

    #[test]
    fn day5_sort() {
        let mut list = vec![0,1,2,3,4,5,6,7,8,9,10];
        list.reverse();
        sort_with_partialord(&mut list);
        let mut trus_sort_list = list.clone();
        trus_sort_list.sort();
        assert_eq!(list, trus_sort_list);
    }

    #[test]
    fn day5_part2() {
        assert_eq!(part2(TEST_INPUT), 123)
    }
}