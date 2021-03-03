use std::io;

/// std::io::stdin().read_line(&buf)
pub fn stdin_read_line(input: &mut String) -> bool {
    match io::stdin().read_line(input) {
        Ok(_) => {},
        Err(err) => {
            println!("Error when read line from stdin: {}", err);
            return false
        },
    }

    true
}

/// Default use `EOF` as end of the reading stage.
pub fn stdin_readlines(input: &mut String) -> bool {
    let eof = String::from("EOF");
    stdin_readlines_end_with(input, &eof)
}

/// CMDS
///     C_H: show help.
///     C_C: clear all input data.
///     C_D N: if N exist, delete the line N, else delete the last line
///     C_I N: insert new lines start with line N.
///     C_P: print the current inputed lines, if more the than 10 lines,
///         every time print 10 lines, then you can input `n` to print the next,
///         use `q` to quit print mode.
pub fn stdin_readlines_end_with(input: &mut String, eof: &String) -> bool {
    let mut lines: Vec<String> = Vec::new();
    let mut index = 0;
    let mut failed_times = 0;

    println!("> `{}` to stop reading from stdin, and `C_H` show help", eof.trim_end());
    // start reading until get eof str.
    loop {
        let mut line = String::new();

        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.trim_end().eq(eof) {
                    break
                }
                if line.eq("C_H\n") {
                    show_help();
                } else if line.eq("C_C\n") {
                    lines = Vec::new();
                } else if line.eq("C_P\n") {
                    printlines(&lines);
                // delete mode
                } else if line.starts_with("C_D") {
                    let args: Vec<_> = line.split(" ").collect();
                    if args.len() == 1 {
                        if lines.len() > 1 {
                            lines.pop().unwrap();
                            // update index
                            index -= 1;
                        } else {
                            println!("No line to drop!");
                        }
                    } else if args.len() == 2 {
                        let max = lines.len();
                        if let Ok(num) = args[1].trim_end().parse::<usize>() {
                            if num < 1 {
                                println!("Delete line must in 1 to {}", max);
                                continue;
                            } else if num > max {
                                println!("Delete line must in 1 to {}", max);
                                continue;
                            }
                            lines.remove(index - 1);
                            // update index
                            index -= 1;
                        } else {
                            println!("Delete line must in 1 to {}", max);
                            show_help();
                        }
                    } else {
                        show_help();
                    }
                // reset insert index
                } else if line.starts_with("C_I") {
                    let max = lines.len() + 1;
                    let args: Vec<_> = line.split(" ").collect();
                    if args.len() == 2 {
                        if let Ok(num) = args[1].trim_end().parse::<usize>() {
                            if num < 1 {
                                println!("Insert line must in 1 to {}", max);
                                continue;
                            } else if num > max {
                                println!("Insert line must in 1 to {}", max);
                                continue;
                            }
                            index = num - 1;
                        } else {}
                    } else {
                        show_help();
                    }
                } else {
                    lines.insert(index, line);
                    index += 1;
                }
            },
            Err(err) => {
                println!("Error when read line from stdin: {}", err);
                failed_times += 1;
                if failed_times > 3 {
                    println!("Failed more than 3 times, stop and exit.");
                    return false
                }
            },
        }
    }
    for line in &lines {
        input.push_str(line);
    }

    true
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
    let mut num = 0;
    let mut lnum = 1;
    let mut input = String::new();
    for line in lines {
        print!("{:2}  {}", lnum, line);
        num += 1;
        lnum += 1;
        if num == 10 {
            io::stdin().read_line(&mut input).unwrap();
            if input.eq("n\n") {
                num = 0;
            } else if input.eq("q\n") {
                break;
            }
        }
    }
}
