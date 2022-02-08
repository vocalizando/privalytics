use bincode::{BorrowDecode, Decode, Encode};
use rocket::serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum DeviceType {
    Null,
    Mobile,
    Tablet,
    Desktop,
    Television,
}

#[derive(Encode, Decode, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum OSType {
    Null,
    Unix,
    Windows,
    Android,
    MacOs,
    IOs,
}

/// Basic device data (device and OS)
#[derive(Encode, Decode, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct DeviceData {
    pub device: Option<DeviceType>,
    pub os: Option<OSType>,
}

/// Core Web Vitals struct
/// Ref: https://web.dev/defining-core-web-vitals-thresholds/
#[derive(Encode, Decode, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct WebVitals {
    /// Largest Contentful Paint (ms)
    pub lcp: Option<u32>,
    /// First Input Delay (ms)
    pub fid: Option<u32>,
    /// First Contentful Paint (ms)
    pub fcp: Option<u32>,
    /// Cumulative Layout Shift (0 - ~1)
    pub cls: Option<f32>,
}

#[derive(Encode, BorrowDecode, Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct DbAnalyticsData<'a> {
    pub id: &'a str,
    pub page: &'a str,
    pub epoch: usize,
    pub vitals: Option<WebVitals>,
    pub device: Option<DeviceData>,
    pub notes: Option<&'a str>,
}
