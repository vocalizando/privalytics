use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::Path;
use crate::structures::entry::Entry;

struct EmptyError;

impl Debug for EmptyError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Display for EmptyError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Error for EmptyError {}

impl Entry {
    pub fn save<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<(), Box<dyn Error>> {
        let serialized = bson::to_bson(self)?;
        let document = serialized.as_document().unwrap();

        let mut writer: Vec<u8> = Vec::new();
        document.to_writer(&mut writer)?;

        fs::write(path, writer)?;

        Ok(())
    }

    pub fn load<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Entry, Box<dyn Error>> {
        let bytes = fs::read(path)?;
        let document = bson::Document::from_reader(&mut bytes.as_slice())?;
        let entry: Entry = bson::from_document(document)?;

        Ok(entry)
    }

    pub fn load_entries<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Vec<Entry>, Box<dyn Error>> {
        let path = fs::read_dir(path).unwrap();

        let mut entries = vec![];
        for entry in path {
            let path = entry?.path();

            if path.to_str().ok_or_else(|| Box::new(EmptyError))?.ends_with(".bson") {
                entries.push(Entry::load(&path)?);
            }
        }

        Ok(entries)
    }
}
