use std::sync::Arc;

use crate::lychrel_sieve::sieve::SieveFunctionContext;

use super::shared_digit_pair::SharedDigitPair;

pub struct Digit{
    pub digit_place: Arc<usize>,
    pub side: Side,
    pub data: SharedDigitPair, 
}
pub enum Side{
    LEFT, RIGHT
}

impl Digit{
    pub fn sum(&mut self, other : &mut Digit, cx : &mut SieveFunctionContext){
        self.data.sum(&self.side, *self.digit_place, other, cx);
    }
    pub fn sum_with_self(&mut self, cx: &mut SieveFunctionContext){
        self.data.sum_with_self(&self.side, *self.digit_place, cx);
    }
    pub fn get(&self) -> u8{
        self.data.get(&self.side)
    }
    pub fn set(&mut self, data: &u8){
        self.data.set(&self.side, data);
    }
    pub fn mark_up(&self, cx: &mut SieveFunctionContext){
        cx.mark(*self.digit_place + 1);
    }
}