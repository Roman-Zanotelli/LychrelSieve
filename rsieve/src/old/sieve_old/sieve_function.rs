use std::sync::{Arc};

use futures::StreamExt;
use tokio::{signal, spawn, sync::{Notify, RwLock}};

use crate::{computation::comp::CompWrapper, shared_number::int::SharedInt};



pub struct SieveFunction{
    is_palindrome: Arc<RwLock<bool>>,
    notify: Arc<Notify>,
    iterations: Arc<u128>
}
impl Clone for SieveFunction {
    fn clone(&self) -> Self {
        Self { is_palindrome: self.is_palindrome.clone(), notify: self.notify.clone(), iterations: self.iterations.clone() }
    }
}
impl Default for SieveFunction{
    fn default() -> Self {
        Self { notify: Arc::new(Notify::new()), is_palindrome: Arc::new(RwLock::new(true)), iterations: Arc::new(0)  }
    }
}
impl SieveFunction{
    pub fn get_iterations(self) -> u128{
        *self.iterations
    }
    pub async fn iterate(mut wrapper : CompWrapper) -> Self{
        let func = SieveFunction::default();
        loop {match wrapper.next().await{
            Some(val) => {
                let func_clone = func.clone();
                spawn(async move {val.compute(func_clone).await});
            },
            None => {
                return func
            },
        }}
    }
    pub async fn carry_ones(self) -> Self{
        self.notify.notify_waiters();
        self
    }
    pub async fn is_palindrome(self) -> (bool, Self){
        let res = *self.is_palindrome.read().await;
        (res, self)
    }
    pub fn get_signal(&self) -> Arc<Notify>{
        self.notify.clone()
    }
}