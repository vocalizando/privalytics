use std::error::Error;
use std::fs;
use std::path::Path;
use crate::structures::analytics::Entry;

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
}
