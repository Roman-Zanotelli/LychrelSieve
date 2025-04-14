use std::{env, u128};

use dotenvy::dotenv;
use lychrel_sieve::{master_sieve::MasterSieve, sieve::{SeededSieve, SieveFunctionContext}};
use number::int::BigInt;

pub mod number;
pub mod lychrel_sieve;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let cfg = (env::var("START_NUMBER").unwrap_or("10".to_string()),env::var("START_ITER").unwrap_or("0".to_string()), env::var("MAX_ITER").unwrap_or("300".to_string()), env::var("MAX_NUMBER").unwrap_or("2000".to_string()), env::var("MAX_CONCURRENT_SEEDS").unwrap_or("1".to_string()));
    
    match env::var("SINGLE_NUMBER_ONLY").unwrap_or("true".to_string()).parse::<bool>().unwrap(){
        true => SeededSieve::new_start_single(BigInt::from(cfg.0).await, cfg.1.parse::<u128>().unwrap(), cfg.2.parse::<u128>().unwrap()).await,
        false => MasterSieve::new_start(cfg.0.parse::<u128>().unwrap(), cfg.2.parse::<u128>().unwrap(), cfg.4.parse::<usize>().unwrap(), cfg.3.parse::<u128>().unwrap()).await,
    }
    
    // let test_string :String = "12000700000025339936491".to_owned();
    // println!("{}", test_string);
    // let mut context = SieveFunctionContext::default(); 
    // let int = BigInt::from(test_string).await;
    // let mut sieve = SeededSieve::new(int, 0);
    
    // loop{
    //     if sieve.iterate(&mut context).await{
    //         println!("Iteration {}: {}", sieve.get_it(), sieve.to_digit_string().await);
    //         break
    //     }
    //     println!("Iteration {}: {}", sieve.get_it(), sieve.to_digit_string().await);

    // }
    // println!("{}", sieve.to_digit_string().await);
    // while !sieve.iterate(&mut context).await{
    //     println!("Iteration: {}, Number: {}", sieve.get_it(), sieve.to_digit_string())
    // }
    
    // let test_delayed = BigInt::from("1000206827388999999095750".to_string()).await;
    // SeededSieve::new_start_single(test_delayed, 0, u128::MAX - 1).await;
    // let test_196 = BigInt::from("196".to_string()).await;
    //SeededSieve::new_start_single(test_196, 0, 1000).await;

    // MasterSieve::new_start(10, 333, 1000).await;


}
