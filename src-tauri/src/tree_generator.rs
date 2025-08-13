use std::collections::HashSet;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

fn is_ignored(entry: &DirEntry, ignore_list: &HashSet<String>) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| ignore_list.contains(s))
        .unwrap_or(false)
}

/// Generates a visual tree structure for a given directory.
pub fn generate_folder_tree(directory: &str, ignore_list: &HashSet<String>) -> Result<String, String> {
    let start_path = PathBuf::from(directory);
    if !start_path.is_dir() {
        return Err(format!("Error: Path '{}' is not a valid directory.", directory));
    }

    let root_name = start_path
        .file_name()
        .unwrap_or(start_path.as_os_str())
        .to_string_lossy();

    let mut tree_string = format!("🗂️ {}/\n", root_name);

    // This iterator will be filtered to skip ignored directories entirely.
    let walker = WalkDir::new(&start_path)
        .min_depth(1)
        .sort_by_file_name();

    // We build a list of entries first to determine which ones are "last" in their directory,
    // which is needed for the '└──' character. This is more complex but gives the correct visual output.
    let mut entries: Vec<DirEntry> = Vec::new();
    for entry in walker.into_iter().filter_entry(|e| !is_ignored(e, ignore_list)) {
        match entry {
            Ok(e) => entries.push(e),
            Err(_) => continue,
        }
    }

    // A little trick to find the last entry at each depth level.
    let mut last_entry_at_depth = std::collections::HashMap::new();
    for entry in &entries {
        last_entry_at_depth.insert(entry.depth(), entry.path().to_path_buf());
    }

    for entry in &entries {
        let depth = entry.depth();
        let mut prefix = String::new();

        // Build the prefix (e.g., "│   ├── ")
        for i in 1..depth {
             // Check if any ancestor was the last in its directory
            let parent_path = entry.path().ancestors().nth(depth - i).unwrap();
            if let Some(last_path) = last_entry_at_depth.get(&i) {
                if last_path == parent_path {
                     prefix.push_str("    ");
                } else {
                     prefix.push_str("│   ");
                }
            } else {
                 prefix.push_str("│   ");
            }
        }
        
        let is_last = last_entry_at_depth.get(&depth) == Some(&entry.path().to_path_buf());
        let connector = if is_last { "└── " } else { "├── " };
        let icon = if entry.file_type().is_dir() { "📁" } else { "📄" };
        
        let file_name = entry.file_name().to_string_lossy();
        tree_string.push_str(&format!("{}{} {} {}\n", prefix, connector, icon, file_name));
    }

    Ok(tree_string)
}