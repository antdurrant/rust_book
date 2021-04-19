fn main() {
    println!("Hello, world!");

    another_function(5);
    some_maffs(50, 11);
}

fn another_function(x: i32){
    println!("Heyooooooo! {}", x)
}

fn some_maffs(x: i32, y: i32){
    let result = x/y;
    println!("x is {}; y is {}, x/y  is {}", x, y, result)
}
