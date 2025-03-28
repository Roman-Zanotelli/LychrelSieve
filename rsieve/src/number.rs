use std::{default, os::unix::process};

//I could use a u128 for upto 340282366920938463463374607431768211455 but lychrel candidates will quickly outgrow them
//Instead im going to use a Vec<u8> assigning each digit as 4 bits 0000 (2^4 = 16 total combinations per digit)
//to keep it simple 0000 = 0, 0001 = 1, 0010 = 2, 0011 = 3, 0100 = 4, 0101 = 5, 0110 = 6, 0111 = 7, 1000 = 8, 1001 = 9; this leaves 1010, 1011, 1100, 1101, 1110, 1111
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

//for comparison u128 (128 bits) holds at max 39 digits, which can be handled by a vec of 19.5 u8s (160 bits) but can extend past the innate hard cap of 340282366920938463463374607431768211455
//and in addition a u128 can only fully represent 38 digits with the 39th leading digit only being able to reach 3, while my system can fully represent 40 digits within the 160 bits
pub struct BigMemoryInt{
    data: Vec<u8>
}

impl Default for BigMemoryInt{
    fn default() -> Self {
        Self { data: Default::default() }
    }
}
impl BigMemoryInt{
    pub fn increment(&mut self){
        todo!()
    }
}

impl crate::iterative_process::Process for BigMemoryInt {
    fn process_memory_bits(&mut self) -> bool{
        self.data.process_memory_bits()
    }
}
impl crate::iterative_process::Process for Vec<u8>{
    fn process_memory_bits(&mut self) -> bool{
        todo!()
    }
}