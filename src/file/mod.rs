use std::fs;
use std::fs::OpenOptions;
use std::io::{Error, Write};
use crate::{AnalyticsData, serialize};

fn get_path_id(id: &String) -> String {
    format!("analytics-data/{}.plytics.bin", &id)
}

fn generate_id_from_epoch_and_uid(epoch: &usize, uid: &String) -> String {
    format!("{}-{}", epoch, uid)
}

pub fn read_file_id(id: &String) -> Result<Vec<u8>, Error> {
    fs::read(get_path_id(id))
}

pub fn read_file_epoch_and_uid(epoch: &usize, uid: &String) -> Result<Vec<u8>, Error> {
    fs::read(get_path_id(&generate_id_from_epoch_and_uid(epoch, uid)))
}

pub fn write_file_epoch_and_uid(epoch: &usize, uid: &String, data: AnalyticsData) -> Result<(), Error> {
    println!("{}", get_path_id(&generate_id_from_epoch_and_uid(epoch, uid)));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(get_path_id(&generate_id_from_epoch_and_uid(epoch, uid)))
        .expect("Unable to read/create/write file");

    let _ = file.write_all(
        serialize(&data)
            .as_slice(),
    );

    Ok(())
}
