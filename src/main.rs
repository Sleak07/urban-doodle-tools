//TODO: Print hello world in rust

fn main() {
    println!("Hello world");

    //variable in rust
    let bunnies: i32 = 4;
    println!("{} bunnies", bunnies);

    //constants in rust
    const BUNNIES: i32 = 4;
    println!("{} bunnies", BUNNIES);

    //scope in rust
    let x = 6;
    {
        let x = 89;
        println!("{} x", x);
    }
    println!("{} x", x);

    //variable shadowing
    /*
    * in variable shadowing replace the value of
    * first one with second one
    * */

    let x = 9;
    let x = x + 1;
    println!("{} x", x);
}
