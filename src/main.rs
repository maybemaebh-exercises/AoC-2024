use regex::Regex;
use size::Size;
use std::time::{Duration, Instant};
use std::{env, fs};
use std::path::Path;
use std::process::Command;
use crate::problems::day1::Day1;
use crate::problems::day2::Day2;
use crate::problems::day3::Day3;
use crate::problems::day4::Day4;
use crate::problems::day5::Day5;
use crate::problems::day6::Day6;
use crate::problems::day7::Day7;
use crate::problems::day8::Day8;
use crate::problems::day9::Day9;
use crate::problems::day10::Day10;

mod problems;
mod staging_tracking_allocator;
#[global_allocator]
static GLOBAL: staging_tracking_allocator::TrackingAllocator<std::alloc::System> = staging_tracking_allocator::TrackingAllocator(std::alloc::System);

const BENCHMARK_TIMES:u32 = 1000;
const BENCHMARK_TIMEOUT: Duration = Duration::from_secs(1);
include!(concat!(env!("OUT_DIR"), "/profile_info.rs"));

fn benchmark_problem_part (day: &dyn Day, part2:bool, multithreaded:bool) -> Duration {
        let mut time = Duration::new(0, 0);
        let total_now = Instant::now();
        let mut benchmark_times = 0u32;
        'benchmark_loop:for _i in 0..(if BUILD_PROFILE == "release" {BENCHMARK_TIMES} else {1}) {
            let now = Instant::now();
            day.run_part(part2, multithreaded);
            time += now.elapsed();
            benchmark_times += 1;
            if total_now.elapsed() > BENCHMARK_TIMEOUT {println!("timeout:{_i}");break 'benchmark_loop;}
        }
        time /= benchmark_times;
        time
    }
fn allocations_problem_part (day: &dyn Day, part2:bool, multithreaded:bool) -> (Size, usize) {
    #[cfg(debug_assertions)]
    println!(stringify!($d));
    #[cfg(debug_assertions)]
    println!(stringify!($part));
    unsafe {
        // No limit, no explicit failure handler for this example
        GLOBAL.start_tracking(None, None);
    }
    day.run_part(part2, multithreaded);
    let peak_allocation = GLOBAL.end_tracking();
    (Size::from_bytes(peak_allocation.0), peak_allocation.1)
}

fn repeat_part(days: &[Box<dyn Day>]) {
    // println!("{:?}", env::args().collect::<Vec<_>>());
    let day_num_regex = Regex::new(r"day(\d+)").unwrap();
    let day_num: usize = env::args()
        .filter_map(|arg| day_num_regex.captures(&arg)?[1].parse().ok())
        .next().expect(r"must have a day\d+ paramerter");
    let start_time = Instant::now();
    while start_time.elapsed() < FLAMEGRAPH_TIMEOUT {
        let _ = days[day_num - 1].run_part(
            env::args().any(|arg| &arg == "part2"),
            env::args().any(|arg| &arg == "multithreaded")
        );
    }
}

#[allow(clippy::borrowed_box)]
fn generate_flamegraph(day: (usize, &Box<dyn Day>), part2:bool, multithreaded:bool) {
    let permutation = [
        format!("day{}",day.0 + 1),
        if part2 { "part2" } else { "part1" }.to_string(),
        if multithreaded { "multithreaded" } else { "" }.to_string()
    ];
    let mut path = Path::new(match env::args().find(|arg| arg == "update-readme") {
        Some(_) => "",
        None => "local_flamegraph"
    })
        .join(permutation.join("-"));
    path.set_extension("svg");
    // println!("output path:{path:?} -> {}", path.to_str().unwrap());
    let _status = Command::new("cargo")
        .arg("flamegraph")
        .arg("--profile=release-debug")
        .arg(format!("--output={}",path.to_str().unwrap()))
        .arg("--")
        .arg("repeat-part")
        .args(permutation)
        .status()
        .unwrap();
    //println!("{status:?}");
}

#[allow(clippy::borrowed_box)]
fn day_row_and_flamegraph(day: &(usize, &Box<dyn Day>)) -> String{
    let mut tr = String::new();

    tr.push_str("\n<tr>");

    tr.push_str(&format!("\n<th>{}</th>", day.0 + 1));
    tr.push_str(&format!("\n<th>{}</th>", day.1.problem_name()));

    tr.push_str(&format!("\n<td>{}</td>", Size::from_bytes(day.1.full_input().len())));

    tr.push_str("\n<th>❌</th>");

    let mut day_multithreaded = false;
    let mut multithreaded_row = String::from("\n<tr>\n<th></th>\n<th></th>\n<th></th>\n<th>✅</th>");

    for permutation in 0..4 {
        let (part2, multithreaded) = (permutation % 2 == 1, permutation / 2 == 0);
        let mut part_row = String::new();

        if day.1.run_part(part2, multithreaded).is_some() {
            generate_flamegraph(*day, part2, multithreaded);
            if multithreaded { day_multithreaded = true; }

            part_row.push_str(&format!("\n<td>{:.0?}</td>", benchmark_problem_part(day.1.as_ref(), part2, multithreaded)));
            let allocations = allocations_problem_part(day.1.as_ref(), part2, multithreaded);
            part_row.push_str(&format!("\n<td>{}</td><td>{:?}</td>", allocations.0, allocations.1));
            part_row.push_str(&format!("\n<td>{}</td>", day.1.run_part(part2, multithreaded).unwrap()));
        } else {
            part_row.push_str("\n<th></th>\n<th></th>\n<th></th>\n<th></th>");
        }
        if multithreaded { multithreaded_row.push_str(&part_row); } else { tr.push_str(&part_row); }
    }
    tr.push_str("\n</tr>");
    if day_multithreaded {
        multithreaded_row.push_str("\n</tr>");
        tr.push_str(&multithreaded_row);
    }
    tr
}

#[allow(unused_variables)]
pub trait Day {
    fn part1(&self, input: &str) -> Option<usize> {None}
    fn part2(&self, input: &str) -> Option<usize> {None}
    fn part1_multithreaded(&self, input: &str) -> Option<usize> {None}
    fn part2_multithreaded(&self, input: &str) -> Option<usize> {None}
    fn full_input(&self) -> &'static str;
    fn problem_name(&self) -> &'static str;
    fn run_part(&self, part2: bool, multithreaded: bool) -> Option<usize> {
        match (part2, multithreaded) {
            (false, false) => {self.part1(self.full_input())},
            (true, false) => {self.part2(self.full_input())},
            (false, true) => {self.part1_multithreaded(self.full_input())},
            (true, true) => {self.part2_multithreaded(self.full_input())},
        }
    }
}
const FLAMEGRAPH_TIMEOUT:Duration = Duration::from_secs(1);
fn main() {
    let days:Vec<Box<dyn Day>> = vec![
        Box::new(Day1()),
        Box::new(Day2()),
        Box::new(Day3()),
        Box::new(Day4()),
        Box::new(Day5()),
        Box::new(Day6()),
        Box::new(Day7()),
        Box::new(Day8()),
        Box::new(Day9()),
        Box::new(Day10())
    ];

    if env::args().any(|arg| &arg == "repeat-part") {
        repeat_part(&days);
        return;
    }

    let mut tbody = String::from("<tbody id=\"results\">");

    let day_num_regex = Regex::new(r"day(\d+)").unwrap();

    if !env::args().any(|arg| day_num_regex.is_match(&arg)) {
        for day in days.iter().enumerate() {
            tbody.push_str(&day_row_and_flamegraph(&day));
        }
    } else {
        for day_num in env::args()
            .filter_map(|arg| day_num_regex.captures(&arg)?[1].parse::<usize>().ok()) {
            tbody.push_str(&day_row_and_flamegraph(&(day_num-1,&days[day_num-1])));
        }
    }


    tbody.push_str("\n</tbody>");

    let readme = fs::read_to_string("README.md").expect("input missing");
    let updated_readme:String = Regex::new("<tbody id=\"results\">(.|\n)*</tbody>").unwrap().replace(&readme, tbody).into();
    let path = match BUILD_PROFILE == "release" {
        false => "DEV_README.md",
        true => match env::args().find(|arg| arg == "update-readme") {
            Some(_) => "README.md",
            None => "LOCAL_README.md",
        }
    };

    fs::write(
        path,
        &updated_readme
    ).expect("could not write to file");

}

