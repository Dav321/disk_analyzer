use crate::app::App;
use crate::scan::scanner::Scanner;
use crate::scan::std_fs::StdFsScanner;
use std::io;
use std::path::PathBuf;

mod filetree;
mod scan;
mod app;

fn main() -> io::Result<()> {
    color_eyre::install().expect("Could not install color_eyre");
    println!("disk_analyzer");

    let scanner = StdFsScanner{};
    print!("Scanning with {}...", scanner.name());
    let tree = scanner.scan_path(PathBuf::from("."))?;
    println!(" Done!");

    let mut app = App::new(tree);
    ratatui::run(|terminal| app.run(terminal))
}