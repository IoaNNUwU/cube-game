use chunk::Chunk;
use block::BlockState;

pub trait ReplaceWith<T> {
    fn replace_with(&mut self, new_value: T) -> Self;
}

impl ReplaceWith<Chunk> for Chunk {
    fn replace_with(&mut self, new_chunk: Chunk) -> Self {
        std::mem::replace(self, new_chunk)
    }
}

impl ReplaceWith<BlockState> for Chunk {
    fn replace_with(&mut self, new_state: BlockState) -> Self {
        std::mem::replace(self, Chunk::of(new_state))
    }
}

impl ReplaceWith<BlockState> for BlockState {
    fn replace_with(&mut self, new_value: BlockState) -> Self {
        std::mem::replace(self, new_value)
    }
}