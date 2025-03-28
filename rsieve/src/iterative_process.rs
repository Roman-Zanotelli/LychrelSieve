use crate::number::{BigMemoryInt};




pub trait Process { //the process modifies the data memory directly and does not create any additional copies 
    fn process_memory_bits(&mut self) -> bool; //singles true when palindrome (allows me to run it in a loop easier)
}
struct LychrelIterative{//data structure that holds the current iteration state and amount of iterations
    data: BigMemoryInt, //actual number we are testing
    iterations: BigMemoryInt //amount of iterations weve completed
}
impl Process for LychrelIterative {
    fn process_memory_bits(&mut self) -> bool {
        self.iterations.increment(); //increment the total iterations for this number
        self.data.process_memory_bits()
    }
}