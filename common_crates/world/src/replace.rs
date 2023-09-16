use crate::chunk::Chunk;
use block::BlockState;

pub trait Replace<T> {
    fn replace(&mut self, new_value: T) -> Self;
}

impl Replace<Chunk> for Chunk {
    fn replace(&mut self, new_chunk: Chunk) -> Self {
        std::mem::replace(self, new_chunk)
    }
}

impl Replace<BlockState> for Chunk {
    fn replace(&mut self, new_state: BlockState) -> Self {
        std::mem::replace(self, Chunk::of(new_state))
    }
}

impl Replace<BlockState> for BlockState {
    fn replace(&mut self, new_value: BlockState) -> Self {
        std::mem::replace(self, new_value)
    }
}