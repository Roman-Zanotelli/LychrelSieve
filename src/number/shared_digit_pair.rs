use std::{sync::Arc, usize};

use tokio::sync::RwLock;

use crate::lychrel_sieve::sieve::SieveFunctionContext;

use super::digit::{Digit, Side};

impl SharedDigitPair{
    pub async fn get(&self, side: &Side) -> u8{
        match side {
            Side::LEFT => return *self.data.read().await >> 4,
            Side::RIGHT => return *self.data.read().await & 0b1111,
        }
    }
    pub async fn set(&self, side: Side, data : u8){
        let old = *self.data.read().await;
        match side {
            Side::LEFT => *self.data.write().await = (old & 0b1111) | (data << 4),
            Side::RIGHT => *self.data.write().await = (old & !0b1111) | (data),
        }
    }
    pub async fn new_digit(&self, at: usize) -> Digit{
        self.set(Side::LEFT, 0b1).await;
        Digit{ digit_place: Arc::new(at), side: Side::LEFT, data: Arc::new(self.clone()) }
    }
    pub async fn increment(&self, side: Side, at: &usize, cx: &SieveFunctionContext){
        let old = *self.data.read().await;
        match side{
            Side::LEFT => {
                let mut copy = (old >> 4) + 1;
                match copy {
                    0..10 => {},
                    10 => {copy = 0; cx.mark(at+1).await;},
                    _ => panic!("This shouldnt happen")
                }
                *self.data.write().await = (copy << 4) | (old & 0b1111)
            },
            Side::RIGHT => {
                let mut copy = (old & 0b1111) + 1;
                match copy {
                    0..10 => {},
                    10 => {copy = 0; cx.mark(at+1).await;},
                    _ => panic!("This shouldnt happen")
                }
                *self.data.write().await = copy | (old & !0b1111)
            },
        }
    }
    pub async fn gen_digit_refs(&self, at: usize) ->  (Option<Digit>, Digit) {
            match (self.get(&Side::LEFT).await, self.get(&Side::RIGHT).await){
                (0b1111, _) => (None, Digit{ side: Side::RIGHT, data: Arc::new(self.clone()), digit_place: Arc::new(at) })
                ,
                (_, 0b1111) => panic!("Right value contained a placeholder"),
                (_, _) => (Some(Digit{ side: Side::LEFT, data: Arc::new(self.clone()), digit_place: Arc::new(at + 1) }), Digit{ side: Side::RIGHT, data: Arc::new(self.clone()), digit_place: Arc::new(at) })
            }
    }
    pub async fn sum(&self, side: Side, at: &usize, other: &Digit, cx: &SieveFunctionContext){
        let mut sum = self.get(&side).await + other.get().await;
        match sum{
            0..10 => {
            },
            10..19 =>{
                sum -= 10;
                cx.mark(at + 1).await;
                other.mark_up(cx).await;
            },
            _ => panic!("This shouldnt happen")
        };
        self.set(side.clone(), sum).await;
        other.set(sum).await;
    }
    pub async fn sum_with_self(&self, side: &Side, at: usize, cx: &SieveFunctionContext){
        let mut sum = self.get(&side).await * 2;
        match sum {
            0..10 => {
            },
            10..19 => {
                sum -= 10;
                cx.mark(at + 1).await;
            },
            _ => panic!("This shouldnt happen")
        };

        self.set(side.clone(), sum).await;
    }
    
    pub(crate) async fn to_char(&self, side: &Side) -> String {
        match side {
            Side::LEFT => ((*self.data.read().await & (!0b1111 as u8)) >> 4).to_string(),
            Side::RIGHT => (*self.data.read().await & (0b1111 as u8)).to_string(),
        }
    }
    pub fn new() -> Self{
        SharedDigitPair { data: Arc::new(RwLock::new(0b1)) }
    }
}
pub struct SharedDigitPair{ //actual data held
    pub data: Arc<RwLock<u8>>
}