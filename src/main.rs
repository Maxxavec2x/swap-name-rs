use clap::Parser;
use libc::{AT_FDCWD, RENAME_EXCHANGE}; // Ca permet d'utiliser les constantes de la libc
use std::ffi::CString;
// SWAP NAME RS : The goal of this code is to swap the name of two file without changing their
// content.

#[derive(Parser)]
struct Cli {
    first_file: String,
    second_file: String,
}

fn main() {
    let args: Cli = Cli::parse();
    swap_file(&args);
}

fn swap_file(cli: &Cli) {
    let realpath1 = std::fs::canonicalize(&cli.first_file).expect("Cannot resolve first file path"); // Permet de recup le full path
    let realpath2 =
        std::fs::canonicalize(&cli.second_file).expect("Cannot resolve second file path");

    let path1_cstr = CString::new(realpath1.to_str().unwrap()).expect("Cannot create CString"); // créé une CString (litterallement une string C-like ptdr)
    let path2_cstr = CString::new(realpath2.to_str().unwrap()).expect("Cannot create CString");

    let result = unsafe {
        libc::renameat2(
            AT_FDCWD,
            path1_cstr.as_ptr(),
            AT_FDCWD,
            path2_cstr.as_ptr(),
            RENAME_EXCHANGE,
        )
    };

    if result == 0 {
    } else {
        panic!("renameat2 failed: {}", std::io::Error::last_os_error());
    }
}
