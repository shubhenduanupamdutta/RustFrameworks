//----------------------------------------------------
//            Standard Library
//----------------------------------------------------

use std_library::{cli_example, datetime_examples, example_01, example_02_a, example_02_b, example_02_c, example_02_d, server_examples};

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

    println!();
    println!("-------------------------------------------------");
    println!("              DateTime Examples");
    println!("-------------------------------------------------");
    datetime_examples::main();

    println!();
    println!("-------------------------------------------------");
    println!("              CLI, Environment Variables Examples");
    println!("-------------------------------------------------");
    cli_example::main();

    println!();
    println!("-------------------------------------------------");
    println!("              Server Examples");
    println!("-------------------------------------------------");
    server_examples::main();
}