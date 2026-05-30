use crate::app::App;
use crate::scan::scanner::Scanner;
use crate::scan::std_fs::StdFsScanner;
use std::path::PathBuf;
use std::{env, io};

mod app;
mod filetree;
mod scan;

fn main() -> io::Result<()> {
    println!("disk_analyzer {}", env!("CARGO_PKG_VERSION"));
    color_eyre::install().expect("Could not install color_eyre");

    let mut args = env::args();
    let _ = args.next();
    let path = args.next().unwrap_or(".".to_string());

    let scanner = StdFsScanner {};
    print!("Scanning with {}...", scanner.name());
    let tree = scanner.scan_path(&PathBuf::from(path))?;
    println!(" Done!");

    let mut app = App::new(tree);
    ratatui::run(|terminal| app.run(terminal))
}
