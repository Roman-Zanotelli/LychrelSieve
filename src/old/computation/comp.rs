use std::{sync::Arc, task::Poll};

use futures::Stream;

use crate::{shared_number::{digit::SharedDigit, int::SharedInt}, sieve_old::sieve_function::SieveFunction};


pub struct Computation{
    data : (SharedDigit, Option<SharedDigit>)
}
impl Computation{
    pub fn from(data: (SharedDigit, Option<SharedDigit>)) -> Self{
        Computation { data }
    }
}
impl Computation{
    pub async fn compute(&self, func: SieveFunction){
        match &self.data{
            (left, None) => {
                left.compute_with_self(func).await
            },
            (left, Some(right)) => {
                left.compute(right, func).await
            }
        }
    }
}
pub struct CompWrapper{
    data : Vec<Computation>
}
impl CompWrapper{
    pub async fn from(seed: &SharedInt) -> Self{
        CompWrapper {data: (seed.as_vec_ref_pair().await)}
    }
}
impl Stream for CompWrapper{
    type Item = Computation;
    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> { 
        Poll::Ready(self.data.pop())       
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0 , Some(self.data.len()))
    }
}