use std::sync::Arc;

use tokio::sync::RwLock;

use crate::lychrel_sieve::sieve::SieveFunctionContext;

use super::digit::{Digit, Side};

impl SharedDigitPair{
    pub fn get(&self, side: &Side) -> u8{
        match side {
            Side::LEFT => return *self.data.blocking_read() >> 4,
            Side::RIGHT => return *self.data.blocking_read() & 0b1111,
        }
    }
    pub fn set(&mut self, side: &Side, data : &u8){
        match side {
            Side::LEFT => *self.data.blocking_write() = (*self.data.blocking_read() & 0b1111) | (data << 4),
            Side::RIGHT => *self.data.blocking_write() = (*self.data.blocking_read() & !0b1111) | (data),
        }
    }
    pub fn increment(&self, side: &Side){
        match side{
            Side::LEFT => *self.data.blocking_write() = (((*self.data.blocking_read() >> 4) + 1) << 4) | *self.data.blocking_read(),
            Side::RIGHT => *self.data.blocking_write() = ((*self.data.blocking_read() & 0b1111) + 1) | (*self.data.blocking_read() & !0b1111),
        }   
    }
    fn is_single(&self) -> bool{
        match *self.data.blocking_read() & !0b1111{
            0b11000000 => return true,
            _ => return false
        }
    }
    pub fn gen_digit_refs(&self, at: usize) -> Result<(Digit, Digit), (Digit, Digit)> {
        match self.is_single(){
            true => Err((Digit{ side: Side::LEFT, data: self.clone(), digit_place: Arc::new(at + 1) }, Digit{ side: Side::RIGHT, data: self.clone(), digit_place: Arc::new(at) })),
            false => Ok((Digit{ side: Side::LEFT, data: self.clone(), digit_place: Arc::new(at + 1) }, Digit{ side: Side::RIGHT, data: self.clone(), digit_place: Arc::new(at) })),
        }
    }
    pub fn sum(&mut self, side: &Side, at: usize, other: &mut Digit, cx: &mut SieveFunctionContext){
        let mut sum = self.get(side) * other.get();
        match sum{
            0..10 => {},
            10..19 =>{
                sum -= 10;
                cx.mark(at + 1);
                other.mark_up(cx);
            },
            _ => panic!("This shouldnt happen")
        };
        self.set(side, &sum);
        other.set(&sum);
    }
    pub fn sum_with_self(&mut self, side: &Side, at: usize, cx: &mut SieveFunctionContext){
        let mut sum = self.get(side) * 2;
        match sum {
            0..10 => {},
            10..19 => {
                sum -= 10;
                cx.mark(at + 1);
            },
            _ => panic!("This shouldnt happen")
        };
        self.set(side, &sum);
    }
}
pub struct SharedDigitPair{ //actual data held
    pub data: Arc<RwLock<u8>>
}