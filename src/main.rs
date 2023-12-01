use std::{borrow::BorrowMut, fs, ops::Add};

fn main() {
    let file_contents =
        fs::read_to_string("input.txt").expect("LogRocket: Should have been able to read the file");
    let content_list: Vec<_> = file_contents.lines().collect();
    println!("{:#?}", content_list[2]);

    let mut big_num: i32 = 0;

    for s in content_list {
        //let listOfNumbers: Vec<char> = s.split(char::is_numeric);
        //let nums: Vec<_> =
        let nums: Vec<char> = s.chars().filter(|x| x.is_numeric()).collect();

        println!("{}", s);

        if nums.first().is_some() && nums.last().is_some() {
            let first: String = nums.first().unwrap().to_string();
            let second: String = nums.last().unwrap().to_string();
            let total = format!("{first}{second}");
            println!("{}", total);
            let my_int: i32 = total.parse().unwrap();
            big_num = big_num.add(my_int);
        }
    }
    print!("{}", big_num);
}
