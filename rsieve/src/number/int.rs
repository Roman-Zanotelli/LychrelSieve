use std::sync::Arc;

use tokio::sync::RwLock;

use super::{digit::Digit, shared_digit_pair::SharedDigitPair};

pub struct BigInt{
    pub(crate) data: Arc<RwLock<Vec<Digit>>>
}
impl From<String> for BigInt{
    fn from(mut value: String) -> Self {
        if value.len() % 2 == 1{
            value = "_".to_string() + &value;
        }
        BigInt { data: Arc::new(RwLock::new(value.chars().map(|char|{if char == '_' { return 0b1100} (unsafe { char.to_digit(10).unwrap_unchecked() }) as u8}).collect::<Vec<u8>>().chunks(2).map(|chunk| SharedDigitPair{ data : Arc::new(RwLock::new(chunk[0] << 4 | chunk[1]))}).collect::<Vec<SharedDigitPair>>().from_pairs())) }
    }
}
trait IntoDigitVec{
    fn from_pairs(self) -> Vec<Digit>;
}

impl IntoDigitVec for Vec<SharedDigitPair>{
    fn from_pairs(mut self) -> Vec<Digit> {
        let mut res = Vec::<Digit>::new();
        self.reverse();
        let mut at= 0;
        for pair in self{
            match pair.gen_digit_refs(at){ //gens 2 pairs
                Ok((left, right)) => {res.push(right); res.push(left);},
                Err((_, right)) => {res.push(right);},
            }
            at += 2;
        }
        res
    }
}