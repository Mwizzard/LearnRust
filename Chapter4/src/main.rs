fn main() {
    //Ownership
    let s = String::from("Hello!");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);


}

fn takes_ownership(string : String) {
    println!("{string}");
}

fn makes_copy(integer : i32) {
    println!("{integer}");
}
