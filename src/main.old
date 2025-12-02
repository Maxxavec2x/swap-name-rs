use clap::Parser;
use std::ffi::CString;
use std::fs::{File, canonicalize};
use std::os::unix::io::AsRawFd;
use syscalls::{Sysno, syscall};
// SWAP NAME RS : The goal of this code is to swap the name of two file without changing their
// content.

const RENAME_EXCHANGE: u32 = 2;

#[derive(Parser)]
struct Cli {
    first_file: std::path::PathBuf,
    second_file: std::path::PathBuf,
}

impl Cli {
    // On part du principe que les fichiers sont dans le même directory
    fn get_dir_descriptor(&self) -> File {
        let realpath1 = canonicalize(&self.first_file).expect("Cannot resolve first file path");
        let realpath2 = canonicalize(&self.second_file).expect("Cannot resolve second file path");
        let path1 = realpath1
            .parent()
            .expect("No parent for first file (root ?)");
        let path2 = realpath2
            .parent()
            .expect("No parent for second file (root ?)");
        //println!("First path : {:?}, Second path :{:?}", path1, path2);
        File::open(path1).expect("Could not open first dir")
    }

    fn get_path_string(&self) -> (CString, CString) {
        let name1 = self.first_file.file_name().unwrap();
        let name2 = self.second_file.file_name().unwrap();

        //println!("Name 1 : {:?}, name2 : {:?}", name1, name2);
        (
            CString::new(name1.to_str().unwrap())
                .expect("Cannot create the C string from first file name"),
            CString::new(name2.to_str().unwrap())
                .expect("Cannot create the C string from second file name"),
        )
    }
}

fn main() {
    let args: Cli = Cli::parse();
    swap_file(&args);
}

fn display(cli: &Cli) {
    println!(
        "{:?} is renamed as {:?}, and {:?} is renamed as {:?}",
        cli.first_file, cli.second_file, cli.second_file, cli.first_file
    );
}

fn swap_file(cli: &Cli) {
    let (fd1, fd2) = cli.get_path_string();
    let fdir = cli.get_dir_descriptor();

    match unsafe {
        syscall!(
            Sysno::renameat2,
            fdir.as_raw_fd(),
            fd1.as_ptr(),
            fdir.as_raw_fd(),
            fd2.as_ptr(),
            RENAME_EXCHANGE
        )
    } {
        Ok(result) => {
            display(cli);
        }
        Err(err) => {
            panic!("renameat failed: {}", err);
        }
    }
}
