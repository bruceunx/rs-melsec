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
