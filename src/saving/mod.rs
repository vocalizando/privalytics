use std::error::Error;

trait EntrySave {
    fn save(&self) -> Result<(), Box<dyn Error>>;
}
