extern crate termion;
extern crate libc;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{BufRead, Write, stdout, stdin};
use std::fs::File;
use std::os::unix::io::IntoRawFd;
use std::cmp;

fn main() {
    let stdin = stdin();

    let input_data = {
        let mut input_data : Vec<String> = vec!();
        for line in stdin.lock().lines() {
            input_data.push(line.unwrap());
        }
        input_data
    };

    // Clearing stdin before calling into_raw_mode
    // https://github.com/ticki/termion/issues/64
    unsafe {
        let tty = File::open("/dev/tty").unwrap();
        libc::dup2(tty.into_raw_fd(), libc::STDIN_FILENO);
    }

    // Set raw mode to allow reading stdin
    let _stdout = stdout().into_raw_mode().unwrap();

    let mut tty = termion::get_tty().unwrap();

    write!(tty, "{}{}", termion::cursor::Hide, termion::clear::All).unwrap();
    let mut selected_option : usize = 0;

    update_display(&tty, &input_data, selected_option);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Up        => selected_option=selected_option.saturating_sub(1),
            Key::Down      => selected_option=cmp::min(selected_option.saturating_add(1), input_data.len()-1),
            Key::Char('\n') => {
                println!("{}", input_data.get(selected_option).unwrap());
                break;
            },
            _ => {}
        }
        update_display(&tty, &input_data, selected_option);
    }

    write!(tty, "{}", termion::cursor::Show).unwrap();
}

fn update_display(mut tty: &File, input_data: &Vec<String>, selected_option: usize) {
    for (index, item) in (&input_data).into_iter().enumerate() {
        let cursor = if selected_option == index {">"} else {" "};
        write!(tty, "{}{} {} {}",
               termion::cursor::Goto(0, index as u16 + 1),
               termion::clear::CurrentLine,
               cursor,
               item).unwrap();
    }
    write!(tty, "\n\r").unwrap();

    tty.flush().unwrap();
}
