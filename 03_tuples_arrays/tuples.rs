fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (a_int, b_bool) = pair;
    (b_bool, a_int)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let many_types = (1u8, 2u16, 'a', true);
    println!("first value: {}", many_types.0);
    println!("second value: {}", many_types.1);

    let tuples_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuples_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reverse pais is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,)); // note extra comma
    println!("just an integer: {}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.2, 2.1, 2.3, 4.1);
    println!("{:?}", matrix)
}
