use crate::scan::scanner::Scanner;
use std::path::PathBuf;
use crate::scan::std_fs::StdFsScanner;

mod filetree;
mod scan;

fn main() {
    println!("disk_analyzer");
    let scanner = StdFsScanner{};

    let tree = scanner.scan_path(PathBuf::from(".")).unwrap();

    println!("{}", tree)
}
