pub enum Opcode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    MOD,
    PUSH1,
}


impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::STOP,
            0x01 => Self::ADD,
            0x02 => Self::MUL,
            0x03 => Self::SUB,
            0x04 => Self::DIV,
            0x05 => Self::MOD,
            0x60 => Self::PUSH1,
            _ => panic!("Invalid opcode")
        }
    }
}
    
impl Into<&'static str> for Opcode {
    fn into(self) -> &'static str {
        match self {
            Self::STOP => "STOP",
            Self::ADD => "ADD",
            Self::MUL => "MUL",
            Self::SUB => "SUB",
            Self::DIV => "DIV",
            Self::MOD => "MOD",
            Self::PUSH1 => "PUSH1",
        }
    }
}