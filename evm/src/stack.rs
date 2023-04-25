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

    pub fn push(&mut self, value: U256) {
        if self.data.len() == self.size {
            panic!("Stack overflow");
        }
        self.data.push(value);
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
}