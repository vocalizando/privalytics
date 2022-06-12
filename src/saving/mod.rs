use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use bson::Bson::Document;
use crate::structures::analytics::Entry;

pub trait EntrySave {
    fn save<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<(), Box<dyn Error>>;
}

pub trait EntryLoad {
    fn load<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Entry, Box<dyn Error>>;
}

impl EntrySave for Entry {
    fn save<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<(), Box<dyn Error>> {
        let serialized = bson::to_bson(self)?;
        let document = serialized.as_document().unwrap();

        let mut writer: Vec<u8> = Vec::new();
        document.to_writer(&mut writer)?;

        fs::write(path, writer)?;

        Ok(())
    }
}

impl EntryLoad for Entry {
    fn load<P: AsRef<Path> + ?Sized>(path: &P) -> Result<Entry, Box<dyn Error>> {
        let bytes = fs::read(path)?;
        let document = bson::Document::from_reader(&mut bytes.as_slice())?;
        let entry: Entry = bson::from_document(document)?;

        Ok(entry)
    }
}
