use std::cell::RefCell;

// #[derive(Debug, Default)]
pub struct File {
    name: String,
}

// #[derive(Debug, Default)]
pub struct Folder {
    name: String,
    entries: Vec<Entry>,
}

// #[derive(Debug)]
pub enum Entry {
    File(File),
    Folder(Folder),
}

pub struct EntryIter<'a> {
    iter_stack: RefCell<Vec<std::slice::Iter<'a, Entry>>>,
    path: RefCell<Vec<&'a Entry>>,
}

impl<'a> Iterator for &EntryIter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter_stack = self.iter_stack.borrow_mut();
        let mut path = self.path.borrow_mut();
        loop {
            if let Some(Entry::File(_)) = path.last() {
                // Pop last element from child folder if a file
                path.pop();
            }
            match iter_stack.last_mut().and_then(|x| x.next()) {
                Some(entry) => {
                    if let Entry::Folder(folder) = entry {
                        iter_stack.push(folder.entries.iter());
                    }
                    path.push(entry);
                    return Some(entry);
                }
                None => {
                    iter_stack.pop();
                    path.pop(); // Pop folder itself
                    if iter_stack.is_empty() {
                        return None;
                    }
                }
            }
        }
    }
}

impl<'a> EntryIter<'a> {
    pub fn get_path(&self, separator: &str) -> String {
        self.path
            .borrow()
            .iter()
            .map(|entry| match entry {
                Entry::File(f) => f.name.to_owned(),
                Entry::Folder(f) => f.name.to_owned(),
            })
            .collect::<Vec<String>>()
            .join(separator)
    }

    pub fn find_file(&self, filename: &str, separator: &str) -> Option<String> {
        for iter in self {
            if let Entry::File(f) = iter {
                if f.name == filename {
                    return Some(self.get_path(separator));
                }
            }
        }
        None
    }
}

pub struct Content {
    entries: Vec<Entry>,
}

impl Content {
    pub fn iter(&self) -> EntryIter<'_> {
        EntryIter {
            iter_stack: RefCell::new(vec![self.entries.iter()]),
            path: RefCell::new(vec![]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simple_test() {
        let content = get_content();
        let mut output = vec![];
        let ci = content.iter();
        for x in &ci {
            output.push(format!("{}", ci.get_path("/")));
            if let Entry::File(f) = x {
                if f.name == "b.txt" {
                    output.push(format!("b.txt found"));
                }
            }
        }
        let expected = vec![
            "a",
            "a/c.txt",
            "a/d",
            "a/d/e.txt",
            "a/b.txt",
            "b.txt found",
            "a/i",
            "a/f.txt",
            "g.txt",
            "h",
        ];
        assert_eq!(output.len(), expected.len());
        for (l, r) in output.iter().zip(expected.iter()) {
            assert_eq!(l, r);
        }
    }

    #[test]
    fn find_test() {
        assert_eq!(
            get_content().iter().find_file("b.txt", "/"),
            Some("a/b.txt".to_owned())
        );
        assert_eq!(get_content().iter().find_file("h", "/"), None);
    }

    fn get_content() -> Content {
        let content = Content {
            entries: vec![
                Entry::Folder(Folder {
                    name: "a".to_owned(),
                    entries: vec![
                        Entry::File(File {
                            name: "c.txt".to_owned(),
                        }),
                        Entry::Folder(Folder {
                            name: "d".to_owned(),
                            entries: vec![Entry::File(File {
                                name: "e.txt".to_owned(),
                            })],
                        }),
                        Entry::File(File {
                            name: "b.txt".to_owned(),
                        }),
                        Entry::Folder(Folder {
                            name: "i".to_owned(),
                            entries: vec![],
                        }),
                        Entry::File(File {
                            name: "f.txt".to_owned(),
                        }),
                    ],
                }),
                Entry::File(File {
                    name: "g.txt".to_owned(),
                }),
                Entry::Folder(Folder {
                    name: "h".to_owned(),
                    entries: vec![],
                }),
            ],
        };
        content
    }
}
