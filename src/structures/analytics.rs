use std::collections::HashMap;

pub struct Entry {
    pub metadata: Metadata,

}

pub struct Metadata {
    pub date: u64,
    pub duid: String,
    pub page: Option<String>,
    pub uid: Option<String>,
}

pub type Data = HashMap<String, DataValues>;

pub enum DataValues {
    String(String),
    Number(u32),
    Bool(bool),
    Date(u64),
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
impl_from_for_data_values!(u64, Date);

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

    #[test]
    fn from_date() {
        let data = DataValues::from(169_u64);
        match data {
            DataValues::Date(v) => { assert_eq!(v, 169_u64) }
            _ => { panic!("not a date") }
        }
    }
}
