use std::collections;
use std::error::Error;
use std::fs;

struct Line<'a> {
    row_1: &'a str,
    row_2: &'a str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content: String = read_file();
    let _lines = split_file_content(&file_content);
    Ok(())
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

    if let Some(first_line) = collection_of_lines.get(0) {
        println!("The first row and first col: {}", first_line.row_1);
    }

    collection_of_lines
}
