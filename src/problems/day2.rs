#[derive(Clone, Copy, PartialEq, Eq)]
enum CheckState {
    Increasing,
    Decreasing,
    NotYetCompared
}

pub fn part1(input:&str) -> usize {
    let reports = input.split(0xA as char);
    let mut safe_reports:usize = 0;
    for report in reports{
        let report = report.split_whitespace().map(|x|x.parse::<u32>().unwrap()).into_iter();
        let fail_count = check_report_fast(&report,None);
        // let old_fail_count = check_report_fail_count_brute_fast(report);
        // if fail_count != old_fail_count{
        //     println!("Old Mistake! Report:{:?}, New Val:{}, Old Val:{}",report,fail_count, old_fail_count)
        // }
        if fail_count == None { safe_reports += 1}
    }
    safe_reports
}

pub fn part2(input:&str) -> usize {
    let reports = input.split(0xA as char);
    let mut dampened_safe_reports:usize = 0;
    for report in reports{
        let fail_count = check_report_fail_count_brute_fast(report);
        // let old_fail_count = check_report_fail_count_brute_fast(report);
        // if fail_count != old_fail_count{
        //     println!("Old Mistake! Report:{:?}, New Val:{}, Old Val:{}",report,fail_count, old_fail_count)
        // // }
        // if fail_count < 1 { safe_reports += 1}
        if fail_count < 2 { dampened_safe_reports += 1 }
    }
    dampened_safe_reports
}

fn check_report_fail_count_brute_fast(report_slice:&str) -> u32{
    let report = report_slice.split_whitespace().map(|x|x.parse::<u32>().unwrap());
    let initial_check = check_report_fast(&report,None);
    match initial_check {
        None => {0},
        Some(0) => 2,
        Some(x) => {
            if check_report_fast(&report, Some(x-1)).is_none() {1}
            else if check_report_fast(&report, Some(x)).is_none() {1}
            else if x > 1 && check_report_fast(&report, Some(0)).is_none() {1}
            else {2}
        }
    }
}

fn check_report_fast(report: &(impl Iterator<Item = u32> + Clone), skip_i:Option<usize>) -> Option<usize>{
    let mut report = report.clone();
    if let Some(0) = skip_i {report.next();}
    let mut last_val: u32 = match report.next() { None => {return Some(0);}, Some(x)=> x};
    let mut check_state: CheckState = CheckState::NotYetCompared;
    let mut fail = None;
    let mut i:usize = 0;
    for value in report{
        i += 1;
        if let Some(skip_i) = skip_i {if i == skip_i{continue;}}
        let valid_direction:bool = match check_state{
            CheckState::Decreasing => value < last_val,
            CheckState::Increasing => value > last_val,
            CheckState::NotYetCompared => {
                check_state = if value > last_val {CheckState::Increasing} else {CheckState::Decreasing};
                true
            }
        };
        let diff = value.abs_diff(last_val);
        if !(valid_direction && (1..=3).contains(&diff)) {
            fail = Some(i);
            break;
        }
        last_val = value;
    }
    fail
}

#[allow(unused)]
const TEST_INPUT:&str =  include_str!("day2_test.txt");

#[cfg(test)]
mod tests {
    use crate::problems::day2::{part1, part2, TEST_INPUT};

    #[test]
    fn day2_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(part2(TEST_INPUT), 4)
    }
}