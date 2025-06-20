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

use std::{env, fs};
use std::time::{Duration, Instant};
use regex::Regex;
use size::Size;

mod problems;

const BENCHMARK_TIMES:u32 = 1000;
const BENCHMARK_TIMEOUT: Duration = Duration::from_secs(1);
include!(concat!(env!("OUT_DIR"), "/profile_info.rs"));

macro_rules! benchmark_problem_part {
    ($d:ident,$part:ident,$input:ident) => {{
        let mut time = Duration::new(0, 0);
        let total_now = Instant::now();
        let mut benchmark_times = 0u32;
        'benchmark_loop:for _i in 0..(if BUILD_PROFILE == "release" {BENCHMARK_TIMES} else {1}) {
            let now = Instant::now();
            problems::$d::$part(&$input);
            time += now.elapsed();
            benchmark_times += 1;
            if total_now.elapsed() > BENCHMARK_TIMEOUT {println!("timeout:{_i}");break 'benchmark_loop;}
        }
        time /= benchmark_times;
        time
    }};
}
macro_rules! allocations_problem_part {
    ($d:ident,$part:ident,$input:ident) => {{
        #[cfg(debug_assertions)]
        println!(stringify!($d));
        #[cfg(debug_assertions)]
        println!(stringify!($part));
        let allocations_info = allocation_counter::measure(||{problems::$d::$part(&$input);});
        (Size::from_bytes(allocations_info.bytes_max), allocations_info.count_total)
    }};
}

macro_rules! table_row {
    ($d:ident, $part1_mutlithreaded:tt, $part2_mutlithreaded:tt) => {{
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

        tr.push_str(&format!("\n<td>{load_time:.0?}</td>"));
        tr.push_str(&format!("\n<td>{}</td>",Size::from_bytes(input_len)));

        tr.push_str("\n<th>❌</th>");

        tr.push_str(&format!("\n<td>{:.0?}</td>",benchmark_problem_part!($d,part1,input)));
        let allocations = allocations_problem_part!($d,part1,input);
        tr.push_str(&format!("\n<td>{}</td><td>{:?}</td>", allocations.0, allocations.1));//asuming size in bytes
        tr.push_str(&format!("\n<td>{}</td>",problems::$d::part1(&input)));

        tr.push_str(&format!("\n<td>{:.0?}</td>",benchmark_problem_part!($d,part2,input)));
        let allocations = allocations_problem_part!($d,part2,input);
        tr.push_str(&format!("\n<td>{}</td><td>{:?}</td>", allocations.0, allocations.1));
        tr.push_str(&format!("\n<td>{}</td>",problems::$d::part2(&input)));

        tr.push_str("\n</tr>");

        #[allow(clippy::eq_op,clippy::nonminimal_bool)]
        if $part1_mutlithreaded || $part2_mutlithreaded {
            tr.push_str("\n<tr>");

            tr.push_str("\n<th>\"</th>");
            tr.push_str("\n<th>\"</th>");
            tr.push_str("\n<th>\"</th>");
            tr.push_str("\n<th>\"</th>");
            tr.push_str("\n<th>✅</th>");
            conditionally_expand!{$part1_mutlithreaded,
                {tr.push_str(&format!("\n<td>{:.0?}</td>",benchmark_problem_part!($d,part1_multithread,input)));
                let allocations = allocations_problem_part!($d,part1_multithread,input);
                tr.push_str(&format!("\n<td>{}</td><td>{:?}</td>",  allocations.0, allocations.1));//asuming size in bytes
                tr.push_str(&format!("\n<td>{}</td>",problems::$d::part1_multithread(&input)));}
            } ;
            if !$part1_mutlithreaded {
                tr.push_str("\n<th></th>");
                tr.push_str("\n<th></th>");
                tr.push_str("\n<th></th>");
                tr.push_str("\n<th></th>");
            }
            conditionally_expand!{$part2_mutlithreaded,
                {tr.push_str(&format!("\n<td>{:.0?}</td>",benchmark_problem_part!($d,part2_multithread,input)));
                //let allocations = (0,0);
                let allocations = allocations_problem_part!($d,part2_multithread,input);
                tr.push_str(&format!("\n<td>{}</td><td>{:?}</td>",  allocations.0, allocations.1));//asuming size in bytes
                tr.push_str(&format!("\n<td>{}</td>",problems::$d::part2_multithread(&input)));}
            };
            if !$part2_mutlithreaded {
                tr.push_str("\n<th></th>");
                tr.push_str("\n<th></th>");
                tr.push_str("\n<th></th>");
                tr.push_str("\n<th></th>");
            }

            tr.push_str("\n</tr>");
        }

        tr
    }}
}

macro_rules! conditionally_expand {
    {
        true,
        $fragment:block
    } => {
        $fragment
    };
    {
        false,
        $fragment:block
    } => {
    };
}

fn main() {
    let mut tbody = String::from("<tbody id=\"results\">");


    tbody.push_str(&table_row!(day1,false,false));
    tbody.push_str(&table_row!(day2,false,false));
    tbody.push_str(&table_row!(day3,false,false));
    tbody.push_str(&table_row!(day4,false,false));
    tbody.push_str(&table_row!(day5,false,false));
    tbody.push_str(&table_row!(day6,false,true));

    tbody.push_str("\n</tbody>");

    let readme = fs::read_to_string("README.md").expect("input missing");
    let updated_readme:String = Regex::new("<tbody id=\"results\">(.|\n)*</tbody>").unwrap().replace(&readme, tbody).into();
    let path = match BUILD_PROFILE == "release" {
        false => "DEV_README.md",
        true => match env::args().find(|arg| arg == "updatereadme") {
            Some(_) => "README.md",
            None => "LOCAL_README.md",
        }
    };

    fs::write(
        path,
        &updated_readme
    ).expect("could not write to file");

}
