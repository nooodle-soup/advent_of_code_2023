use std::fs::File;
use std::{io, u32};
use std::io::{BufRead, BufReader};

fn main() {
    // Something is wrong with global snow production, and you've been
    // selected to take a look. The Elves have even given you a map; on it,
    // they've used stars to mark the top fifty locations that are likely to
    // be having problems.

    // You've been doing this long enough to know that to restore snow
    // operations, you need to check all fifty stars by December 25th.

    // Collect stars by solving puzzles. Two puzzles will be made available
    // on each day in the Advent calendar; the second puzzle is unlocked when
    // you complete the first. Each puzzle grants one star. Good luck!

    // You try to ask why they can't just use a weather machine
    // ("not powerful enough") and where they're even sending you ("the sky")
    // and why your map looks mostly blank ("you sure ask a lot of questions")
    // and hang on did you just say the sky ("of course, where do you
    // think snow comes from") when you realize that the Elves are already
    // loading you into a trebuchet ("please hold still, we need to strap
    // you in").

    // As they're making the final adjustments, they discover that their
    // calibration document (your puzzle input) has been amended by a very
    // young Elf who was apparently just excited to show off her art skills.
    // Consequently, the Elves are having trouble reading the values on
    // the document.

    println!("DAY 01");
    match part1("src/input.txt") {
        Ok(sum) => {
            println!("SUCCESS PART 1:  {}", sum);
        }
        Err(err) => {
            println!("FAILURE PART 1: {}", err);
        }
    }
    match part2("src/input.txt") {
        Ok(sum) => {
            println!("SUCCESS PART 2:  {}", sum);
        }
        Err(err) => {
            println!("FAILURE PART 2: {}", err);
        }
    }

}

fn part1(file_path: &str) -> io::Result<u32>{
    // The newly-improved calibration document consists of lines of text;
    // each line originally contained a specific calibration value that the
    // Elves now need to recover. On each line, the calibration value can be
    // found by combining the first digit and the last digit (in that order)
    // to form a single two-digit number.

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let output = reader
        .lines() // Get all lines
        .map(|line| { // Map each line
            let l = line.expect("Must be a line"); // Unwrap line
            let mut it = l
                .chars() // Get each character
                .filter_map(|character| { // Filter to digit
                    character.to_digit(10)
                }
            );
            let first = it.next().expect("should be a number");
            
            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}")
            }
            .parse::<u32>()
            .expect("Should be a valid number")
        })
        .sum::<u32>();

    Ok(output)
}

fn part2(file_path: &str) -> io::Result<u32> {
    // Your calculation isn't quite right. It looks like some of the digits
    // are actually spelled out with letters: one, two, three, four, five,
    // six, seven, eight, and nine also count as valid "digits".

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let output = reader
        .lines()
        .map(|line| {
            let l = line.unwrap();
            let mut it = (0..l.len()).filter_map(|index| {
                let reduced_line = &l[index..];
                let result = if reduced_line.starts_with("one") {
                    '1'
                } else if reduced_line.starts_with("two") {
                    '2'
                } else if reduced_line.starts_with("three") {
                    '3'
                } else if reduced_line.starts_with("four") {
                    '4'
                } else if reduced_line.starts_with("five") {
                    '5'
                } else if reduced_line.starts_with("six") {
                    '6'
                } else if reduced_line.starts_with("seven") {
                    '7'
                } else if reduced_line.starts_with("eight") {
                    '8'
                } else if reduced_line.starts_with("nine") {
                    '9'
                } else {
                    reduced_line.chars().next().unwrap()
                };

                result.to_digit(10)
            }
        );
        let first = it.next().expect("should be a number");

        match it.last() {
            Some(num) => format!("{first}{num}"),
            None => format!("{first}{first}"),
        }
        .parse::<u32>()
            .expect("should be a valid number")

        }).sum::<u32>();

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_file_path = "src/test_input_part1.txt";

        match part1(test_file_path) {
            Ok(result) => {
                assert_eq!(result, 142);
            }
            Err(err) => {
                panic!("ERROR: {err}");
            }
        }

    }

    #[test]
    fn test_part2() {
        let test_file_path = "src/test_input_part2.txt";

        match part2(test_file_path) {
            Ok(result) => {
                assert_eq!(result, 281);
            }
            Err(err) => {
                panic!("ERROR: {err}");
            }
        }

    }
}
