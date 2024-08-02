use std::error::Error;

pub mod consts {
    // PLC definition
    pub const Q_SERIES: &str = "Q";
    pub const L_SERIES: &str = "L";
    pub const QNA_SERIES: &str = "QnA";
    pub const IQL_SERIES: &str = "iQ-L";
    pub const IQR_SERIES: &str = "iQ-R";

    // communication type
    pub const COMMTYPE_BINARY: &str = "binary";
    pub const COMMTYPE_ASCII: &str = "ascii";

    // endian types
    pub const ENDIAN_NATIVE: char = '=';
    pub const ENDIAN_LITTLE: char = '<';
    pub const ENDIAN_BIG: char = '>';
    pub const ENDIAN_NETWORK: char = '!';
}

// Commands
pub mod commands {
    pub const BATCH_READ: u16 = 0x0401;
    pub const BATCH_WRITE: u16 = 0x1401;
    pub const RANDOM_READ: u16 = 0x0403;
    pub const RANDOM_WRITE: u16 = 0x1402;
    pub const MONITOR_REG: u16 = 0x0801;
    pub const MONITOR: u16 = 0x0802;
    pub const REMOTE_RUN: u16 = 0x1001;
    pub const REMOTE_STOP: u16 = 0x1002;
    pub const REMOTE_PAUSE: u16 = 0x1003;
    pub const REMOTE_LATCH_CLEAR: u16 = 0x1005;
    pub const REMOTE_RESET: u16 = 0x1006;
    pub const REMOTE_UNLOCK: u16 = 0x1630;
    pub const REMOTE_LOCK: u16 = 0x1631;
    pub const ERROR_LED_OFF: u16 = 0x1617;
    pub const READ_CPU_MODEL: u16 = 0x0101;
    pub const LOOPBACK_TEST: u16 = 0x0619;
}

// SubCommands
pub mod subcommands {
    pub const ZERO: u16 = 0x0000;
    pub const ONE: u16 = 0x0001;
    pub const TWO: u16 = 0x0002;
    pub const THREE: u16 = 0x0003;
    pub const FIVE: u16 = 0x0005;
    pub const A: u16 = 0x000A;
    pub const F: u16 = 0x000F;
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    BIT,
    SWORD,
    UWORD,
    SDWORD,
    UDWORD,
    FLOAT,
    DOUBLE,
    SLWORD,
    ULWORD,
}

impl DataType {
    pub fn size(&self) -> i8 {
        match self {
            DataType::BIT | DataType::SWORD | DataType::UWORD => 2,
            DataType::SDWORD | DataType::UDWORD | DataType::FLOAT => 4,
            DataType::DOUBLE | DataType::SLWORD | DataType::ULWORD => 8,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "b" => Some(DataType::BIT),
            "h" => Some(DataType::SWORD),
            "H" => Some(DataType::UWORD),
            "i" => Some(DataType::SDWORD),
            "I" => Some(DataType::UDWORD),
            "f" => Some(DataType::FLOAT),
            "d" => Some(DataType::DOUBLE),
            "q" => Some(DataType::SLWORD),
            "Q" => Some(DataType::ULWORD),
            _ => None,
        }
    }

    pub fn to_struct_type(&self) -> &str {
        match self {
            DataType::BIT => "b",
            DataType::SWORD => "h",
            DataType::UWORD => "H",
            DataType::SDWORD => "i",
            DataType::UDWORD => "I",
            DataType::FLOAT => "f",
            DataType::DOUBLE => "d",
            DataType::SLWORD => "q",
            DataType::ULWORD => "Q",
        }
    }
}

pub struct DeviceConstants;

impl DeviceConstants {
    // Define constants
    pub const SM_DEVICE: u8 = 0x91;
    pub const SD_DEVICE: u8 = 0xA9;
    pub const X_DEVICE: u8 = 0x9C;
    pub const Y_DEVICE: u8 = 0x9D;
    pub const M_DEVICE: u8 = 0x90;
    pub const L_DEVICE: u8 = 0x92;
    pub const F_DEVICE: u8 = 0x93;
    pub const V_DEVICE: u8 = 0x94;
    pub const B_DEVICE: u8 = 0xA0;
    pub const D_DEVICE: u8 = 0xA8;
    pub const W_DEVICE: u8 = 0xB4;
    pub const TS_DEVICE: u8 = 0xC1;
    pub const TC_DEVICE: u8 = 0xC0;
    pub const TN_DEVICE: u8 = 0xC2;
    pub const SS_DEVICE: u8 = 0xC7;
    pub const SC_DEVICE: u8 = 0xC6;
    pub const SN_DEVICE: u8 = 0xC8;
    pub const CS_DEVICE: u8 = 0xC4;
    pub const CC_DEVICE: u8 = 0xC3;
    pub const CN_DEVICE: u8 = 0xC5;
    pub const SB_DEVICE: u8 = 0xA1;
    pub const SW_DEVICE: u8 = 0xB5;
    pub const DX_DEVICE: u8 = 0xA2;
    pub const DY_DEVICE: u8 = 0xA3;
    pub const R_DEVICE: u8 = 0xAF;
    pub const ZR_DEVICE: u8 = 0xB0;

    pub const LTS_DEVICE: u8 = 0x51;
    pub const LTC_DEVICE: u8 = 0x50;
    pub const LTN_DEVICE: u8 = 0x52;
    pub const LSTS_DEVICE: u8 = 0x59;
    pub const LSTC_DEVICE: u8 = 0x58;
    pub const LSTN_DEVICE: u8 = 0x5A;
    pub const LCS_DEVICE: u8 = 0x55;
    pub const LCC_DEVICE: u8 = 0x54;
    pub const LCN_DEVICE: u8 = 0x56;
    pub const LZ_DEVICE: u8 = 0x62;
    pub const RD_DEVICE: u8 = 0x2C;

    pub const BIT_DEVICE: &'static str = "bit";
    pub const WORD_DEVICE: &'static str = "word";
    pub const DWORD_DEVICE: &'static str = "dword";

    // Static methods
    pub fn get_binary_device_code(
        plc_type: &str,
        device_name: &str,
    ) -> Result<(u8, u32), Box<dyn Error>> {
        match device_name {
            "SM" => Ok((DeviceConstants::SM_DEVICE, 10)),
            "SD" => Ok((DeviceConstants::SD_DEVICE, 10)),
            "X" => Ok((DeviceConstants::X_DEVICE, 16)),
            "Y" => Ok((DeviceConstants::Y_DEVICE, 16)),
            "M" => Ok((DeviceConstants::M_DEVICE, 10)),
            "L" => Ok((DeviceConstants::L_DEVICE, 10)),
            "F" => Ok((DeviceConstants::F_DEVICE, 10)),
            "V" => Ok((DeviceConstants::V_DEVICE, 10)),
            "B" => Ok((DeviceConstants::B_DEVICE, 16)),
            "D" => Ok((DeviceConstants::D_DEVICE, 10)),
            "W" => Ok((DeviceConstants::W_DEVICE, 16)),
            "TS" => Ok((DeviceConstants::TS_DEVICE, 10)),
            "TC" => Ok((DeviceConstants::TC_DEVICE, 10)),
            "TN" => Ok((DeviceConstants::TN_DEVICE, 10)),
            "SS" => Ok((DeviceConstants::SS_DEVICE, 10)),
            "SC" => Ok((DeviceConstants::SC_DEVICE, 10)),
            "SN" => Ok((DeviceConstants::SN_DEVICE, 10)),
            "CS" => Ok((DeviceConstants::CS_DEVICE, 10)),
            "CC" => Ok((DeviceConstants::CC_DEVICE, 10)),
            "CN" => Ok((DeviceConstants::CN_DEVICE, 10)),
            "SB" => Ok((DeviceConstants::SB_DEVICE, 16)),
            "SW" => Ok((DeviceConstants::SW_DEVICE, 16)),
            "DX" => Ok((DeviceConstants::DX_DEVICE, 16)),
            "DY" => Ok((DeviceConstants::DY_DEVICE, 16)),
            "R" => Ok((DeviceConstants::R_DEVICE, 10)),
            "ZR" => Ok((DeviceConstants::ZR_DEVICE, 16)),
            "LTS" if plc_type == "iQR_SERIES" => Ok((DeviceConstants::LTS_DEVICE, 10)),
            "LTC" if plc_type == "iQR_SERIES" => Ok((DeviceConstants::LTC_DEVICE, 10)),
            "LTN" if plc_type == "iQR_SERIES" => Ok((DeviceConstants::LTN_DEVICE, 10)),
            "LSTS" if plc_type == "iQR_SERIES" => Ok((DeviceConstants::LSTS_DEVICE, 10)),
            "LSTC" if plc_type == "iQR_SERIES" => Ok((DeviceConstants::LSTC_DEVICE, 10)),
            "LSTN" if plc_type == "iQR_SERIES" => Ok((DeviceConstants::LSTN_DEVICE, 10)),
            "LCS" if plc_type == "iQR_SERIES" => Ok((DeviceConstants::LCS_DEVICE, 10)),
            "LCC" if plc_type == "iQR_SERIES" => Ok((DeviceConstants::LCC_DEVICE, 10)),
            "LCN" if plc_type == "iQR_SERIES" => Ok((DeviceConstants::LCN_DEVICE, 10)),
            "LZ" if plc_type == "iQR_SERIES" => Ok((DeviceConstants::LZ_DEVICE, 10)),
            "RD" if plc_type == "iQR_SERIES" => Ok((DeviceConstants::RD_DEVICE, 10)),
            _ => Err(format!(
                "failed to get binary device code for device: {}",
                device_name,
            )
            .into()),
        }
    }

    pub fn get_ascii_device_code(
        plc_type: &str,
        device_name: &str,
    ) -> Result<(String, u32), Box<dyn Error>> {
        let padding = if plc_type == consts::IQR_SERIES { 4 } else { 2 };
        let padded_name = format!("{:*<width$}", device_name, width = padding);

        match device_name {
            "SM" | "SD" | "X" | "Y" | "M" | "L" | "F" | "V" | "B" | "D" | "W" | "TS" | "TC"
            | "TN" | "CS" | "CC" | "CN" | "SB" | "SW" | "DX" | "DY" | "R" | "ZR" => {
                Ok((padded_name, 16))
            }
            "STS" if plc_type == consts::IQR_SERIES => {
                Ok((format!("{:*<width$}", "STS", width = padding), 10))
            }
            "STS" => Ok((format!("{:*<width$}", "SS", width = padding), 10)),
            "STC" if plc_type == consts::IQR_SERIES => {
                Ok((format!("{:*<width$}", "STC", width = padding), 10))
            }
            "STC" => Ok((format!("{:*<width$}", "SC", width = padding), 10)),
            "STN" if plc_type == consts::IQR_SERIES => {
                Ok((format!("{:*<width$}", "STN", width = padding), 10))
            }
            "STN" => Ok((format!("{:*<width$}", "SN", width = padding), 10)),
            "LTS" if plc_type == "iQR_SERIES" => Ok((padded_name, 10)),
            "LTC" if plc_type == "iQR_SERIES" => Ok((padded_name, 10)),
            "LTN" if plc_type == "iQR_SERIES" => Ok((padded_name, 10)),
            "LSTS" if plc_type == "iQR_SERIES" => Ok((padded_name, 10)),
            "LSTN" if plc_type == "iQR_SERIES" => Ok((padded_name, 10)),
            "LCS" if plc_type == "iQR_SERIES" => Ok((padded_name, 10)),
            "LCC" if plc_type == "iQR_SERIES" => Ok((padded_name, 10)),
            "LCN" if plc_type == "iQR_SERIES" => Ok((padded_name, 10)),
            "LZ" if plc_type == "iQR_SERIES" => Ok((padded_name, 10)),
            "RD" if plc_type == "iQR_SERIES" => Ok((padded_name, 10)),
            _ => Err(format!(
                "failed to get ascii device code  for device: {}",
                device_name,
            )
            .into()),
        }
    }

    pub fn get_device_type(
        plc_type: &str,
        device_name: &str,
    ) -> Result<&'static str, Box<dyn Error>> {
        match device_name {
            "SM" | "X" | "Y" | "M" | "L" | "F" | "V" | "B" | "TS" | "TC" | "STS" | "STC" | "CS"
            | "CC" | "SB" | "DX" | "DY" => Ok(DeviceConstants::BIT_DEVICE),
            "SD" | "D" | "W" | "TN" | "STN" | "CN" | "SW" | "R" | "ZR" => {
                Ok(DeviceConstants::WORD_DEVICE)
            }
            "LSTN" | "LCN" | "LZ" => match plc_type {
                consts::IQR_SERIES => Ok(DeviceConstants::DWORD_DEVICE),
                _ => Err(format!("Unsupported PLC type: {}", plc_type).into()),
            },
            "LST" | "LTC" | "LTN" | "LSTS" | "LCS" | "LCC" => match plc_type {
                consts::IQR_SERIES => Ok(DeviceConstants::BIT_DEVICE),
                _ => Err(format!("Unsupported PLC type: {}", plc_type).into()),
            },
            "RD" => match plc_type {
                consts::IQR_SERIES => Ok(DeviceConstants::WORD_DEVICE),
                _ => Err(format!("Unsupported PLC type: {}", plc_type).into()),
            },
            _ => Err(format!(
                "failed to get ascii device code  for device: {}",
                device_name,
            )
            .into()),
        }
    }
}
