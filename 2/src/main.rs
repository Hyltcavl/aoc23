use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("shit broke");
    let list_of_lines: Vec<_> = input.lines().collect();
    let mut count = 0;

    list_of_lines.iter().for_each(|line| {
        let split_game_from_result: Vec<String> = line.split(": ").map(String::from).collect();
        let result: Vec<String> = split_game_from_result[1]
            .split("; ")
            .map(String::from)
            .collect();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        result
            .iter()
            // .map(|line| line.split(","))
            .for_each(|line| {
                line.split(", ").for_each(|dicecount| {
                    let num_color_vec: Vec<&str> = dicecount.split(' ').collect();
                    // println!("{}, {},{}",line, num_color_vec[0], num_color_vec[1]);
                    if num_color_vec[1] == "red" {
                        let number = num_color_vec[0].parse::<i32>().unwrap();
                        if number > red {
                            red = number
                        }
                    }
                    if num_color_vec[1] == "green" {
                        let number = num_color_vec[0].parse::<i32>().unwrap();
                        if number > green {
                            green = number
                        }
                    }
                    if num_color_vec[1] == "blue" {
                        let number = num_color_vec[0].parse::<i32>().unwrap();
                        if number > blue {
                            blue = number
                        }
                    }
                })
            });
            count = (red * blue * green) + count
    });

    println!("{}", count)
    //Soltion 1
    // let input = fs::read_to_string("input.txt").expect("shit broke");
    // let list_of_lines: Vec<_> = input.lines().collect();
    // let mut count = 0;

    // list_of_lines.iter().for_each(|line| {
    //     let split_game_from_result: Vec<String> = line.split(": ").map(String::from).collect();
    //     let result: Vec<String> = split_game_from_result[1]
    //         .split("; ")
    //         .map(String::from)
    //         .collect();
    //     let gameNumber = split_game_from_result[0]
    //         .replace("Game ", "")
    //         .parse::<i32>()
    //         .unwrap();
    //     let mut possible: bool = true;
    //     result
    //         .iter()
    //         // .map(|line| line.split(","))
    //         .for_each(|line| {
    //             line.split(", ").for_each(|dicecount| {
    //                 let num_color_vec: Vec<&str> = dicecount.split(' ').collect();
    //                 // println!("{}, {},{}",line, num_color_vec[0], num_color_vec[1]);
    //                 if num_color_vec[1] == "red" {
    //                     if num_color_vec[0].parse::<i32>().unwrap() > 12 {
    //                         possible = false;
    //                         println!("{}, {}", gameNumber, possible);

    //                     }
    //                 }
    //                 if num_color_vec[1] == "green" {
    //                     if num_color_vec[0].parse::<i32>().unwrap() > 13 {
    //                         possible = false;
    //                         println!("{}, {}", gameNumber, possible);
    //                     }
    //                 }
    //                 if num_color_vec[1] == "blue" {
    //                     if num_color_vec[0].parse::<i32>().unwrap() > 14 {
    //                         possible = false;
    //                         println!("{}, {}", gameNumber, possible);
    //                     }
    //                 }
    //             })
    //         });
    //     if possible {
    //         count = count + gameNumber;
    //         println!("row: {}, count: {}", gameNumber, count);
    //     }
    // });

    // println!("{}", count)

    // let input = fs::read_to_string("input.txt").expect("shit broke");
    // let list_of_lines: Vec<_> = input.lines().collect();
    // let list_of_lines_cleaned: Vec<Vec<String>> = list_of_lines
    //     .iter()
    //     .map(|line| line.split(":").map(String::from).collect())
    //     .map(|line: String| line.split(";").map(String::from).collect())
    //     .collect();
    // let mut count = 0;
    // for line in list_of_lines_cleaned {
    //     let mut possible = true;
    //     line.iter().for_each(|line| {
    //         line.split(",").for_each(|dicecount| {
    //             let num_color_vec: Vec<&str> = dicecount.split(" ").collect();
    //             if num_color_vec[1] == "red" {
    //                 if num_color_vec[0].parse::<i32>().unwrap() > 12 {
    //                     possible = false
    //                 }
    //             }
    //             if num_color_vec[1] == "green" {
    //                 if num_color_vec[0].parse::<i32>().unwrap() > 12 {
    //                     possible = false
    //                 }
    //             }
    //             if num_color_vec[1] == "blue" {
    //                 if num_color_vec[0].parse::<i32>().unwrap() > 13 {
    //                     possible = false
    //                 }
    //             }
    //         })
    //     })
    //     if possible
    // }

    // println!("{}", input)
}
