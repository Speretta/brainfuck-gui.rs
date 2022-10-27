use std::{io::Read, collections::VecDeque};


pub struct BrainFuckStream{
    pub(super) input: Input,
    pub(super) output_queue: VecDeque<u8>
}

impl Read for BrainFuckStream{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        for byte in buf{
            if let Some(n) = self.next(){
                *byte = n;
            }
        }
        Ok(0)
    }
}


impl Iterator for BrainFuckStream{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.output_queue.pop_front()
    }
}

pub(super) enum Input{
    Wait,
    Ok(u8)
}