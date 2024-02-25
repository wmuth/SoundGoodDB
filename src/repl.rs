use std::io::{self, Write};

use crate::{
    controller::{Command, ControlError, ControlResult, Controller},
    parser::{self, ParseResult},
};

const COMMAND_STRING: &str = "Commands: (is optional) [is required]\n\
                              Begin:\t\tb(egin)\n\
                              Commit:\t\tc(ommit)\n\
                              Help:\t\th(elp)\n\
                              List:\t\tl(ist) (instrument_type)\n\
                              Quit:\t\tq(uit)\n\
                              Rent:\t\tre(nt) [student] [instrument]\n\
                              Rollback:\tro(llback)\n\
                              Terminate:\tt(erminate) [student] [instrument]";

/// Starts the read-evaluate-print-loop
///
/// Prints the welcome, available commands, prompt and takes in input from the user.
/// The input is parsed by [`parser::parse_to_command`] and the result is run on the controller
/// unless it is of type help or quit which are caught here since they effect the view.
///
/// # Parameters
/// - `con` mutable refernce to the controller which acts as the "parent" to this repl view
pub async fn repl<'a>(con: &mut Controller<'a>) {
    let mut input = String::new();
    println!("Welcome to the 🎵 Soundgood Music School Database Program 🎵");
    println!("{COMMAND_STRING}");

    loop {
        print!("\n🎵>>> ");
        flush_and_read(&mut input);

        match parser::parse_to_command(&input) {
            Ok(r) => match r {
                ParseResult::Help => println!("{COMMAND_STRING}"),
                ParseResult::Quit => break,
                ParseResult::Command(c) => match c {
                    Command::TryTerminate(u, i) => handle_terminate(con, u, i).await,
                    _ => match con.execute(c).await {
                        Ok(r) => print_control_result(r),
                        Err(e) => eprintln!("{e}"),
                    },
                },
            },
            Err(e) => eprintln!("{e}"),
        }

        input.clear();
    }
}

async fn handle_terminate<'a>(con: &mut Controller<'a>, user: String, inst: String) {
    let result = con.execute(Command::TryTerminate(user, inst)).await;
    match result {
        Ok(r) => print_control_result(r),
        Err(e) => match e {
            ControlError::TerminateMultiple(ref vec) => {
                eprintln!("{e}");
                println!("Please pick one from the following list:");
                for row in vec {
                    println!("{row}");
                }

                print!("ID to terminate: ");
                let mut input = String::new();
                flush_and_read(&mut input);

                let res = con.execute(Command::Terminate(input.trim().into())).await;
                match res {
                    Ok(cr) => print_control_result(cr),
                    Err(e) => eprintln!("{e}"),
                }
            }
            _ => eprintln!("{e}"),
        },
    }
}

fn print_control_result(cr: ControlResult) {
    match cr {
        ControlResult::Begin => println!("Begun new transaction!"),
        ControlResult::Commit => println!("Commited!"),
        ControlResult::List(v) => v.iter().for_each(|i| println!("{i}")),
        ControlResult::Rent(r) => print_rows("Rented!", r),
        ControlResult::Rollback => println!("Rolled back!"),
        ControlResult::Terminate(r) | ControlResult::TryTerminate(r) => {
            print_rows("Terminated!", r);
        }
    }
}

fn flush_and_read(buf: &mut String) {
    io::stdout().flush().expect("Could not flush stdout!");
    io::stdin()
        .read_line(buf)
        .expect("Could not read from stdin!");
}

fn print_rows(s: &str, n: u64) {
    println!("{s} {n} rows affected!");
}
