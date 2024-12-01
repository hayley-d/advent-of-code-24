use std::fs;

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Unable to read the file");
    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        println!("{:?}", parts);
        list1.push(parts[0].parse::<usize>().unwrap());
        list2.push(parts[1].parse::<usize>().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut total: usize = 0;
    for i in 0..list1.len() {
        total += find_difference(list1[i], list2[i]);
    }

    println!("The total distance is {}", total);
}

fn find_difference(num1: usize, num2: usize) -> usize {
    if num1 > num2 {
        return num1 - num2;
    } else {
        return num2 - num1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_test() {
        let mut my_vec: Vec<usize> = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        my_vec.sort();
        assert_eq!(my_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
