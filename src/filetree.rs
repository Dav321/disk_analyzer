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

    pub fn children(&self, folder_id: NodeId) -> Vec<NodeId> {
        let FileNode::Dir { children, .. } = &self.nodes[folder_id] else {
            unreachable!("Folder ID does not reference a folder")
        };

        let mut children = children.to_owned();
        children.sort_by(|n1, n2| {
            let n1 = self.nodes[*n1].size().unwrap_or(0);
            let n2 = self.nodes[*n2].size().unwrap_or(0);
            n2.cmp(&n1)
        });
        children
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
        children: Vec<NodeId>,
    },
}

impl FileNode {
    pub fn file(name: String, parent: NodeId, size: u64) -> Self {
        Self::File { name, parent, size }
    }

    pub fn dir(
        name: String,
        parent: Option<NodeId>,
        size: Option<u64>,
        children: Vec<NodeId>,
    ) -> Self {
        Self::Dir {
            name,
            parent,
            size,
            children,
        }
    }

    pub fn name(&self) -> String {
        match self {
            FileNode::File { name, .. } => name.to_owned(),
            FileNode::Dir { name, .. } => name.to_owned(),
        }
    }

    pub fn size(&self) -> Option<u64> {
        match self {
            FileNode::File { size, .. } => Some(*size),
            FileNode::Dir { size, .. } => *size,
        }
    }

    pub fn size_str(&self) -> String {
        const SIZES: [(u64, &str); 6] = [
            (1024u64.pow(5), "PiB"),
            (1024u64.pow(4), "TiB"),
            (1024u64.pow(3), "GiB"),
            (1024u64.pow(2), "MiB"),
            (1024u64, "KiB"),
            (0u64, "B"),
        ];

        if let Some(size) = self.size() {
            for (min, trail) in SIZES {
                if size >= min {
                    return format!("{:.2} {}", size / min.max(1), trail);
                }
            }
        }

        "".to_owned()
    }
}

impl Display for FileNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileNode::File { name, size, .. } => write!(f, "File \"{}\" ({})", name, size),
            FileNode::Dir { name, .. } => {
                write!(f, "Dir \"{}\"", name)
            }
        }
    }
}
