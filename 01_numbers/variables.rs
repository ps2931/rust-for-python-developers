fn main() {
    let x = 1;
    println!("x: {}", x);
    // binds x again, shadowing the old one from above
    let x = 'A';
    println!("x: {}", x);

    // declare, initialize
    let something;
    let x = 5;
    println!("x: {}, something: {}", x, something);
    // not allowed to access uninitialized variable
    // so above println! will throw error

    something = x * 5; // initialized, but will not work
                       // as we are using the variable beofre initialization
    println!("x: {}, something: {}", x, something);

    // Immutability is the default behavior.
    // Once initialized a variable's value cannot
    // be changed.
    let y = 0; // make it mutable
    y = y * 2 + x;
    dbg!(y); //

    const BLAH: i32 = 42; // convention is to use UPPERCASE for constant
    y *= blah; // same problem as above
    dbg!(y);
}
