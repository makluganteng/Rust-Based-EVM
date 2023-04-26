use ethereum_types::U256;

pub struct Stack {
    size: usize,
    data: Vec<U256>
}


impl Stack {
    //contsructor
    pub fn new(size: usize) -> Self {
        Self {
            size,
            data: Vec::new()
        }
    }

    //push functionality
    pub fn push(&mut self, value: U256) {
        if self.data.len() == self.size {
            panic!("Stack overflow");
        }
        self.data.push(value);
    }
    //pop functionality
    pub fn pop(&mut self){
        if self.data.len() == 0 {
            panic!("Stack underflow");
        }
        self.data.pop();
    }
    //duplicate functionality
    pub fn duplicate(&mut self, index: usize) {
        if self.data.len() == 0 {
            panic!("Stack underflow");
        }
        //[1,2,3,4,5]
        let value = self.data[self.data.len() - index];
        self.data.push(value);
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