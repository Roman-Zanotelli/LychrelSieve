pub mod number;
pub mod lychrel_sieve;
fn main() {


    let test_string :String = "564".to_owned();
    println!("{}", test_string);
    // u8, u16, u32, u64, u128
    // 00000000 = 0;
    // 0b0 = 0;
    // 0b1 = 1;
    // 1 + 1 = 0;

    // U8
    // U4 + U4 = U8;

    // 0000 0
    // 0001 1
    // 0010 2
    // 0011 3
    // 0100 4
    // 0101 5
    // 0110 6 
    // 0111 7 
    // 0b1000 8
    // 0b1001 9
    // 0b1010  

    // 99 != 10011001;

    // 0b10 = 2;
    // 0b1001 = 9;
    // 0b1010 = 10;
    // 0b1 = 00000001;
    // 0b10 = 00000010 = 2;

    // let mut test = IntVec::load_string(&test_string);
    // println!("{}",test_string);
    // // println!("{}",test.as_string());
    // println!("{}", test.as_string_of_digits());
    // println!("Testing Increment:");
    // test.increment();
    // // println!("{}",test.as_string());
    // println!("{}", test.as_string_of_digits());
    // println!("Testing Increment:");
    // test.increment();
    // // println!("{}",test.as_string());
    // println!("{}", test.as_string_of_digits());
    // test.iterate_bits();
    // println!("{}", test.as_string_of_digits());

    // for i in 0..15{
    //     println!("{}", i);
    //     println!("{}", test.as_string());
    //     test.increment();
    // }



    // let mut test2 :u8 = 0b10100000;
    // println!("{}", test2);
    // test2 &= !(0b1111 << 4);
    // println!("{}", test2);
    // let testu8 : u8 = 0b1111;
    // let test = (testu8 & !(0b1111)) != 0;
    // println!("{}",test) 

}
