fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = &x;

    println!("Value y={}", y);
    println!("Value y={:?}", y);
    println!("Address of y={:p}", y);
    println!("Deref of y={}", *y);
    // println!("Address of x={:p}", x);

    borrow();
}

fn borrow() {
    let v = vec![10,20,30];
    print_vector(&v);
    println!("{:#?}", v); // can access v here as references can't move the value
 }
 
 fn print_vector(x: &Vec<i32>) {
    println!("Inside print_vector function {:?}", x);
 }