let read_file filename = 
 let channel = open_in filename in
  let rec read_lines acc = 
    try
        let line = input_line channel in
        read_lines (line :: acc)
    with End_of_file ->
        close_in channel;
        List.rev acc
 in
 read_lines []

let () = 
    let filename = "input.txt" in
    try
        let lines = read_file filename in
        List.iter print_endline lines
    with Sys_error err ->
        Printf.eprintf "Error: %s\n" err

