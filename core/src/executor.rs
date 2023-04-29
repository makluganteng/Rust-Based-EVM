pub struct Execution {
    stack: Stack,
    memory: Memory,
    program_counter: ProgramCounter,
}

impl Execution {
    pub fn new(stack_size: usize) -> Execution {
        Execution {
            stack: Stack::new(stack_size),
            memory: Memory::new(),
            program_counter: ProgramCounter::new(),
        }
    }

    pub fn run 
}