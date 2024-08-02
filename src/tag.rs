use super::db::DataType;
use std::fmt;
use std::option::Option;

#[derive(Debug)]
pub struct Tag {
    pub device: String,
    pub value: Option<String>,
    pub data_type: DataType,
}

#[derive(Debug)]
pub struct QueryTag {
    pub device: String,
    pub data_type: DataType,
}

impl Tag {
    pub fn new(device: String, value: Option<String>, data_type: DataType) -> Self {
        Self {
            device,
            value,
            data_type,
        }
    }

    pub fn is_success(&self) -> bool {
        self.value.is_some()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {:?}, {:?}", self.device, self.value, self.data_type)
    }
}
