use std::fmt::{Debug, Display, Formatter};

pub type NodeId = usize;

#[derive(Debug)]
pub struct FileTree {
    pub nodes: Vec<FileNode>,
    pub root: NodeId,
}

impl FileTree {
    pub fn new(root: FileNode) -> Self {
        Self {
            nodes: vec![root],
            root: 0,
        }
    }

    fn print_tree(&self, f: &mut Formatter<'_>, node: NodeId, depth: usize) -> std::fmt::Result {
        let node = self.nodes.get(node).unwrap();

        write!(f, "{}{}\n", " ".repeat(depth), node)?;

        match node {
            FileNode::Dir { children, .. } => {
                for child in children {
                    self.print_tree(f, *child, depth + 2)?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

impl Display for FileTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.print_tree(f, self.root, 0)
    }
}

#[derive(Debug)]
pub enum FileNode {
    File {
        name: String,
        parent: NodeId,
        size: u64,
    },
    Dir {
        name: String,
        parent: Option<NodeId>,
        size: Option<u64>,
        children: Vec<NodeId>
    }
}

impl FileNode {
    pub fn file(name: String, parent: NodeId, size: u64) -> Self {
        Self::File {
            name,
            parent,
            size
        }
    }

    pub fn dir(name: String, parent: Option<NodeId>, size: Option<u64>, children: Vec<NodeId>) -> Self {
        Self::Dir {
            name,
            parent,
            size,
            children
        }
    }
}

impl Display for FileNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileNode::File { name, size, .. } =>
                write!(f, "File \"{}\" ({})", name, size),
            FileNode::Dir { name, .. } => {
                write!(f, "Dir \"{}\"", name)
            }
        }
    }
}