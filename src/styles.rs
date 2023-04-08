use std::{fs, path::PathBuf};

use crate::error::Error;

#[derive(Debug, Clone, PartialEq)]
enum EntryType {
    Style,
    Vocab,
    Rule,
}

#[derive(Debug, Clone)]
struct PathEntry {
    name: String,
    size: usize,
    path: PathBuf,
    kind: EntryType,
}

#[derive(Debug)]
struct StylesPath {
    root: PathBuf,
}

impl StylesPath {
    fn new(root: PathBuf) -> StylesPath {
        StylesPath { root }
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.root = path;
    }

    pub fn path(&self) -> PathBuf {
        self.root.clone()
    }

    pub fn add_to_accept(&self, name: &str, term: &str) -> Result<(), Error> {
        self.add_to_vocab(name, term, true)
    }

    pub fn add_to_reject(&self, name: &str, term: &str) -> Result<(), Error> {
        self.add_to_vocab(name, term, false)
    }

    fn add_to_vocab(&self, name: &str, term: &str, accept: bool) -> Result<(), Error> {
        let mut path = self.root.join("Vocab").join(name);

        if accept {
            path = path.join("accept.txt");
        } else {
            path = path.join("reject.txt");
        }

        let content = fs::read_to_string(path.clone())?;
        let mut lines = content.lines().collect::<Vec<_>>();

        lines.push(term);
        lines.sort();

        let content = lines.join("\n");
        fs::write(path, content)?;

        Ok(())
    }

    pub fn count(&self, kind: EntryType) -> Result<usize, Error> {
        let idx = self.index()?;
        Ok(idx.iter().filter(|e| e.kind == kind).count())
    }

    pub fn get(&self, kind: EntryType, name: &str) -> Result<PathEntry, Error> {
        let idx = self.index()?;

        let entry = idx
            .iter()
            .find(|e| e.kind == kind && e.name == name)
            .ok_or(Error::Msg("Not found".to_string()))?;

        Ok(entry.clone())
    }

    fn index(&self) -> Result<Vec<PathEntry>, Error> {
        let subdirs = fs::read_dir(self.path())?;
        let mut entries = Vec::new();

        for path in subdirs {
            let subdir = path?;
            let path = subdir.path();

            let dir_name = self.entry_name(path.clone());
            if dir_name == ".vale-config" {
                continue;
            } else if dir_name == "Vocab" && path.is_dir() {
                entries.append(&mut self.index_dir(path.clone(), EntryType::Vocab)?);
            } else if path.is_dir() {
                entries.push(PathEntry {
                    name: dir_name,
                    size: fs::read_dir(path.clone()).unwrap().count(),
                    path: path.clone(),
                    kind: EntryType::Style,
                });
                entries.append(&mut self.index_dir(path.clone(), EntryType::Rule)?);
            }
        }

        Ok(entries)
    }

    fn entry_name(&self, path: PathBuf) -> String {
        path.file_name()
            .unwrap_or("".as_ref())
            .to_string_lossy()
            .to_string()
    }

    fn index_dir(&self, path: PathBuf, kind: EntryType) -> Result<Vec<PathEntry>, Error> {
        let mut entries = vec![];

        fs::read_dir(path)?
            .into_iter()
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap().path())
            .for_each({
                |path| {
                    let ext = path.extension().unwrap_or("".as_ref());
                    if ext == "yml" || (path.is_dir() && kind == EntryType::Vocab) {
                        entries.push(PathEntry {
                            name: self.entry_name(path.clone()),
                            size: 0,
                            path: path.clone(),
                            kind: kind.clone(),
                        });
                    }
                }
            });

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STYLES: &str = ".github/styles";

    #[test]
    fn index() {
        let p = StylesPath::new(PathBuf::from(STYLES));

        assert_eq!(p.count(EntryType::Style).unwrap(), 2);
        assert_eq!(p.count(EntryType::Rule).unwrap(), 8);
        assert_eq!(p.count(EntryType::Vocab).unwrap(), 1);

        let style = p.get(EntryType::Style, "Test").unwrap();
        assert_eq!(style.name, "Test");
        assert_eq!(style.size, 1);
    }
}