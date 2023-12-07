use std::collections::HashMap;
use crate::debugger_command::DebuggerCommand;
use crate::inferior::{Inferior, self};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::dwarf_data::{DwarfData, Error as DwarfError};

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
    breakpoints: HashMap<usize, u8>,    // addr -> orig_byte
}

fn parse_address(addr: &str) -> Option<usize> {
    let addr_without_0x = if addr.to_lowercase().starts_with("0x") {
        &addr[2..]
    } else {
        &addr
    };
    usize::from_str_radix(addr_without_0x, 16).ok()
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("Could not open file {}", target);
                std::process::exit(1);
            }
            Err(DwarfError::DwarfFormatError(err)) => {
                println!("Could not debugging symbols from {}: {:?}", target, err);
                std::process::exit(1);
            }
        };

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);
        // debug_data.print();

        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            debug_data,
            breakpoints: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Run(args) => {
                    if let Some(inf) = self.inferior.as_mut() {
                        inf.kill(); // you pause an inferior using ctrl+c, then type run
                        self.inferior = None;
                    } 
                    if let Some(mut inferior) = Inferior::new(&self.target, &args) {
                        // Create the inferior
                        for (addr, orig_byte) in &mut self.breakpoints {    // set breakpoints
                            *orig_byte = inferior.write_byte(*addr, 0xcc).unwrap();
                        }

                        self.inferior = Some(inferior);
                        // You may use self.inferior.as_mut().unwrap() to get a mutable reference
                        // to the Inferior object
                        let inf = self.inferior.as_mut().unwrap();
                        if let Ok(status) = inf.cont(&self.breakpoints) {
                            match status {
                                inferior::Status::Exited(ecode) => {
                                    println!("Child exited (status {})", ecode);
                                    self.inferior = None;
                                }
                                inferior::Status::Signaled(signal) => {
                                    println!("Child exited (signal {})", signal);
                                    self.inferior = None;
                                }
                                inferior::Status::Stopped(signal, ip) => {
                                    if let Some(line) = self.debug_data.get_line_from_addr(ip) {
                                        println!("Child stopped (signal {})", signal);
                                        println!("Stopped at {}", line);
                                    }
                                }
                            }
                        }
                    } else {
                        println!("Error starting subprocess");
                    }
                }
                DebuggerCommand::Quit => {
                    if let Some(inf) = self.inferior.as_mut() {
                        inf.kill(); // if you exit DEET while a process is paused
                    }
                    return;
                }
                DebuggerCommand::Continue => {
                    if let Some(inf) = self.inferior.as_mut() {
                        // inf.cont(&self.breakpoints).unwrap();
                        if let Ok(status) = inf.cont(&self.breakpoints) {
                            match status {
                                inferior::Status::Exited(ecode) => {
                                    println!("Child exited (status {})", ecode);
                                    self.inferior = None;
                                },
                                inferior::Status::Signaled(signal) => {
                                    println!("Child stopped (signal {})", signal);
                                    self.inferior = None;
                                },
                                inferior::Status::Stopped(signal, ip) => {
                                    if let Some(line) = self.debug_data.get_line_from_addr(ip) {
                                        println!("Child stopped (signal {})", signal);
                                        println!("Stopped at {}", line);
                                    }
                                }
                            }
                        }
                    } else {
                        println!("no inferior running");    // if you type continue before you type run
                    }
                }
                DebuggerCommand::Backtrace => {
                    if let Some(inf) = self.inferior.as_mut() {
                        inf.print_backtrace(&self.debug_data).unwrap();
                    } else {
                        println!("no inferior running");
                    }
                }
                DebuggerCommand::Break(brkp) => {
                    let b;
                    if brkp.starts_with("*") {
                        if let Some(addr) = parse_address(&brkp[1..]) {
                            b = addr;
                        } else {
                            println!("invalid address");
                            continue;
                        }
                    } else if let Ok(line_num) = usize::from_str_radix(&brkp, 10) {
                        dbg!(line_num);
                        if let Some(addr) = self.debug_data.get_addr_for_line(None, line_num) {
                            b = addr;
                        } else {
                            println!("invalid line number");
                            continue;
                        }
                    } else if let Some(addr) = self.debug_data.get_addr_for_function(None, &brkp) {
                        b = addr;
                    } else {
                        println!("Usage: b|break|breakpoint *address|line|func");
                        continue;
                    }

                    println!("Set breakpoint {} at {}", self.breakpoints.len(), b);

                    if let Some(inf) = self.inferior.as_mut() {     // after run
                        let orig_byte = inf.write_byte(b, 0xcc).unwrap();
                        self.breakpoints.insert(b, orig_byte);
                    } else {
                        self.breakpoints.insert(b, 0);  // before run
                    }
                }
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }
}
