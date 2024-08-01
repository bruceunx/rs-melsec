use std::fmt;
use std::option::Option;

#[derive(Debug)]
pub struct Tag {
    device: String,
    value: Option<String>,
    data_type: Option<String>,
    error: Option<String>,
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
