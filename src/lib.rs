use std::io;

/// Default use `EOF` as end of the reading stage.
pub fn stdin_readlines(input: &mut String) {
    let eof = String::from("EOF\n");
    stdin_readlines_end_with(input, &eof);
}

/// CMDS
///     C_H: show help.
///     C_C: clear all input data.
///     C_D N: if N exist, delete the line N, else delete the last line
///     C_I N: start insert new line under the line N.
///     C_P: print the current inputed lines, if more the than 10 lines,
///         every time print 10 lines, then you can input `n` to print the next,
///         use `q` to quit print mode.
pub fn stdin_readlines_end_with(input: &mut String, eof: &String) {
    let mut lines: Vec<String> = Vec::new();
    let mut index = 0;

    println!("`{}` to stop reading from stdin, and `C_H` show help", eof.trim_end());
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.eq(eof) {
                    break
                }
                if line.eq("C_H\n") {
                    show_help();
                } else if line.eq("C_C\n") {
                    lines = Vec::new();
                } else if line.eq("C_P\n") {
                    printlines(&lines);
                } else if line.starts_with("C_D") {
                    let args: Vec<_> = line.split(" ").collect();
                    if args.len() == 1 {
                        if lines.len() > 1 {
                            lines.pop().unwrap();
                        } else {
                            println!("No line to drop!");
                        }
                    } else if args.len() == 2 {
                        if let Ok(num) = args[1].trim_end().parse::<usize>() {
                            let max = lines.len() + 1;
                            if num < 1 {
                            } else if num <= max {
                            } else {
                                let index = num - 1;
                                lines.remove(index);
                            }
                        } else {
                            show_help();
                        }
                    } else {
                        show_help();
                    }
                } else if line.starts_with("C_I") {
                    let args: Vec<_> = line.split(" ").collect();
                    if args.len() == 2 {
                        if let Ok(num) = args[1].trim_end().parse::<usize>() {
                            index = num - 1;
                        }
                    } else {
                        show_help();
                    }
                } else {
                    lines.insert(index, line);
                    index += 1;
                }
            },
            Err(err) => println!("Error when read line from stdin: {}", err),
        }
    }
    for line in &lines {
        input.push_str(line);
    }
}

fn show_help() {
    let help_info = "
Usage:
    C_H         Show this help
    C_C         Clear all the inputed data
    C_D [N]     Delete the line N, or delete the last line
    C_I [N]     Move the insert index to line N
    C_P         print the current inputed lines, if more the than 10 lines,
                every time print 10 lines, then you can input `n` to print
                the next, use `q` to quit print mode
    EOF         Stop readlines
";

    println!("{}", help_info);
}

fn printlines(lines: &Vec<String>) {
    let mut num = 1;
    let mut input = String::new();
    for line in lines {
        print!("{}: {}", num, line);
        num += 1;
        if num == 10 {
            io::stdin().read_line(&mut input).unwrap();
            if input.eq("n\n") {
                num = 1;
            } else if input.eq("q\n") {
                break;
            }
        }
    }
}
