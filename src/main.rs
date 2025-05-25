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
use std::time::Instant;

mod problems;


macro_rules! benchmark_problem {
    ($d:ident) => {
        let day_string = stringify!($d);
        let now = Instant::now();
        let input = fs::read_to_string(format!("input/{day_string}.txt")).expect("input missing"); 
        let load_time = now.elapsed();

        let now = Instant::now();
        let part1 = problems::$d::part1(&input);
        let part1_time = now.elapsed();

        let now = Instant::now();
        let part2 = problems::$d::part2(&input);
        let part2_time = now.elapsed();

        println("{}: Load")        
    };
}

fn main() {
    println!("|Name|Input Size|Load Time|");
    benchmark_problem!(day1);
}
