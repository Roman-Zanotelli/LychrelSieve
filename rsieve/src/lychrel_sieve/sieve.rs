use std::sync::{Arc, RwLock};

use dashmap::DashSet;
use futures::{future::join_all, StreamExt};
use tokio::{spawn, task::JoinHandle};

use crate::number::int::BigInt;

use super::view::IntoView;

struct Sieve{
    seed: BigInt,
    iterations: u128
}
impl Sieve{
    fn new(seed : BigInt, iterations: u128) -> Self{
        Sieve{seed, iterations}
    }
    async fn iterate(&mut self, context: &mut SieveFunctionContext) -> bool{
        self.iterations += 1;
        let mut handles = Vec::<JoinHandle<()>>::new();
        loop {
            let mut cx = context.clone();
            match self.seed.into_view().next().await{
                Some(val) => match val {
                    (None, None) => break,
                    (None, Some(mut right)) => handles.push(spawn(async move {right.sum_with_self(&mut cx)})),
                    (Some(mut left), None) => handles.push(spawn(async move {left.sum_with_self(&mut cx)})),
                    (Some(mut left), Some(mut right)) => handles.push(spawn(async move {left.sum(&mut right, &mut cx)})),
                },
                None => break,
            };
        };
        join_all(handles).await;
        return context.carey_ones().await
    }
    
}

pub struct SieveFunctionContext{
    carry: Arc<DashSet<usize>>
}

impl SieveFunctionContext {
    async fn carey_ones(&mut self) -> bool {
        todo!()
    }
    pub fn mark(&mut self, at :usize){
        self.carry.insert(at);
    }
}
impl Default for SieveFunctionContext{
    fn default() -> Self {
        Self { carry: Arc::new(DashSet::new())  }
    }
}
impl Clone for SieveFunctionContext{
    fn clone(&self) -> Self {
        Self { carry: self.carry.clone() }
    }
}