struct Memory(Vec<u8>);

impl Memory {
    pub fn new() -> Memory {
        Self::default()
    }

    pub fn read(&self, from:usize, to:usize) -> Vec<u8> {
        let size = self.0.len();
        // if the from is greater than the size of the memory then return a vector of 0's
        if from > size {
            vec![0u8; from-size]
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
}