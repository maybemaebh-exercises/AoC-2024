use tinyvec::TinyVec;

pub struct Day3();
impl crate::Day for Day3 {
    fn part1(&self, input: &str) -> Option<usize> {
        Some(part1(input))
    }

    fn part2(&self, input: &str) -> Option<usize> {
        Some(part2(input))
    }

    fn full_input(&self) -> &'static str {
        include_str!("../../input/day3.txt")
    }

    fn problem_name(&self) -> &'static str {
        "Mull It Over"
    }
}

fn part1(input:&str) -> usize {
    let input = input.chars();

    let mul_term = ['m','u','l','(',',',')'];

    let mut running_total:usize = 0;
    let mut index = 0;
    let mut first_term:usize = 0;
    let mut current_term:usize = 0;//if no number in either half, mul by zero so effectivly void

    for char in input {
        let search_char = mul_term[index];

        if search_char == char {
            index += 1;
            if search_char == ',' {
                first_term = current_term;
                current_term = 0;
            } else if search_char == ')' {
                index = 0;
                running_total += first_term * current_term;
                current_term = 0;
            }
        }

        else if search_char == ',' || search_char == ')' {
            match char.to_digit(10) {
                Some(dig) => {
                    current_term *= 10;
                    current_term += dig as usize;
                },
                None => {
                    index = 0;
                    current_term = 0
                }
            }
        }

        else {
            index = 0;
            current_term = 0;
        }

    }
    running_total
}

fn part2(input:&str) -> usize {

    let input = input.chars();

    let mul_term = "mul(,)";
    let enable_term = "do()";
    let disable_term = "don't()";
    let terms:[TinyVec<[char;10]>;3] = [mul_term.chars().collect(),enable_term.chars().collect(),disable_term.chars().collect()];

    let mut running_total:usize = 0;
    let mut indexs = [0,0,0];
    let mut first_term:usize = 0;
    let mut current_term:usize = 0;//if no number in either half, mul by zero so effectivly void
    let mut enabled = true;

    for char in input {
        for term_index in 0..3 {
            let search_char = terms[term_index][indexs[term_index]];
            let index = &mut indexs[term_index];

            if search_char == char {
                *index += 1;
                if term_index == 0 && search_char == ',' {
                    first_term = current_term;
                    current_term = 0;
                } else if search_char == ')' {
                    *index = 0;
                    match term_index {
                        0 => {
                            if enabled { running_total += first_term * current_term; }
                            current_term = 0;
                            Ok(())
                        },
                        1 => {
                            enabled = true;
                            Ok(())
                        },
                        2 => {
                            enabled = false;
                            Ok(())
                        },
                        _ => { Err("term index too high") }
                    }.unwrap();
                }
            }

            else if (search_char == ',' || search_char == ')') && term_index == 0 {
                match char.to_digit(10) {
                    Some(dig) => {
                        current_term *= 10;
                        current_term += dig as usize;
                    },
                    None => {
                        *index = 0;
                        if term_index == 0 {current_term = 0};
                    }
                }
            }

            else {
                *index = 0;
                if term_index == 0 {current_term = 0;}
            }
        }

    }
    running_total
}

#[allow(unused)]
const PART1_TEST_INPUT:&str =  "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
#[allow(unused)]
const PART2_TEST_INPUT:&str =  "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[cfg(test)]
mod tests {
    use crate::problems::day3::{part1, part2, PART1_TEST_INPUT, PART2_TEST_INPUT};

    #[test]
    fn day3_part1() {
        assert_eq!(part1(PART1_TEST_INPUT), 161);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(part2(PART2_TEST_INPUT), 48)
    }
}