fn main() {
    let mut x = 5;
    { //Creating a new scope
        let y = &mut x; 
        *y = 6;
        println!("x in first scope = {}", x);
    } 
    { //Creating a new scope
        let z = &mut x;
        *z = 7;
        println!("x in second scope = {}", x);
    }
    println!("x = {}", x);
}