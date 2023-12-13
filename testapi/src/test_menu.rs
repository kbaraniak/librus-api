/* Test Menu */

use std::io::{self, Write};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn display_menu() {
    println!("Menu:");
    println!("1 - Option 1");
    println!("2 - Option 2");
    println!("3 - Option 3");
    println!("4 - Option 4");
    println!("5 - Option 5");
    println!("0 - Exit");
}

fn main() {
    let stdin = io::stdin();
    display_menu();
    let mut stdout = io::stdout().into_raw_mode().unwrap();


    for c in stdin.keys() {
        match c.unwrap() {
            termion::event::Key::Char('1') => {
                println!("Option 1");
                break;
            }
            termion::event::Key::Char('2') => {
                println!("Option 2");
                break;
            }
            termion::event::Key::Char('3') => {
                println!("Option 3");
                break;
            }
            termion::event::Key::Char('4') => {
                println!("Option 4");
                break;
            }
            termion::event::Key::Char('5') => {
                println!("Option 5");
                display_menu();
                break;
            }
            termion::event::Key::Char('0') => {
                println!("Exiting the program...");
                return;
            }
            _ => (),
        }
    }
    print!("\x1B[2J\x1B[1;1H"); // Clear the console screen
    io::stdout().flush().unwrap();
}
