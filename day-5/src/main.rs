use std::fs;

use day_5::{construct_adjacency_list, get_list, Dag};

fn main() {
    let contents: String = fs::read_to_string("input.txt").unwrap();
    let mut flag: bool = false;
    let mut content1: String = String::new();
    let mut content2: String = String::new();
    for line in contents.lines() {
        if line.trim().is_empty() {
            flag = true;
            continue;
        }

        if flag {
            let _ = &content2.push_str(line.trim());
            let _ = &content2.push_str("\n");
        } else {
            let _ = &content1.push_str(line.trim());
            let _ = &content1.push_str("\n");
        }
    }

    let dag: Dag = construct_adjacency_list(content1);

    let mut valid_updates: Vec<Vec<usize>> =
        Vec::with_capacity(content2.lines().count() / 2 as usize);

    let mut invalid_updates: Vec<Vec<usize>> =
        Vec::with_capacity(content2.lines().count() / 2 as usize);

    for line in content2.lines() {
        let mut list: Vec<usize> = get_list(line.to_string());
        if !dag.topological_sort(&mut list) {
            valid_updates.push(list);
        } else {
            let _ = dag.topological_sort(&mut list);
            invalid_updates.push(list);
        }
    }

    let mut total: usize = 0;

    for list in valid_updates {
        let mid: usize = list.len() / 2 as usize;
        total += list[mid];
    }

    println!("The total is {}", total);

    let mut total: usize = 0;

    for list in invalid_updates {
        let mut mid: usize = list.len() / 2 as usize;
        if list.len() % 2 != 0 {
            mid += 1;
        }

        total += list[mid];
    }

    println!("The invalid total is {}", total);
}
