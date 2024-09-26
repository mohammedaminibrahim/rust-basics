fn main() {
    let x = 4;
    println!("The value of x is {}", x);

    {
        let y = 7;
        println!("The value of y is {}", y);
    }

    let x = 5;
    println!("The value of x is {}", x);
}
