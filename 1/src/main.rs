use regex::Regex;
use std::{fs, ops::Add};

fn main() {
    // let example_data = vec!["4nineight"];
    // let example_data = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    let file_contents =
        fs::read_to_string("input.txt").expect("LogRocket: Should have been able to read the file");
    let example_data: Vec<_> = file_contents.lines().collect();
    let mut big_num: i32 = 0;
    let regex = Regex::new(r#"(one|two|three|four|five|six|seven|eight|nine)"#).unwrap();
    for text in example_data {
        let digitized_text =
            regex.replace_all(text, |capture: &regex::Captures| match &capture[0] {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => unreachable!(),
            });
        let nums: Vec<char> = digitized_text.chars().filter(|x| x.is_numeric()).collect();

        let reversed_regex =
            Regex::new(r#"(enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)"#).unwrap();
        let reversed = text.chars().rev().collect::<String>();
        let digitized_text2 = reversed_regex.replace_all(
            reversed.as_str(),
            |capture: &regex::Captures| match &capture[0] {
                "enin" => "9",
                "thgie" => "8",
                "neves" => "7",
                "xis" => "6",
                "evif" => "5",
                "ruof" => "4",
                "eerht" => "3",
                "owt" => "2",
                "eno" => "1",
                _ => unreachable!(),
            },
        );
        let nums2: Vec<char> = digitized_text2.chars().filter(|x| x.is_numeric()).collect();

        let first: String = nums.first().unwrap().to_string();
        let second: String = nums2.first().unwrap().to_string();
        let total = format!("{first}{second}");
        let my_int: i32 = total.parse().unwrap();
        big_num = big_num.add(my_int);
        println!(
            "{}, {}, {:?}, {}, {}",
            text, digitized_text, nums, total, big_num
        );
    }
    println!("{}", big_num)
}
