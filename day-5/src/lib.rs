pub mod dag;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Dag {
    adjacency_list: HashMap<usize, RefCell<Vec<usize>>>,
}

impl Dag {
    pub fn new() -> Self {
        return Dag {
            adjacency_list: HashMap::new(),
        };
    }

    pub fn add_dependancy(&mut self, start: usize, end: usize) {
        match self.adjacency_list.get(&start) {
            Some(list) => {
                list.borrow_mut().push(end);
            }
            None => {
                let mut list: Vec<usize> = Vec::with_capacity(1);
                list.push(end);
                self.adjacency_list.insert(start, RefCell::new(list));
            }
        }
    }

    pub fn get_list(&self, value: usize) -> Option<RefCell<Vec<usize>>> {
        match self.adjacency_list.get(&value) {
            Some(list) => Some(list.clone()),
            None => None,
        }
    }

    pub fn topological_sort(&self, list: &Vec<usize>) -> bool {
        for i in 0..list.len() {
            let num1: usize = list[i];

            let mut flag: bool = true;
            match self.adjacency_list.get(&num1) {
                Some(_) => (),
                None => flag = false,
            }

            for j in i + 1..list.len() {
                let num2: usize = list[j];

                if flag
                    && self
                        .adjacency_list
                        .get(&num1)
                        .unwrap()
                        .borrow()
                        .contains(&num2)
                {
                    continue;
                } else if self
                    .adjacency_list
                    .get(&num2)
                    .unwrap()
                    .borrow()
                    .contains(&num1)
                {
                    return false;
                }
            }
        }
        return true;
    }
}

pub fn construct_adjacency_list(content: String) -> Dag {
    let mut dag: Dag = Dag::new();
    for line in content.lines() {
        let parts: Vec<usize> = line
            .split("|")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let (start, end): (usize, usize) = (parts[0], parts[1]);

        drop(parts);

        let _ = &dag.add_dependancy(start, end);
    }

    return dag;
}

pub fn get_list(content: String) -> Vec<usize> {
    return content
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
}

#[cfg(test)]
mod tests {
    use crate::{construct_adjacency_list, Dag};

    #[test]
    fn test_dag_construction() {
        let content: String = String::from("47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13");
        let my_dag: Dag = construct_adjacency_list(content);

        assert_eq!(my_dag.get_list(47).unwrap().take(), vec![53, 13, 61, 29]);
    }
}
