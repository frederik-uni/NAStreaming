use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::Path;

#[derive(Debug, Default)]
struct PathNode {
    children: BTreeMap<OsString, PathNode>,
}

#[derive(Debug, Default)]
pub struct PathTree {
    root: PathNode,
}
//TODO: add file size(check if replaced)
impl PathTree {
    /// Inserts a path into the tree.
    pub fn insert(&mut self, path: &Path) {
        let mut node = &mut self.root;
        for part in path.iter().map(|s| s.to_owned()) {
            node = node.children.entry(part).or_default();
        }
    }

    /// Checks if a path exists in the tree.
    pub fn contains(&self, path: &Path) -> bool {
        let mut node = &self.root;
        for part in path.iter() {
            if let Some(next) = node.children.get(part) {
                node = next;
            } else {
                return false;
            }
        }
        true
    }
}
