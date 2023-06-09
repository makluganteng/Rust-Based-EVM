
#[derive(Debug, Default)]
pub struct ProgramCounter(usize);

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        Self::default()
    }

    pub fn increment(&mut self, value: usize) {
        self.0 += value;
    }

    pub fn set_exact(&mut self, value: usize) {
        self.0 = value;
    }

    pub fn get(&self) -> usize {
        self.0
    }
}