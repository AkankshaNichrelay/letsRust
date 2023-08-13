fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = &x;

    println!("Value y={}", y);
    println!("Value y={:?}", y);
    println!("Address of y={:p}", y);
    println!("Deref of y={}", *y);
    // println!("Address of x={:p}", x);
}
