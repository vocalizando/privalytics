use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;
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
        let path = fs::read_dir(path)?;

        let mut date_and_entry = HashMap::new();
        for entry in path {
            let path = entry?.path();
            let metadata = fs::metadata(&path)?;
            let creation_date = metadata.created()?.duration_since(UNIX_EPOCH)?.as_millis();

            if path.to_str().ok_or_else(|| Box::new(EmptyError))?.ends_with(".bson") {
                date_and_entry.insert(creation_date, Entry::load(&path)?);
            }
        }

        let mut keys = date_and_entry.keys().collect::<Vec<&u128>>();
        keys.sort();

        let mut entries = vec![];
        for key in keys {
            // FIXME: Do not use ``.clone()`` -> anti-pattern
            entries.push(date_and_entry.get(key).unwrap().clone())
        }

        Ok(entries)
    }
}
