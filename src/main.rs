fn main() {
    // DATA TYPE
    println!("Hello world!");

    // VARIABLES
    let var: i32 = 0;
    let var2: i64 = 10;

    println!("{}", var);
    println!("{}", var2);

    // TUPLES
    let tuple: (i32, i64, u32, u64) = (-9, 64, 0, 11);
    println!("{}", tuple.2);

    // ARRAYS
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    // STRINGS
    let string: &str = "Hello, world!";
    println!("{}", string);

    // BOOLEANS
    let bool_var: bool = true;
    println!("{}", bool_var);

    // CHARACTERS
    let char_var: char = 'a';
    println!("{}", char_var);

    // CONSTANTS
    const CONSTANT: i32 = 100;
    println!("{}", CONSTANT);
}
