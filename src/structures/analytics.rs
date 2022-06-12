use std::collections::HashMap;
use std::fmt::Formatter;
use bson::{Bson};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub metadata: Metadata,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub date: u64,
    pub duid: String,
    pub page: Option<String>,
    pub uid: Option<String>,
}

pub type Data = HashMap<String, DataValues>;

#[derive(Debug)]
pub enum DataValues {
    String(String),
    Number(u32),
    Bool(bool),
}

impl Into<Bson> for DataValues {
    fn into(self) -> Bson {
        match self {
            DataValues::String(v) => Bson::String(v),
            DataValues::Number(v) => Bson::Int32(i32::try_from(v.to_owned()).unwrap()),
            DataValues::Bool(v) => Bson::Boolean(v),
        }
    }
}

impl Serialize for DataValues {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        match self {
            DataValues::String(v) => serializer.serialize_str(v.as_str()),
            DataValues::Number(v) => serializer.serialize_i32(i32::try_from(v.to_owned()).unwrap()),
            DataValues::Bool(v) => serializer.serialize_bool(*v),
        }
    }
}

struct DataValuesVisitor;
impl<'de> Visitor<'de> for DataValuesVisitor {
    type Value = DataValues;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("expected str, number or bool")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: Error {
        Ok(DataValues::Bool(v))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: Error {
        Ok(DataValues::Number(u32::try_from(v).unwrap()))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Ok(DataValues::String(v.to_string()))
    }
}

impl<'de> Deserialize<'de> for DataValues {
    fn deserialize<D>(deserializer: D) -> Result<DataValues, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DataValuesVisitor)
    }
}

macro_rules! impl_from_for_data_values {
    ($type:ty, $name:ident) => {
        impl From<$type> for DataValues {
            fn from(v: $type) -> Self {
                DataValues::$name(v)
            }
        }
    }
}

impl_from_for_data_values!(String, String);
impl_from_for_data_values!(u32, Number);
impl_from_for_data_values!(bool, Bool);

#[cfg(test)]
mod data_values_tests {
    use super::*;

    #[test]
    fn from_string() {
        let data = DataValues::from(String::from("hi!"));
        match data {
            DataValues::String(v) => { assert_eq!(v.as_str(), "hi!") }
            _ => { panic!("not a string") }
        }
    }

    #[test]
    fn from_u32() {
        let data = DataValues::from(42_u32);
        match data {
            DataValues::Number(v) => { assert_eq!(v, 42_u32) }
            _ => { panic!("not a number") }
        }
    }

    #[test]
    fn from_bool() {
        let data = DataValues::from(true);
        match data {
            DataValues::Bool(v) => { assert_eq!(v, true) }
            _ => { panic!("not a bool") }
        }
    }
}
