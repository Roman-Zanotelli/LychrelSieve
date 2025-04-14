
use std::{slice, sync::Arc, task::Context, usize};

use futures::{future::join_all, FutureExt};
use tokio::{spawn, sync::RwLock};

use crate::lychrel_sieve::sieve::SieveFunctionContext;

use super::{digit::Digit, shared_digit_pair::SharedDigitPair};

pub struct BigInt{
    pub(crate) data: Arc<RwLock<Vec<Digit>>>
}
impl BigInt{
    pub async fn from(value: String) -> Self {
        let pairs = value.chars().rev().map(|char| char.to_digit(10).unwrap() as u8).collect::<Vec<u8>>().chunks(2).map(|chunk|{
            SharedDigitPair{
                data: Arc::new(RwLock::new(match (chunk.get(1), chunk.get(0)){
                    (Some(left), Some(right)) =>{ //smashes 2 u4 values into a single u8
                        left << 4 | (right & 0b1111) //shifts left u4 4 spaces left 0000LLLL -> LLLL0000; then combines with the right into a single u8 LLLL0000 -> LLLLRRRR
                    },
                    (None, Some(right) ) => { //formates a single u4 value
                        (!0b1111) | right & 0b1111 //1111 is being used as a left placeholder u4 combined with right u4 11110000 -> 1111RRRR
                    },

                    _ => panic!("This shouldnt happen")
                }))
            }

        }).collect::<Vec<SharedDigitPair>>();
        let mut res    = Vec::new();
        let mut at :usize = 0;
        for pair in pairs{
            pair.from_pairs(&mut res, at).await;
            at += 2;
        }   
        BigInt { 
            data: Arc::new(RwLock::new(
                res
            ))
        }
    }
    pub async fn to_string(&self) -> String{
        let mut res = Vec::new();
        for digit in &*self.data.read().await{
            res.push(digit.to_char().await);
        };
        res.reverse();
        res.into_iter().collect()
    }
    
    pub(crate) async fn is_palindrome(&self) -> bool {
        let read = self.data.read().await;
        let mut res = true;
        for i in 0..read.len()/2{
            res = res & (read.get(i).unwrap().get().await == read.get(read.len()-1-i).unwrap().get().await);
        };
        return res
    }
    
    pub(crate) async fn carry_ones(&self, context: &mut SieveFunctionContext){
        
        while !context.is_empty().await{

            for digit_space in context.get_carry().await{
                let context = context.clone();
                let clone = self.clone();
                clone.increment_at(digit_space, context).await;
            }
        }
        
    }
    pub async fn increment_at(self, at: usize, cx: SieveFunctionContext){
        let read = self.data.read().await;
        match read.get(at){
            Some(digit) => {
                digit.increment(&cx).await;
            },
            None => {
                let next = match read.len() % 2 != 1{
                    true => Digit::new(&at).await,
                    false => read.last().unwrap().generate_next_digit().await,
                };
                drop(read);
                self.data.write().await.push(next);
            },
        }
    }
}
trait IntoDigitVec{
    fn from_pairs(self) -> Vec<Digit>;
}

impl SharedDigitPair{
    pub async fn from_pairs(self,  res: &mut Vec<Digit>, at: usize){
        match self.gen_digit_refs(at).await{ //gens 2 pairs
                (Some(left), right) => {res.push(right); res.push(left);},
                (None, right) => res.push(right),
        }
    
    }
}