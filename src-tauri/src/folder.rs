use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
pub struct TreeNode {
    id: String,
    name: String,
    children: Option<Vec<TreeNode>>,
}

#[tauri::command]
pub fn read_folder(path: String) -> Result<Vec<TreeNode>, String> {
    let root_path = Path::new(&path);
    if root_path.exists() && root_path.is_dir() {
        let mut nodes = Vec::new();
        build_tree(root_path, &mut nodes, root_path.display().to_string());
        Ok(nodes)
    } else {
        Err("Invalid directory path".to_string())
    }
}

fn build_tree(path: &Path, nodes: &mut Vec<TreeNode>, parent_id: String) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let file_name = entry.file_name().into_string().unwrap();
                let id = format!("{}/{}", parent_id, file_name);
                if path.is_dir() {
                    let mut children = Vec::new();
                    build_tree(&path, &mut children, id.clone());
                    nodes.push(TreeNode {
                        id,
                        name: file_name,
                        children: Some(children),
                    });
                } else {
                    nodes.push(TreeNode {
                        id,
                        name: file_name,
                        children: None,
                    });
                }
            }
        }
    }
}
