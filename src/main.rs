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
use std::time::{Duration, Instant};

mod problems;
mod allocation_track;

const BENCHMARK_TIMES:u32 = 10;

macro_rules! benchmark_problem_part {
    ($d:ident,$part:ident,$input:ident) => {{
        let mut time = Duration::new(0, 0);
        for i in 0..BENCHMARK_TIMES {
            let now = Instant::now();
            problems::$d::$part(&$input);
            time += now.elapsed();
        }
        time /= BENCHMARK_TIMES;
        time
    }};
}
macro_rules! allocations_problem_part {
    ($d:ident,$part:ident,$input:ident) => {{
        allocation_track::AllocationRegistry::enable_tracking();
        problems::$d::$part(&$input);
        allocation_track::AllocationRegistry::disable_tracking();
        allocation_track::AllocationRegistry::get_global_tracker().
    }};
}

macro_rules! table_row {
    ($d:ident) => {
        let day_string = stringify!($d);
        let day_num:String = "day1".chars().filter(|x| x.is_numeric()).collect();
        let day_num:usize = day_num.parse().unwrap();
        let now = Instant::now();
        let input = fs::read_to_string(format!("input/{day_string}.txt")).expect("input missing"); 
        let load_time = now.elapsed();

        println!("<tr>");

        println!("<th>{}</th>",day_num );
        println!("<th>{}</th>",PROBLEM_NAMES[day_num]);

        println!("<td>{:?}</td>",load_time);
        println!("<td>{}</td>",input.len());

        println!("<td>{:?}</td>",benchmark_problem_part!($d,part1,input));
        println!("<td>{:?}</td>",allocations_problem_part!($d,part1,input));
        println!("<td>{}</td>",problems::$d::part1(&input));

        println!("<td>{:?}</td>",benchmark_problem_part!($d,part2,input));
        println!("<td>{:?}</td>",allocations_problem_part!($d,part2,input));
        println!("<td>{}</td>",problems::$d::part1(&input));

        println!("</tr>");
    };
}

fn main() {
    let _ = allocation_track::AllocationRegistry::set_global_tracker(allocation_track::StdoutTracker::new())
    .expect("no other global tracker should be set yet");
tracking_allocator::get_global_tracker();

    table_row!(day1);
}
