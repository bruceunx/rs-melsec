use super::db::DataType;
use std::fmt;
use std::option::Option;

#[derive(Debug)]
pub struct Tag {
    pub device: String,
    pub value: Option<String>,
    pub data_type: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct QueryTag {
    pub device: String,
    pub data_type: DataType,
}

impl Tag {
    pub fn new(
        device: String,
        value: Option<String>,
        data_type: Option<String>,
        error: Option<String>,
    ) -> Self {
        Self {
            device,
            value,
            data_type,
            error,
        }
    }

    pub fn is_success(&self) -> bool {
        self.value.is_some() && self.error.is_none()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {:?}, {:?}, {:?}",
            self.device, self.value, self.data_type, self.error
        )
    }
}
