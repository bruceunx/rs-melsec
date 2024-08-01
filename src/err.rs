use std::fmt;

#[derive(Debug)]
pub struct MCError {
    error_code: String,
}

impl MCError {
    pub fn new(error_code: u16) -> MCError {
        Self {
            error_code: format!("0x{:04x}", error_code),
        }
    }
    pub fn description(&self) -> String {
        match self.error_code.as_str() {
            "0x0050" => "0x0050: When \"Communication Data Code\" is set to ASCII Code, ASCII code data that cannot be converted to binary were received.".to_string(),
            "0x0051" | "0x0052" | "0x0053" | "0x0054" => "0x0051-0x0054: The number of read or write points is outside the allowable range.".to_string(),
            "0x0055" => "0x0055: Although online change is disabled, the connected device requested the RUN-state CPU module for data writing.".to_string(),
            "0xC056" => "0xC056: The read or write request exceeds the maximum address.".to_string(),
            "0xC058" => "0xC058: The request data length after ASCII-to-binary conversion does not match the data size of the character area (a part of text data).".to_string(),
            "0xC059" => "0xC059: The command and/or subcommand are specified incorrectly. The CPU module does not support the command and/or subcommand.".to_string(),
            "0xC05B" => "0xC05B: The CPU module cannot read data from or write data to the specified device.".to_string(),
            "0xC05C" => "0xC05C: The request data is incorrect. (e.g. reading or writing data in units of bits from or to a word device)".to_string(),
            "0xC05D" => "0xC05D: No monitor registration.".to_string(),
            "0xC05F" => "0xC05F: The request cannot be executed to the CPU module.".to_string(),
            "0xC060" => "0xC060: The request data is incorrect. (ex. incorrect specification of data for bit devices)".to_string(),
            "0xC061" => "0xC061: The request data length does not match the number of data in the character area (a part of text data).".to_string(),
            "0xC06F" => "0xC06F: The CPU module received a request message in ASCII format when \"Communication Data Code is set to Binary Code, or received it in binary format when the setting is set to ASCII Code. (This error code is only registered to the error history, and no abnormal response is returned.)".to_string(),
            "0xC070" => "0xC070: The device memory extension cannot be specified for the target station.".to_string(),
            "0xC0B5" => "0xC0B5: The CPU module cannot handle the data specified.".to_string(),
            "0xC200" => "0xC200: The remote password is incorrect.".to_string(),
            "0xC201" => "0xC201: The port used for communication is locked with the remote password. Or, because of the remote password lock status with \"Communication Data Code\" set to ASCII Code, the subcommand and later part cannot be converted to a binary code.".to_string(),
            "0xC204" => "0xC204: The connected device is different from the one that requested for unlock processing of the remote password.".to_string(),
            _ => format!("{}: Unknown error code.", self.error_code),
        }
    }
}

impl fmt::Display for MCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for MCError {}
