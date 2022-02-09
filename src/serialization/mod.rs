use bincode::config::Configuration;
use bincode::{decode_from_slice, encode_to_vec};
use crate::AnalyticsData;

const BINCODE_CONFIGURATION: Configuration = bincode::config::standard();

pub fn serialize(data: &AnalyticsData) -> Vec<u8> {
    encode_to_vec(data, BINCODE_CONFIGURATION)
        .unwrap()
}

pub fn deserialize(data: &Vec<u8>) -> AnalyticsData {
    let (parsed, _): (AnalyticsData, usize) =
        decode_from_slice(&data[..], BINCODE_CONFIGURATION).unwrap();

    parsed
}