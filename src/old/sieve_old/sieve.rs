
use crate::{computation::comp::CompWrapper, shared_number::int::SharedInt};

use super::sieve_function::SieveFunction;

struct Sieve{
    seed : SharedInt,
}
impl Sieve{
    async fn start(&self) -> u128{
        self.seed.print_as_digits();
        loop {
            match SieveFunction::iterate( CompWrapper::from(&self.seed).await).await.carry_ones().await.is_palindrome().await{
                (false, _) => {self.seed.print_as_digits(); continue}, 
                (true, func) => {
                    self.seed.print_as_digits();
                    return func.get_iterations()
                }
            }
        } 
    }
    fn load(seed: SharedInt) -> Self{
        Sieve { seed}
    }
}
