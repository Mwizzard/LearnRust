fn main() {
    variables_and_mutability();
    
    data_types();
   
    let index : i32 = 21;

    let value : i32 = fibonacci(index);

    println!("the value at the position {index} in the fibonacci sequence is : {value}");

}

fn variables_and_mutability(){
    println!("Chapter 3.1 :");

    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //declaring a constant

    let x: i32 = 5;

    let x = x + 2; //shadows x for x + 1

    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x} ");
    }
    
    println!("The value of x in the outer scope is: {x}");

}

fn data_types(){
    println!("Chapter 3.2");


    //basic types
    let _sum : u32 = 5 + 10;

    let _difference : f32 = 99.5 - 4.4;

    let _product : u32 = 4 * 30;

    let _quotient : f64 = 56.6 / 32.2;
    let _truncated = -5/3; // -1

    let _remainder = 43 % 5;

    let _c = 'z';
    let _emoji = '😻';

    let _t : bool = true;
    
    //Compound types
    let tuple : (i32, f64, u8) = (400, 6.4, 1);

    let (x,y,z) = tuple;
    println!("The second value of the tuple is : {y}");


    //accessing a specific element of a tuple

    let four_hundred = tuple.0;

    let six_point_four = tuple.1;

    println!("four_hunderd {four_hundred}, six_point_four {six_point_four}");

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}

//defined as (0, 1, 1, 2, 3, 5, etc) where 0 is at the index 1.
fn fibonacci(mut n : i32) -> i32{
    if n == 0 {
        return 0;
    } else if n <= 2 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n -2);
    }
}
