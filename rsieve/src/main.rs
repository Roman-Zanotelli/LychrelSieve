use int_vec::{BitNumber, BitOp, IntVec};

pub mod int_vec;
fn main() {
    let mut test = IntVec::default();
    for i in 0..u32::MAX{
        println!("{}", i);
        println!("{}", test.as_string());
        test.increment();
    }
    let mut test2 :u8 = 0b10100000;
    println!("{}", test2);
    test2 &= !(0b1111 << 4);
    println!("{}", test2);
}
