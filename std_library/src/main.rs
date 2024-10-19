//----------------------------------------------------
//            Standard Library
//----------------------------------------------------

use std_library::{example_01, example_02_a, example_02_b, example_02_c, example_02_d};

fn main() {
    println!();
    println!("-------------------------------------------------");
    println!("            Standard Library");
    println!("-------------------------------------------------");

    println!();
    println!("########### Example 01 ###########");
    let _ = example_01::main();

    println!();
    println!("########### Example 02_A - Arrays ###########");
    example_02_a::main();

    println!();
    println!("########### Example 02_B - Vectors ###########");
    example_02_b::main();

    println!();
    println!("########### Example 02_C - HashMap ###########");
    example_02_c::main();

    println!();
    println!("########### Example 02_D - Queues - VecDeque ###########");
    example_02_d::main();
}