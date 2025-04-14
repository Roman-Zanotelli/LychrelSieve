use std::sync::Arc;

use super::{digit::{Digit, Side}, int::BigInt, shared_digit_pair::SharedDigitPair};

impl Clone for Side{
    fn clone(&self) -> Self {
        match self {
            Self::LEFT => Self::LEFT,
            Self::RIGHT => Self::RIGHT,
        }
    }
}
impl Clone for Digit{
    fn clone(&self) -> Self {
        Self { side: self.side.clone(), data: self.data.clone(), digit_place: Arc::new(*self.digit_place) }
    }
}
impl Clone for SharedDigitPair{
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
    
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}
impl Clone for BigInt{
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
}