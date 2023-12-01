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
    match part1("src/input_part1.txt") {
        Ok(sum) => {
            println!("SUCCESS PART 1:  {}", sum);
        }
        Err(err) => {
            println!("FAILURE PART 1: {}", err);
        }
    }

}

fn part1(file_path: &str) -> io::Result<u32>{
    // The newly-improved calibration document consists of lines of text;
    // each line originally contained a specific calibration value that the
    // Elves now need to recover. On each line, the calibration value can be
    // found by combining the first digit and the last digit (in that order)
    // to form a single two-digit number.

    let mut calibration_values = Vec::<u32>::new();
    
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;

        let line = line?;
        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(digit);
                }
                last_digit = Some(digit);
            }
        }

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            let combined = format!("{}{}", first, last).parse::<u32>();
            match combined {
                Ok(int_val) => {
                    calibration_values.push(int_val);
                }
                Err(err) => {
                    println!("Error creating integer: {}", err);
                }
            }
        }
    }

    let sum: u32 = calibration_values.iter().sum();

    Ok(sum)
}

fn part2() {
    // Your calculation isn't quite right. It looks like some of the digits
    // are actually spelled out with letters: one, two, three, four, five,
    // six, seven, eight, and nine also count as valid "digits".
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
}
