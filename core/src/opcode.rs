pub enum Opcode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    MOD
}


impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Opcode::STOP,
            0x01 => Opcode::ADD,
            0x02 => Opcode::MUL,
            0x03 => Opcode::SUB,
            0x04 => Opcode::DIV,
            0x05 => Opcode::MOD,
            _ => panic!("Invalid opcode")
        }
    }
}

impl Into<&' static str> for Opcode {
    fn into(self) -> &'static str {
        match self {
            Opcode::STOP => "STOP",
            Opcode::ADD => "ADD",
            Opcode::MUL => "MUL",
            Opcode::SUB => "SUB",
            Opcode::DIV => "DIV",
            Opcode::MOD => "MOD",
        }
    }
}