use std::sync::Arc;

use crate::lychrel_sieve::sieve::SieveFunctionContext;

use super::shared_digit_pair::SharedDigitPair;

pub struct Digit{
    pub digit_place: Arc<usize>,
    pub side: Side,
    pub data: Arc<SharedDigitPair>, 
}
pub enum Side{
    LEFT, RIGHT
}

impl Digit{
    pub async fn new(digit_place: &usize) -> Self{
        Digit{ digit_place: Arc::new(*digit_place), side: Side::RIGHT, data: Arc::new(SharedDigitPair::new()) }
    }
    pub async fn sum(&self, other : &Digit, cx : &SieveFunctionContext){
        self.data.sum(self.side.clone(), &*self.digit_place, other, cx).await;
    }
    pub async fn sum_with_self(&self, cx: &SieveFunctionContext){
        self.data.sum_with_self(&self.side, *self.digit_place, cx).await;
    }
    pub async fn get(&self) -> u8{
        self.data.get(&self.side).await
    }
    pub async fn set(&self, data: u8){
        self.data.set(self.side.clone(), data).await
    }
    pub async fn mark_up(&self, cx: &SieveFunctionContext){
        cx.mark(*self.digit_place + 1).await
    }
    pub async fn to_char(&self) -> String{
        self.data.to_char(&self.side).await
    }
    pub async fn increment(&self, cx: &SieveFunctionContext) {
        self.data.increment(self.side.clone(),&*self.digit_place, cx).await
    }
    pub async fn generate_next_digit(&self) -> Digit{
        self.data.new_digit(*self.digit_place + 1).await
    }
}