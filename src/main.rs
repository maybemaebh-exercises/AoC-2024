const PROBLEM_NAMES: [&str; 25] = [
    "Historian Hysteria",
    "Red-Nosed Reports",
    "Mull It Over",
    "Ceres Search",
    "Print Queue",
    "Guard Gallivant",
    "Bridge Repair",
    "Resonant Collinearity",
    "Disk Fragmenter",
    "Hoof It",
    "Plutonian Pebbles",
    "Garden Groups",
    "Claw Contraption",
    "Restroom Redoubt",
    "Warehouse Woes",
    "Reindeer Maze",
    "Chronospatial Computer",
    "RAM Run",
    "Linen Layout",
    "Race Condition",
    "Keypad Conundrum",
    "Monkey Market",
    "LAN Party",
    "Crossed Wires",
    "Code Chronicle",
];

use std::fs;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use regex::Regex;

mod problems;
mod allocation_track;

const BENCHMARK_TIMES:u32 = 10;
include!(concat!(env!("OUT_DIR"), "/profile_info.rs"));

macro_rules! benchmark_problem_part {
    ($d:ident,$part:ident,$input:ident) => {{
        let mut time = Duration::new(0, 0);
        for _i in 0..(if BUILD_PROFILE == "release" {BENCHMARK_TIMES} else {1}) {
            let now = Instant::now();
            problems::$d::$part(&$input);
            time += now.elapsed();
        }
        time /= BENCHMARK_TIMES;
        time
    }};
}
macro_rules! allocations_problem_part {
    ($d:ident,$part:ident,$input:ident, $alloc_reciver:ident) => {{
        #[cfg(debug_assertions)]
        println!(stringify!($d));
        #[cfg(debug_assertions)]
        println!(stringify!($part));
        allocation_track::AllocationRegistry::enable_tracking();
        problems::$d::$part(&$input);
        allocation_track::AllocationRegistry::disable_tracking();
        let mut allocation_count:usize = 0;
        let mut allocation_size:usize = 0;
        for alocation in $alloc_reciver.try_iter() {
            allocation_count += 1;
            allocation_size += alocation
        }
        (allocation_size, allocation_count)
    }};
}

macro_rules! table_row {
    ($d:ident, $alloc_reciver:ident) => {{
        let mut tr = String::new();
        let day_string = stringify!($d);
        let day_num:String = day_string.chars().filter(|x| x.is_numeric()).collect();
        let day_num:usize = day_num.parse().unwrap();
        let now = Instant::now();
        let input = fs::read(format!("input/{day_string}.txt")).expect("input missing");
        let input_len = input.len();
        let input = String::from_utf8_lossy(&input);
        let load_time = now.elapsed();

        tr.push_str("\n<tr>");

        tr.push_str(&format!("\n<th>{}</th>",day_num));
        tr.push_str(&format!("\n<th>{}</th>",PROBLEM_NAMES[day_num-1]));

        tr.push_str(&format!("\n<td>{:?}</td>",load_time));
        tr.push_str(&format!("\n<td>{}b</td>",input_len));

        tr.push_str(&format!("\n<td>{:?}</td>",benchmark_problem_part!($d,part1,input)));
        let allocations = allocations_problem_part!($d,part1,input,$alloc_reciver);
        tr.push_str(&format!("\n<td>{:?}b</td><td>{:?}</td>", allocations.0,allocations.1));//asuming size in bytes
        tr.push_str(&format!("\n<td>{}</td>",problems::$d::part1(&input)));

        tr.push_str(&format!("\n<td>{:?}</td>",benchmark_problem_part!($d,part2,input)));
        let allocations = allocations_problem_part!($d,part2,input,$alloc_reciver);
        tr.push_str(&format!("\n<td>{:?}b</td><td>{:?}</td>", allocations.0,allocations.1));
        tr.push_str(&format!("\n<td>{}</td>",problems::$d::part2(&input)));

        tr.push_str("\n</tr>");
        tr
    }}
}

fn main() {
    let mut tbody = String::from("<tbody id=\"results\">");
    let (allocation_size_send, allocation_size_receive) = mpsc::channel();
    allocation_track::AllocationRegistry::set_global_tracker(allocation_track::StdoutTracker::new(allocation_size_send))
    .expect("no other global tracker should be set yet");

    tbody.push_str(&table_row!(day1,allocation_size_receive));
    tbody.push_str(&table_row!(day2,allocation_size_receive));
    tbody.push_str(&table_row!(day3,allocation_size_receive));
    tbody.push_str(&table_row!(day4,allocation_size_receive));
    tbody.push_str(&table_row!(day5,allocation_size_receive));
    tbody.push_str(&table_row!(day6,allocation_size_receive));

    tbody.push_str("\n</tbody>");

    let readme = fs::read_to_string("README.md").expect("input missing");
    let updated_readme:String = Regex::new("<tbody id=\"results\">(.|\n)*</tbody>").unwrap().replace(&readme, tbody).into();
    if BUILD_PROFILE == "release" {
        fs::write("README.md", &updated_readme).expect("could not write to file");
    } else {
        println!("{}", updated_readme);
    }
}
