use std::path::PathBuf;

pub struct GitFinder {
    entries: Vec<String>,
    recursive: bool,
}

impl GitFinder {
    pub fn new(entries: Vec<String>, recursive: bool) -> GitFinder {
        GitFinder { entries, recursive }
    }
}

impl Iterator for GitFinder {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = match self.entries.pop() {
                Some(entry) => entry,
                _ => break,
            };

            let mut root = PathBuf::from(entry);
            let is_git = is_git_project(&mut root);

            if !is_git || self.recursive {
                self.entries
                    .extend(root.as_path().read_dir().ok()?.filter_map(|entry| {
                        let entry = entry.ok()?.path();
                        if !entry.ends_with(".git") && entry.is_dir() {
                            entry.to_str().map(String::from)
                        } else {
                            None
                        }
                    }));
            }

            if is_git {
                return root.to_str().and_then(|x| Some(x.to_owned()));
            }
        }

        None
    }
}

fn is_git_project(folder: &mut PathBuf) -> bool {
    folder.push(".git");
    let flg = folder.is_dir();
    folder.pop();
    flg
}
