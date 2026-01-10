use std::cmp::Ordering;
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
            let get = |i| match &self.nodes[i] {
                FileNode::File { size, .. } => (false, *size),
                FileNode::Dir { size, .. } => (true, size.unwrap_or(0u64)),
            };
            let n1 = get(*n1);
            let n2 = get(*n2);

            if !n1.0 && !n2.0 {
                n1.1.cmp(&n2.1)
            } else if n1.0 && n2.0 {
                Ordering::Equal
            } else if n1.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
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

    pub fn size_str(&self) -> String {
        const SIZES: [(u64, &str); 6] = [
            (1024u64.pow(5), "PiB"),
            (1024u64.pow(4), "TiB"),
            (1024u64.pow(3), "GiB"),
            (1024u64.pow(2), "MiB"),
            (1024u64, "KiB"),
            (0u64, "B"),
        ];

        let size = match self {
            FileNode::File { size, .. } => Some(*size),
            FileNode::Dir { size, .. } => *size,
        };
        if let Some(size) = size {
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
