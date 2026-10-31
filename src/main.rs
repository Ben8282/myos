#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi::proto::console::text::Key;
use uefi::{boot, print, println, system};

#[entry]
fn main() -> Status {
    uefi::boot::set_watchdog_timer(0, 0x10000, None).unwrap();
    uefi::helpers::init().unwrap();

    uefi::system::with_stdout(|stdout| {
        let _ = stdout.clear();
    });

    println!("Welcome to MyOS!");
    println!("================");
    println!();
    println!("A minimal operating environment written in Rust.");
    println!("Architecture: x86_64");
    println!("Boot mode: UEFI");
    println!();
    println!("MyOS initialized successfully.");

    loop {
        // Sleep until a key is pressed, then read it
        let key = system::with_stdin(|stdin| {
            let mut events = [stdin.wait_for_key_event().unwrap()];
            boot::wait_for_event(&mut events).discard_errdata().unwrap();
            stdin.read_key().unwrap()
        });

        // Show the key on screen (Enter starts a new line)
        if let Some(Key::Printable(c)) = key {
            let ch = char::from(c);
            if ch == '\r' {
                println!();
            } else {
                print!("{}", ch);
            }
        }
    }
}