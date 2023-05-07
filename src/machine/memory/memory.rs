
use std::{collections::VecDeque, usize, vec};

pub type Umi = u32;
pub type Mem = Vec<Box<[Umi]>>;

/// Memory structure of the UM machine
pub struct Memory {
    // segmented memory
    pub segs: Mem,
    // unmapped segments ready to be mapped again
    pub unmapped_segs: VecDeque<usize>,
}
impl Memory {
    /// Memory constructor function
    pub fn new(instructions: Box<[u32]>) -> Self {

        Memory {
            segs:  vec![instructions],
            unmapped_segs: VecDeque::new(),
        }
    }
    /// return a word from the segmented memory based on its id and offset
    /// return None if no word found at that location
    pub fn get(&self, id: Umi, offset: Umi) -> Umi {
        (*self.segs[id as usize])[offset as usize]

    }
    /// set a word in the segmented memory based on its id and offset
    /// return None if no word found at that location
    pub fn set(&mut self, id: Umi, offset: Umi, val: Umi) {
        
        (*self.segs[id as usize])[offset as usize] = val

    }
    pub fn allocate(&mut self, len: usize) -> usize {
        let new_seg = vec![0 as Umi; len].into_boxed_slice();
        
        match self.unmapped_segs.pop_front() {
            Some(o) => {
                self.segs[o] = new_seg;
                o
            },
            None => {
                self.segs.push(new_seg);
                self.segs.len() - 1
            }
        }
    }
    pub fn deallocate(&mut self, id: usize) {
        self.segs[id] = Box::new([]);
        self.unmapped_segs.push_back(id)
    }


}