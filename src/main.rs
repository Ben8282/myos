#![no_std]
#![no_main]

use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    uefi::system::with_stdout(|stdout| {
        let _ = stdout.clear();
    });

    uefi::println!("Welcome to MyOS!");
    uefi::println!("================");
    uefi::println!();
    uefi::println!("A minimal operating environment written in Rust.");
    uefi::println!("Architecture: x86_64");
    uefi::println!("Boot mode: UEFI");
    uefi::println!();
    uefi::println!("MyOS initialized successfully.");

    loop {
        core::hint::spin_loop();
    }
}