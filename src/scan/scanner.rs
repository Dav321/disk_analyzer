use crate::filetree::FileTree;
use std::path::PathBuf;

pub trait Scanner {
    fn scan_path(&self, path: PathBuf) -> std::io::Result<FileTree>;
    fn name(&self) -> &'static str;
}