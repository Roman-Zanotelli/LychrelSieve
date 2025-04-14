
use std::sync::Arc;

use futures::{future::join_all, StreamExt};
use tokio::{spawn, sync::{OwnedSemaphorePermit, RwLock, Semaphore}};

use crate::number::int::BigInt;


pub struct SeededSieve{
    seed: BigInt,
    iterations: u128
}
impl SeededSieve{
    pub fn new(seed : BigInt, iterations: u128) -> Self{
        SeededSieve{seed, iterations}
    }
    pub async fn iterate(&mut self, context: &mut SieveFunctionContext) -> bool{
        self.iterations += 1;
        let mut view = self.seed.into_view().await;
        loop {
            let context = context.clone();
            match view.next().await{
                Some(val) => match val {
                    (None, Some(right)) => right.sum_with_self(&context).await,
                    (Some(left), None) => left.sum_with_self(&context).await,
                    (Some(left), Some(right)) => left.sum(&right, &context).await,
                    _ => panic!("This should never happen")
                },
                None => break,
            };
        };
        self.seed.carry_ones(context).await;
        self.seed.is_palindrome().await
    }
    pub async fn to_digit_string(&self) -> String{
        self.seed.to_string().await
    }
    pub fn get_it(&self) -> u128{
        return self.iterations
    }
    pub async fn new_start(seed : BigInt, iterations: u128, stop_at: u128, permit: OwnedSemaphorePermit){
        let mut sieve = SeededSieve::new(seed, iterations);
        let original_seed = sieve.to_digit_string().await;
        let mut context = SieveFunctionContext::default();
        while !sieve.iterate(&mut context).await{
            if sieve.get_it() == stop_at{
                println!("Found Possible Candidate: {}\n Current Iteration: {}\n Current Value: {}", original_seed, sieve.get_it(), sieve.to_digit_string().await);
                return
            }
        }
        if sieve.get_it() >= 293{
            println!("Reached Palindrome of 293+ iterations with seed: {}", original_seed)
        }
    }
    pub async fn new_start_single(seed : BigInt, iterations: u128, stop_at: u128){
        let mut sieve = SeededSieve::new(seed, iterations);
        let original_seed = sieve.to_digit_string().await;
        let mut context = SieveFunctionContext::default();
        while !sieve.iterate(&mut context).await{
            println!("Current Iteration: {}, Current Value: {}", sieve.get_it(), sieve.to_digit_string().await);
            if sieve.get_it() == stop_at{
                println!("Max Iterations reached");
                return
            }
        }
        println!("Palindrome Reached at {} iterations, with the value: \n{}", sieve.get_it(), sieve.to_digit_string().await)
    }
}

pub struct SieveFunctionContext{
    pub carry: Arc<RwLock<Vec<usize>>>
}

impl SieveFunctionContext {
    pub async fn mark(&self, at :usize){
        self.carry.write().await.push(at);
    }
    pub async fn get_carry(&mut self) -> Vec<usize>{
        
        let res =  self.carry.read().await.iter().map(|digit| *digit).collect();
        self.carry.write().await.clear();
        return res;
    }
    pub async fn is_empty(&self) -> bool{
        self.carry.read().await.is_empty()
    }
}
impl Default for SieveFunctionContext{
    fn default() -> Self {
        Self { carry: Arc::new(RwLock::new(Vec::new()))  }
    }
}
impl Clone for SieveFunctionContext{
    fn clone(&self) -> Self {
        Self { carry: self.carry.clone() }
    }
}