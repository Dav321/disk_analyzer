use crate::filetree::{FileNode, FileTree, NodeId};
use crate::scan::scanner::Scanner;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

pub struct StdFsScanner {}

impl StdFsScanner {
    fn filename(path: &PathBuf) -> String {
        let file_name = path.file_name();
        if file_name.is_none() {
            return "".to_string();
        }
        file_name
            .unwrap()
            .to_owned()
            .into_string()
            .expect("Invalid Path!")
    }
}

impl Scanner for StdFsScanner {
    fn scan_path(&self, path: PathBuf) -> std::io::Result<FileTree> {
        let root_name = Self::filename(&path);
        let root = FileNode::dir(root_name, None, None, vec![]);
        let mut tree = FileTree::new(root);

        let mut buf: VecDeque<(NodeId, PathBuf)> = VecDeque::new();
        buf.push_back((0, path));
        while let Some((node_id, path)) = buf.pop_front() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                let name = Self::filename(&path);
                let index = tree.nodes.len();
                match tree.nodes.get_mut(node_id).unwrap() {
                    FileNode::Dir { children, .. } => children.push(index),
                    _ => unreachable!(),
                }

                if path.is_dir() {
                    let node = FileNode::dir(name, Some(node_id), None, vec![]);
                    buf.push_back((index, path));
                    tree.nodes.push(node);
                } else if path.is_file() {
                    let size = path.metadata()?.len();
                    let node = FileNode::file(name, node_id, size);
                    tree.nodes.push(node);
                } else if path.is_symlink() {
                    unimplemented!("Symlinks are not implemented!");
                } else {
                    unimplemented!("Unknown File Type: {:?}", path)
                }
            }
        }

        Ok(tree)
    }

    fn name(&self) -> &'static str {
        "Standard Scanner"
    }
}
