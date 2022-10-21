use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct File {
    name: String,
}

#[derive(Debug, Default)]
pub struct Folder {
    name: String,
    entries: Vec<Entry>,
}

#[derive(Debug)]
pub enum Entry {
    File(File),
    Folder(Folder),
}

pub struct EntryIter<'a> {
    path: Vec<std::slice::Iter<'a, Entry>>,
}

impl<'a> Iterator for EntryIter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        match self.path.last_mut().and_then(|x| x.next()) {
            Some(entry) => {
                if let Entry::Folder(folder) = entry {
                    self.path.push(folder.entries.iter());
                }
                Some(entry)
            }
            None => {
                self.path.pop();
                self.path.last_mut().and_then(|x| x.next())
            }
        }
    }
}

impl<'a> EntryIter<'a> {
    pub fn get_path(&self, separator: &str) -> String {
        "".to_owned()
    }
}

pub struct Content {
    entries: Vec<Entry>,
}

impl Content {
    pub fn iter(&self) -> EntryIter<'_> {
        EntryIter {
            path: vec![self.entries.iter()],
        }
    }
}

fn main() {
    let content = Content {
        entries: vec![Entry::Folder(Folder {
            name: "a".to_owned(),
            entries: vec![
                Entry::File(File {
                    name: "c".to_owned(),
                }),
                Entry::Folder(Folder {
                    name: "d".to_owned(),
                    entries: vec![Entry::File(File {
                        name: "e".to_owned(),
                    })],
                }),
                Entry::File(File {
                    name: "b".to_owned(),
                }),
                Entry::File(File {
                    name: "f".to_owned(),
                }),
            ],
        })],
    };
    for x in content.iter() {
        println!(
            "x {}",
            match &x {
                Entry::File(f) => &f.name,
                Entry::Folder(f) => &f.name,
            }
        );
        if let Entry::File(f) = x {
            if f.name == "b" {
                println!("Yeah");
            }
        }
    }
}
