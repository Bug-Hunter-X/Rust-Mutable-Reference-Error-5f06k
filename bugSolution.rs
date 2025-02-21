fn main() {
    let mut x = 5;
    { // create a scope for the reference y. 
        let y = &mut x;
        *y = 10;
    }
    { // create a scope for the reference z.
        let z = &mut x;
        *z = 15;
    }
    println!("x = {}", x);
}