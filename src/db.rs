#[derive(Debug)]
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
            "b" => Ok(DataType::BIT),
            "h" => Ok(DataType::SWORD),
            "H" => Ok(DataType::UWORD),
            "i" => Ok(DataType::SDWORD),
            "I" => Ok(DataType::UDWORD),
            "f" => Ok(DataType::FLOAT),
            "d" => Ok(DataType::DOUBLE),
            "q" => Ok(DataType::SLWORD),
            "Q" => Ok(DataType::ULWORD),
            _ => Err(format!("Invalid data type: {}", s)),
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
