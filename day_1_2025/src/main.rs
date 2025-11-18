use std::collections::VecDeque;
use std::error::Error;
use std::fs;

struct Line<'a> {
    row_1: &'a str,
    row_2: &'a str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content: String = read_file();

    let lines = split_file_content(&file_content);

    let column_1 = extract_first_column(&lines);
    let column_2 = extract_second_column(&lines);

    task_1(column_1.clone(), column_2.clone());
    task_2(column_1, column_2);

    Ok(())
}

fn task_2<'a>(column_1: VecDeque<&'a str>, column_2: VecDeque<&'a str>) {
    let mut similarity = 0;

    for string_number in column_1 {
        let number: i32 = string_number.parse().unwrap();
        let count: i32 = column_2.iter().filter(|&s| *s == string_number).count() as i32;
        similarity += count * number;
    }

    println!("Similarity score is {}", similarity);
}

fn task_1<'a>(mut column_1: VecDeque<&'a str>, mut column_2: VecDeque<&'a str>) {
    let mut distance: i32 = 0;

    let column_1 = column_1.make_contiguous();
    let column_2 = column_2.make_contiguous();

    column_1.sort();
    column_2.sort();

    if column_1.len() != column_2.len() {
        println!("Error columns do not contain the same amount of elements");
        std::process::exit(1);
    }

    for index in 0..column_1.len() {
        let number_1: i32 = column_1[index].parse().unwrap();
        let number_2: i32 = column_2[index].parse().unwrap();

        distance += (number_1 - number_2).abs();
    }

    println!("The distance is {}", distance);
}

fn read_file() -> String {
    let file_path: &str = "input.txt";
    let contents = fs::read_to_string(file_path);
    let file_content = match contents {
        Ok(content) => content,
        Err(err) => {
            println!("Error getting input {}", err);
            std::process::exit(1);
        }
    };

    file_content
}

fn split_file_content<'a>(file_content: &'a str) -> std::collections::VecDeque<Line<'a>> {
    let mut collection_of_lines: std::collections::VecDeque<Line> =
        std::collections::VecDeque::new();

    let lines: Vec<&str> = file_content.lines().collect();

    for line in lines {
        let mut parts = line.split("   ");
        let row_1 = parts.next().unwrap_or("");
        let row_2 = parts.next().unwrap_or("");

        collection_of_lines.push_back(Line { row_1, row_2 });
    }

    collection_of_lines
}

fn extract_first_column<'a>(lines: &'a VecDeque<Line<'a>>) -> VecDeque<&'a str> {
    let mut column_1: VecDeque<&'a str> = VecDeque::new();

    for line in lines {
        column_1.push_back(line.row_1);
    }

    column_1
}

fn extract_second_column<'a>(lines: &'a VecDeque<Line<'a>>) -> VecDeque<&'a str> {
    let mut column_2: VecDeque<&'a str> = VecDeque::new();

    for line in lines {
        column_2.push_back(line.row_2);
    }

    column_2
}
