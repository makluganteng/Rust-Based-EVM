use ethereum_types::U256;
use evm::{stack::Stack, memory::Memory, program_counter::ProgramCounter};
use crate::opcode::Opcode;

pub struct Execution {
    stack: Stack,
    memory: Memory,
    program_counter: ProgramCounter,
}

impl Execution {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(1024),
            memory: Memory::new(),
            program_counter: ProgramCounter::new(),
        }
    }

    pub fn run(&mut self, program: Vec<u8>) ->Vec<u8>{
        let mut result: Vec<u8> = Vec::new();
        while let Some(opcode) = program.get(self.program_counter.get()){
            let operation = Opcode::from(*opcode);

            match operation {
                Opcode::STOP => {
                    return result;
                },
                Opcode::ADD => {
                    self.stack.add();
                    self.program_counter.increment(1);
                },
                Opcode::MUL => {
                    self.stack.mul();
                    self.program_counter.increment(1);
                },
                Opcode::SUB => {
                    self.stack.sub();
                    self.program_counter.increment(1);
                },
                Opcode::DIV => {
                    self.stack.div();
                    self.program_counter.increment(1);
                },
                Opcode::MOD => {
                    self.stack.mod_op();
                    self.program_counter.increment(1);
                },
                Opcode::PUSH1 => {
                    let Some(value) =  program.get(self.program_counter.get() + 1) else {
                        panic!("expect PUSH operation followed by a number : PUSH1")
                    };
                    self.stack.push(U256::from(*value)).unwrap();
                    self.program_counter.increment(2);
                }
            }
        }
        return result;     
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_stack() {
        let mut execution = Execution::new();
        let program = vec![0x60, 0x01, 0x60, 0x01, 0x01, 0x60, 0x01, 0x01, 0x60, 0x01, 0x04, 0x60, 0x03, 0x02];
        let result = execution.run(program);
        assert_eq!(execution.stack.height(), 1);
    }
}