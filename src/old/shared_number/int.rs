use std::{str::Chars, sync::Arc};

use tokio::sync::RwLock;

use crate::computation::comp::Computation;

use super::digit::{self, SharedDigit, SharedDigitPair};

pub struct SharedInt{
    leading_space: Option<SharedDigit>,
    data: Arc<RwLock<Vec<SharedDigit>>>,
}
impl SharedInt{
    pub fn print_as_digits(&self){
        println!("d");
        let mut read = self.data.blocking_read().clone();
        read.reverse();
        for digit in read {
            match digit.to_char(){
                Some(char) => print!("{}", char),
                None => continue,
            }
            
        }
        print!("\n")
    }
    fn new((leading_space, data) : (Option<SharedDigit>, Arc<RwLock<Vec<SharedDigit>>>)) -> Self{
        SharedInt { leading_space, data }
    }
    pub fn load_string(seed: &String) -> Self{
        SharedInt::new(match (seed.len(), seed.len() % 2 != 1){
            (0, _) =>{
                (None, Arc::new(RwLock::new(Vec::<SharedDigit>::new())))
            }
            (_, true) => {
                let mut data = Vec::<SharedDigit>::new();
                let mut collect = seed.chars().collect::<Vec<char>>().chunks(2).map(|chunk| { Arc::new(RwLock::new(SharedDigitPair {data : unsafe {(chunk[0].to_digit(10).unwrap_unchecked() as u8) << 4 | (chunk[1].to_digit(10).unwrap_unchecked() as u8)}}))}).collect::<Vec<Arc<RwLock<SharedDigitPair>>>>();
                for digit in collect{
                    data.push(SharedDigit::new(digit.clone(), &digit::Side::LEFT));
                    data.push(SharedDigit::new(digit, &digit::Side::RIGHT));
                }
                data.reverse();
                (None, Arc::new(RwLock::new(data)))
            },
            (_, false) => {
                let mut data = Vec::<SharedDigit>::new();
                let mut collect = ("X".to_owned() + seed).chars().collect::<Vec<char>>().chunks(2).map(|chunk| match (chunk[0], chunk[1]){
                    ('X', right) => {
                        Arc::new(RwLock::new(SharedDigitPair {data : unsafe {(0b1011 as u8) << 4 | (right.to_digit(10).unwrap_unchecked() as u8)}}))
                    },
                    (left, right) => {
                        Arc::new(RwLock::new(SharedDigitPair {data : unsafe {(left.to_digit(10).unwrap_unchecked() as u8) << 4 | (right.to_digit(10).unwrap_unchecked() as u8)}}))
                    }
                }).collect::<Vec<Arc<RwLock<SharedDigitPair>>>>();
                let front = collect.remove(0);
                data.push(SharedDigit::new(front.clone(), &digit::Side::RIGHT));
                for digit in collect{
                    data.push(SharedDigit::new(digit.clone(), &digit::Side::LEFT));
                    data.push(SharedDigit::new(digit, &digit::Side::RIGHT));
                }
                data.reverse();
                (Some(SharedDigit::new(front, &digit::Side::LEFT)), Arc::new(RwLock::new(data)))
            },
         })
    }
    pub async fn as_vec_ref_pair(&self) -> Vec<Computation>{
        let data = self.data.read().await;
        let len = data.len();
        match (len, len % 2){
            (0, _) => return Vec::new(),
            (1, _) => vec![Computation::from((unsafe{data.get_unchecked(0)}.to_owned(), None))],
            (len, 0) => { //EVEN
                let mut res: Vec<Computation> = Vec::new();
                for i in 0..len/2{
                    res.push(Computation::from(unsafe { (data.get_unchecked(i).clone(), Some(data.get_unchecked(len - 1 - i).to_owned()))}));
                }
                res
            },

            (len, 1) => { //ODD
                let mut res: Vec<Computation> = Vec::new();
                for i in 0..len/2{
                    res.push(Computation::from(unsafe { (data.get_unchecked(i).clone(), Some(data.get_unchecked(len - 1 - i).to_owned()))}));
                }
                res.push(Computation::from((unsafe {data.get_unchecked(len/2 + 1).to_owned()}, None)));
                res
            }
            (_, _) => return Vec::new()
        }
    }
}
