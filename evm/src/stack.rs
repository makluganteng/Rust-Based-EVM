use ethereum_types::U256;
use thiserror::Error;
pub struct Stack {
    size: usize,
    data: Vec<U256>
}

#[derive(Debug, Error)]
pub enum StackError {
    #[error("stack underflow")]
    StackUnderflow,
    #[error("stack overflow")]
    StackOverflow,
    #[error("index out of bounds")]
    IndexOutOfBounds,
}

pub type Result<T> = std::result::Result<T, StackError>;

impl Stack {
    //contsructor
    pub fn new(size: usize) -> Self {
        Self {
            size,
            data: Vec::new()
        }
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> Result<U256> {
        if self.data.len() <= index {
            panic!("Stack underflow");
        }
        Ok(self.data[index])
    }   

    //push functionality
    pub fn push(&mut self, value: U256) -> Result<()> {
        if self.data.len() == self.size {
            panic!("Stack overflow");
        }
        self.data.push(value);
        Ok(())
    }
    //pop functionality
    pub fn pop(&mut self) -> Result<U256>{
        if self.data.len() == 0 {
            panic!("Stack underflow");
        }
        self.data.pop().ok_or(StackError::StackUnderflow)
    }
    //duplicate functionality
    pub fn duplicate(&mut self, index: usize) -> Result<()> {
        if self.data.len() == 0 {
            panic!("Stack underflow");
        }
        //[1,2,3,4,5]
        let value = self.data[self.data.len() - index];
        self.data.push(value);
        Ok(())
    }

    //swap functionality
    pub fn swap(&mut self, index: usize) -> Result<()> {
        if self.data.len() == 0 {
            panic!("Stack underflow");
        }

        if self.data.len() <= index {
            panic!("Stack underflow");
        }
        let len = self.data.len();
        self.data.swap(len-1, index);
        Ok(())
    }

    // Arithmetic operations
    pub fn add(&mut self) {
        if self.data.len() == 0 {
            panic!("Stack underflow");
        }
        let a = self.data.pop().unwrap();
        let b = self.data.pop().unwrap();
        self.data.push(a + b);
    }

    pub fn sub(&mut self) {
        if self.data.len() == 0 {
            panic!("Stack underflow");
        }
        let a = self.data.pop().unwrap();
        let b = self.data.pop().unwrap();
        self.data.push(a - b);
    }

    pub fn mul(&mut self) {
        if self.data.len() == 0 {
            panic!("Stack underflow");
        }
        let a = self.data.pop().unwrap();
        let b = self.data.pop().unwrap();
        self.data.push(a * b);
    }

    pub fn div(&mut self) {
        if self.data.len() == 0 {
            panic!("Stack underflow");
        }
        let a = self.data.pop().unwrap();
        let b = self.data.pop().unwrap();
        self.data.push(a / b);
    }

    pub fn mod_op(&mut self) {
        if self.data.len() == 0 {
            panic!("Stack underflow");
        }
        let a = self.data.pop().unwrap();
        let b = self.data.pop().unwrap();
        self.data.push(a % b);
    }

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_stack() {
        let mut stack = Stack::new(10);
        stack.push(U256::from(1));
        stack.push(U256::from(2));
        stack.push(U256::from(3));
        assert_eq!(stack.data.len(), 3);
        assert_eq!(stack.data[0], 1.into());
        assert_eq!(stack.data[1], 2.into());
        assert_eq!(stack.data[2], 3.into());
    }

    #[test]
    fn test_pop(){
        let mut stack = Stack::new(10);
        stack.push(U256::from(1));
        stack.push(U256::from(2));
        stack.push(U256::from(3));
        stack.pop();
        assert_eq!(stack.data.len(), 2);
        assert_eq!(stack.data[0], 1.into());
        assert_eq!(stack.data[1], 2.into());
    }

    #[test]
    fn test_duplicate(){
        let mut stack = Stack::new(10);
        stack.push(U256::from(1));
        stack.push(U256::from(2));
        stack.push(U256::from(3));
        stack.duplicate(2);
        assert_eq!(stack.data.len(), 4);
        assert_eq!(stack.data[0], 1.into());
        assert_eq!(stack.data[1], 2.into());
        assert_eq!(stack.data[2], 3.into());
        assert_eq!(stack.data[3], 2.into());
    }

    #[test]
    fn test_swap(){
        let mut stack = Stack::new(10);
        stack.push(U256::from(1));
        stack.push(U256::from(2));
        stack.push(U256::from(3));
        stack.swap(1);
        assert_eq!(stack.data.len(), 3);
        assert_eq!(stack.data[0], 1.into());
        assert_eq!(stack.data[1], 3.into());
        assert_eq!(stack.data[2], 2.into());
    }
    #[test]
    fn test_add(){
        let mut stack = Stack::new(10);
        stack.push(U256::from(1));
        stack.push(U256::from(2));
        stack.add();
        assert_eq!(stack.data.len(), 1);
        assert_eq!(stack.data[0], 3.into());
    }

    #[test]
    fn test_sub(){
        let mut stack = Stack::new(10);
        stack.push(U256::from(1));
        stack.push(U256::from(2));
        stack.sub();
        assert_eq!(stack.data.len(), 1);
        assert_eq!(stack.data[0], 1.into());
    }

    #[test]
    fn test_mul(){
        let mut stack = Stack::new(10);
        stack.push(U256::from(1));
        stack.push(U256::from(2));
        stack.mul();
        assert_eq!(stack.data.len(), 1);
        assert_eq!(stack.data[0], 2.into());
    }

    #[test]
    fn test_div(){
        let mut stack = Stack::new(10);
        stack.push(U256::from(1));
        stack.push(U256::from(2));
        stack.div();
        assert_eq!(stack.data.len(), 1);
        assert_eq!(stack.data[0], 2.into());
    }

    #[test]
    fn test_mod(){
        let mut stack = Stack::new(10);
        stack.push(U256::from(1));
        stack.push(U256::from(2));
        stack.mod_op();
        assert_eq!(stack.data.len(), 1);
        assert_eq!(stack.data[0], 0.into());
    }

}