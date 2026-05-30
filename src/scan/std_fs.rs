use crate::filetree::{FileNode, FileTree, NodeId};
use crate::scan::scanner::Scanner;
use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};

pub struct StdFsScanner {}

impl StdFsScanner {
    fn filename(path: &Path) -> String {
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

    fn folder_size(&self, folder: NodeId, tree: &mut FileTree) -> u64 {
        let children = match &tree.nodes[folder] {
            FileNode::Dir { children, .. } => children.clone(),
            _ => unreachable!(),
        };

        let mut total_size: u64 = 0;
        for child in children {
            total_size += match &tree.nodes[child] {
                FileNode::File { size, .. } => *size,
                FileNode::Dir {
                    size: Some(size), ..
                } => *size,
                FileNode::Dir { size: None, .. } => self.folder_size(child, tree),
                FileNode::Symlink { .. } => 0,
            }
        }

        if let FileNode::Dir { size, .. } = &mut tree.nodes[folder] {
            *size = Some(total_size);
        }

        total_size
    }
}

impl Scanner for StdFsScanner {
    fn scan_path(&self, path: &Path) -> std::io::Result<FileTree> {
        let path = path.canonicalize()?;
        let root_name = Self::filename(&path);
        let root = FileNode::dir(root_name, None, None, vec![]);
        let root_path = path.to_str().expect("Invalid root path!").to_string() + "/";
        let mut tree = FileTree::new(root, root_path);

        let mut buf: VecDeque<(NodeId, PathBuf)> = VecDeque::new();
        buf.push_back((0, path));
        while let Some((node_id, path)) = buf.pop_front() {
            let Ok(iter) = fs::read_dir(&path) else {
                println!("Error enumerating \"{}\"", path.to_str().unwrap());
                continue;
            };
            for entry in iter {
                let entry = entry?;
                let path = entry.path();
                let name = Self::filename(&path);
                let index = tree.nodes.len();
                match tree.nodes.get_mut(node_id).unwrap() {
                    FileNode::Dir { children, .. } => children.push(index),
                    _ => unreachable!(),
                }

                if path.is_symlink() {
                    let path = path.read_link()?;
                    let target = path
                        .into_os_string()
                        .into_string()
                        .expect("Invalid Symlik Path!");
                    let node = FileNode::symlink(name, node_id, target);
                    tree.nodes.push(node);
                } else if path.is_dir() {
                    let node = FileNode::dir(name, Some(node_id), None, vec![]);
                    buf.push_back((index, path));
                    tree.nodes.push(node);
                } else if path.is_file() {
                    let size = path.metadata()?.len();
                    let node = FileNode::file(name, node_id, size);
                    tree.nodes.push(node);
                } else {
                    println!("Unknown File Type: {:?}, Skipping!", path);
                }
            }
        }

        self.folder_size(0, &mut tree);

        Ok(tree)
    }

    fn name(&self) -> &'static str {
        "Standard Scanner"
    }
}
