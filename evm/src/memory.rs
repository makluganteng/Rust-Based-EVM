#[derive(Debug, Default)]
pub struct Memory(Vec<u8>);


impl Memory {
    pub fn new() -> Memory {
        Self::default()
    }

    pub fn read(&self, from:usize, to:usize) -> Vec<u8> {
        let size = self.0.len();
        // if the from is greater than the size of the memory then return a vector of 0's
        if from > size {
            return vec![0u8; from-to];
        }
        //This is how to read from unallocated memory
        //So what it does is that it will get the from to size and then add the rest if the byte array using 0's till it reach the 'to' index
        //This is a way of safely handling attempts to read beyond the end of the array by just returning zeroes for these out-of-bounds elements
        if to > size {
            [self.0[from..size].to_vec(), vec![0u8; to-size]].concat()
        }else{
            self.0[from..to].to_vec()
        }
    }

    pub fn write(&mut self, from:usize, value: Vec<u8>) {
        // declare the size to be the length of the memory
        let to = from + value.len();
        // if the from is greater than the size of the memory then extend the memory with 0's
        if to > self.0.len() {
            self.0.resize_with(to, || 0);
        }
        // write the value to the memory
        self.0.splice(from..to, value);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_write(){
        let mut memory = Memory::new();
        memory.write(0, vec![1,2,3,4,5]);
        assert_eq!(memory.read(0, 5), vec![1,2,3,4,5]);
        assert_eq!(memory.read(0, 6), vec![1,2,3,4,5,0]);
        assert_eq!(memory.read(0, 4), vec![1,2,3,4]);
        assert_eq!(memory.read(0, 3), vec![1,2,3]);
        assert_eq!(memory.read(0, 2), vec![1,2]);
        assert_eq!(memory.read(0, 1), vec![1]);
        assert_eq!(memory.read(0, 0), vec![]);
        assert_eq!(memory.read(1, 5), vec![2,3,4,5]);
        assert_eq!(memory.read(2, 5), vec![3,4,5]);
        assert_eq!(memory.read(3, 5), vec![4,5]);
        assert_eq!(memory.read(4, 5), vec![5]);
        assert_eq!(memory.read(5, 5), vec![]);
        assert_eq!(memory.read(6, 5), vec![0]);
    }

    #[test]
    fn test_write(){
        let mut memory = Memory::new();
        memory.write(0, vec![1,2,3,4,5]);
        assert_eq!(memory.read(0, 5), vec![1,2,3,4,5]);
        memory.write(0, vec![6,7,8]);
        assert_eq!(memory.read(0, 3), vec![6,7,8]);
        assert_eq!(memory.read(0, 5), vec![6,7,8,4,5]);
        assert_eq!(memory.read(3, 5), vec![4,5]);
        assert_eq!(memory.read(5, 5), vec![]);
    }
}
