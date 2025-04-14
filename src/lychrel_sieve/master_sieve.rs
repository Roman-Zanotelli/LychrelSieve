use std::sync::Arc;

use futures::future::join_all;
use tokio::{spawn, sync::{RwLock, Semaphore}};

use crate::number::{int::BigInt, shared_digit_pair::SharedDigitPair};

use super::sieve::SeededSieve;

pub struct MasterSieve{
    master_seed: u128,
    stop_at: u128,
    max_sieves: usize,
    max_seed: u128
}
struct MasterSeed{ //representation of BigInt used by set sieve to create incremental sieve seeds
    data: Vec<u8>
}

impl MasterSieve{
    pub fn new(master_seed: u128, stop_at: u128, max_sieves: usize, max_seed: u128) -> MasterSieve{
        MasterSieve { master_seed, stop_at, max_sieves, max_seed}
    }
    pub async fn start(&mut self){
        let slots = Arc::new(Semaphore::new(self.max_sieves));
        let mut handles = Vec::new();
        loop {
            let permit = slots.clone().acquire_owned().await.unwrap();
            handles.push(spawn( SeededSieve::new_start( BigInt::from(self.master_seed.to_string()).await, 0, self.stop_at, permit)));
            self.master_seed += 1;
            if self.master_seed > self.max_seed{
                break
            }
        };
        join_all(handles).await;
    }
    pub async fn new_start(master_seed: u128, stop_at: u128, max_sieves: usize, max_seed: u128){
        MasterSieve::new(master_seed, stop_at, max_sieves, max_seed).start().await
    }
    
}


impl MasterSeed{
    pub async fn into_big_int(&self) -> BigInt{
        let pairs = self.data.iter().map(|pair| SharedDigitPair{ data: Arc::new(RwLock::new(*pair)) });
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
    pub fn increment(&mut self){
        let mut index = 0;
        loop{
            match self.data.get_mut(index){
                Some(pair) => {
                    if pair.increment(){
                        break;
                    }
                },
                None => self.data.push(0b11110001 as u8),
            }
            index  += 1;
        }
    }
    pub fn from(value: &String) -> MasterSeed{
        MasterSeed{ 
            data: value.chars().rev().map(|char|char.to_digit(10).unwrap() as u8).collect::<Vec<u8>>().chunks(2).map(|chunk|
                match (chunk.get(1), chunk.get(0)) {
                    (Some(left), Some(right)) => {
                        left << 4 | right
                    },
                    (None, Some(right)) => {
                        (!0b1111) | right
                    },
                    _ => panic!("This should never happen")
                }
            ).collect::<Vec<u8>>()
        }
    }
}


trait SyncBitWise{
    fn increment(&mut self) -> bool;   
}
impl SyncBitWise for u8{
    fn increment(&mut self) -> bool {
        let right = (*self & 0b1111) + 1;
        match right{
            0..10 => {
                *self = (*self & !0b1111) | right;
            },
            10 => { //if right is 10 it can basically be ignored becaus it needs to be treated as a 0 digit
                let left = (*self >> 4) + 1;
                match left{
                    0b00010000 => { //this result should only occur if the digit was previously a placeholder
                        *self = 0b00010000;
                    }, //left was a placeholder value
                    0..10 => {
                        *self = left << 4;
                    },
                    10 => {
                        *self = 0; //this happens if both numbers roll over meaning the pair would be X00000000X
                        return false
                    },
                    _ => panic!("This should never happen")
                }
            }
            _ => panic!("This should never happen")
        };
        return true
    }
}
