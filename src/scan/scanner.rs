use crate::filetree::FileTree;
use std::path::Path;

pub trait Scanner {
    fn scan_path(&self, path: &Path) -> std::io::Result<FileTree>;
    fn name(&self) -> &'static str;
}
