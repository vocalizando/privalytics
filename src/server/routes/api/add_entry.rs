use crate::{file, AnalyticsData};
use rocket::{put, serde::json::Json};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[put("/add", data = "<data>")]
pub fn add_entry(data: Json<AnalyticsData>) -> Result<(), std::io::Error> {
    let mut clean_data = data;
    let uid = Uuid::new_v4().to_string();
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock is going backwards, why tho?")
        .as_millis();

    clean_data.id = uid.as_str();
    clean_data.epoch = epoch as usize;

    file::write_file_epoch_and_uid(&(epoch as usize), &uid, clean_data.into_inner())?;
    Ok(())
}
