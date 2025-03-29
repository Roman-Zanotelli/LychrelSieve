//I could use a u128 for upto 340282366920938463463374607431768211455 but lychrel candidates will quickly outgrow them
//Instead im going to use a Vec<u8> assigning each digit as 4 bits 0000 (2^4 = 16 total combinations per digit)
//to keep it simple 0000 = 0, 0001 = 1, 0010 = 2, 0011 = 3, 0100 = 4, 0101 = 5, 0110 = 6, 0111 = 7, 1000 = 8, 1001 = 9; this leaves 1010 (ROLL OVER), 1011, 1100, 1101, 1110, 1111
//as command codes for logic such as END-OF-NUMBER, WAIT-UNTIL-AVAILABLE, and other special values for when large numbers need to be saved/read from filesystem rather than in memory

//this structure wil allow simple bit operations under a single digit for example 0000 + 0001 = 0001 = 1 or 0101 + 0100 = 1001 = 9

//the bit math does slightly break down for intergers that sum to double digits, this can be handles by roling over the current digit space to 0000
//and adding 0001 to the digit infront

//in general since u8 is the smallest amount of bits i can hold most digits will be held in pairs of 2 00001001 = 09, 10010000 = 90, etc
//in the event a leading digit of the u8 pair surpases 1001 (9) it will roll over the same, but add 0001 to the 2nd digit of the prior u8 inside the data Vec<u8> or append a new u8 (00000001) to the front

//each digit operation will act independently of other digits when adding, the only situation where a digit's operations will interact with another digit is when carrying a 0001 digit
//similar to regular base 10 math where you carry the 1 when adding 9+6 = 15 = 10 + 5, the 1 is carried to the 10's digit space

//When conducting the iterative process only 2 u8's are required but to better handle carried values loading 4 u8s or 2 sliding u16 will be used

//This architecture will be critical to the processing of Lychrel candidates of a high order/iteration

use std::{path::Path, u8};
const BASE : u32 = 10;

//for comparison u128 (128 bits) holds at max 39 digits, which can be handled by a vec of 19.5 u8s (160 bits) but can extend past the innate hard cap of 340282366920938463463374607431768211455
//and in addition a u128 can only fully represent 38 digits with the 39th leading digit only being able to reach 3, while my system can fully represent 40 digits within the 160 bits
pub trait BitNumber {
    
    fn increment(&mut self) -> bool; //increments underlying data
    fn iterate_bits(&mut self) -> (bool, bool); //singles true when palindrome (allows me to run it in a loop easier)
    fn load_number(&mut self, at: &i128);
    fn load_string(&mut self, at: &String);
    fn load_file(&mut self, at : &Path); //load binary file
    fn load_file_as_number(&mut self, at : &Path); //load digit file
    fn write(&self, path : &Path)-> Result<(), ()>; //write number as binary
    fn write_as_number(&self, path : &Path)-> Result<(), ()>; //write number as digits
    fn as_string(&self) -> String; //get number as binary string
    fn as_string_of_digits(&self) -> String; //get number as digit string
}

pub struct IntVec{
    data : Vec<u8>
}
impl Default for IntVec{
    fn default() -> Self {
        Self { data: vec![0b0000] }
    }
}

impl BitNumber for IntVec{
    fn increment(&mut self) -> bool{
        self.data.increment()
    }
    fn iterate_bits(&mut self) -> (bool, bool) {
        self.data.iterate_bits()
    }

    fn load_number(&mut self, at: &i128) {
        self.data.load_string(&at.to_string());
    }

    fn load_file(&mut self, path : &Path) {
        self.data.load_file(path);
    }

    fn as_string(&self) -> String{
        self.data.as_string()
    }
    fn as_string_of_digits(&self) -> String{
        self.data.as_string_of_digits()
    }
    
    fn write(&self, path : &Path) -> Result<(), ()>{
        self.data.write(path)
    }
    
    fn write_as_number(&self, path : &Path) -> Result<(), ()>{
        self.data.write_as_number(path)
    }
    
    fn load_file_as_number(&mut self, path : &Path) {
        todo!()
    }
    
    fn load_string(&mut self, at: &String) {
        self.data.load_string(at);
    }
}


impl BitNumber for Vec<u8> {
    fn increment(&mut self) -> bool {
        let len = self.len();
        let mut pin : usize = 0;
        loop {
            match self.get_mut(len - 1 - pin){
                Some(byte) => {
                    if byte.increment(){
                        pin  += 1;
                        match (pin != len, len == usize::MAX - 1){
                            (true, _) => continue, //if there are values proceeding
                            (false, at_limit) => {
                                self.insert(0, 0b1); 
                                break at_limit; //signals to outer process that this number is at its byte limit
                            }
                        }
                    }
                    break false;
                },
                None => panic!("Something went wrong"),
            }    
        }
    }

    fn iterate_bits(&mut self) -> (bool, bool) {
        let len = self.len();
        for i in 0 .. len{

        }
        todo!()
    }

    fn load_number(&mut self, at: &i128) {
        self.load_string(&at.to_string());
    }

    fn load_file(&mut self, path: &Path) {
        todo!()
    }
    fn load_file_as_number(&mut self, path : &Path) {
        todo!()
    }
    
    fn write(&self, path : &Path) -> Result<(), ()>{
        todo!()
    }
    
    fn write_as_number(&self, path : &Path) -> Result<(), ()>{
        todo!()
    }
    
    fn as_string_of_digits(&self) -> String {
        let mut result = "".to_string();
        for bits in self {
            result = format!("{}{}", result, bits.as_string_of_digits());
        }
        return result;
    }
    fn as_string(&self) -> String {
        let mut result = "".to_string();
        for bits in self {
            result = format!("{}{}", result, bits.as_string());
        }
        return result;
    }
    
    fn load_string(&mut self, at: &String) {
        let mut digits = at.chars().collect::<Vec<char>>();
        digits.reverse();
        for reversed_digit_pair in digits.chunks(2){
            //construct binary representation
            match (reversed_digit_pair.get(0), reversed_digit_pair.get(1)){
                (Some(right), Some(left)) =>{
                    match (&left.to_digit(BASE), &right.to_digit(BASE)){
                        (Some(left_digit), Some(right_digit)) =>self.insert(0, ((*left_digit as u8) << 4) | *right_digit as u8), //combines the digits in my format (the left digit is shifter 4 to the right 00001001 = 10010000 then adds right digit 10010000 | 00000110 = 10010110)    
                        (Some(_), None) => panic!("Right Char Invalid: {}", right),
                        (None, Some(_)) => panic!("Left Char Invalid: {}", left),
                        (None, None) => panic!("Both Char Invalid: {} {}",left, right),
                    }
                }
                (Some(right), None) => {
                    match right.to_digit(BASE) {
                        Some(digit) => self.insert(0, digit as u8),
                        None => panic!("Invalid Character"),
                    }
                }
                _ => panic!("Chunks didnt work?")
            }
        }
    }
    
    
}



pub trait BitOp{
    fn increment(&mut self) -> bool; //true if carry 1
    fn increment_at(&mut self, at : u8) -> bool; //increment at digit place
    fn as_string(self) -> String; //debug as string
    fn as_string_of_digits(self) -> String; //debug as string


    fn iterate_bits_with_self(&mut self) -> bool;
}
impl BitOp for u8{
    fn increment(&mut self) -> bool {
        let mut i : u8 = 0;
        loop {
            println!("{}", i);
            *self ^= 0b1 << i; //working as intended
            match (*self & (1 << i) != 0) {
                true => { 
                    match i % 4 == 1 && *self & (1 << i+2) != 0{
                        true => { //ROLL OVER
                            *self &= !(0b1111 << (i-i%4)); //WIPE
                            match i > 4 {
                                true => break true, //carry 1 outside
                                false => {
                                    i += (4 - i%4);
                                    continue; //carry 1 inside
                                },
                            }
                        },
                        false => break false, //DONE
                    }}, //changed bit to 1
                false => {i += 1; continue;}, //turned bit into a 0 
            }
        }
    }

    fn increment_at(&mut self, at : u8) -> bool {
        todo!()
    }
    
    fn as_string_of_digits(self) -> String {
        let mut result = "".to_string();
        for i in (0..128).step_by(4).rev() {
            result = format!("{}{}", result, match ((self >> (i)) & 0b1111) {   
                0b0000 => "0",
                0b0001 => "1",
                0b0010 => "2",
                0b0011 => "3",
                0b0100 => "4",
                0b0101 => "5",
                0b0110 => "6",
                0b0111 => "7",
                0b1000 => "8",
                0b1001 => "9",
                _ => panic!("Woah I Messed Up")
            });
        };
        return result;
    }
    fn as_string(self) -> String {
        let mut result = "".to_string();
        
        for i in (0..8).rev() {
            result = format!("{}{}", result, (self >> i & 0b1) );
        };
        return result;
    }
    
    fn iterate_bits_with_self(&mut self) -> bool {
        match (*self >> 4, (*self << 4) >> 4){
            (left, right) => {
                //I can just add them together like a regular u8 and then process it back into my digit format (and carry any 1's) 
                
            }
        }
        todo!()
    }
    
}