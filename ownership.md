# Ownership in Rust

## Rules

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time. (you can move ownership or copy values )
3. When the owner goes out of scope, the value will be dropped.

### Copy, Move and Clone

* primitive types with fixed-size implement the copy trait to make copies.
* "move" means the ownership of the memory is transferred to another owner. String is heap allocated so it is moved. 
* If we do want to deeply copy the heap data of the String, not just the stack data, we can use a method called clone.

## How Ownership Moves
There are three ways to transfer ownership from one variable to another in a Rust program:

1. Assigning the value of one variable to another variable.
2. Passing value to a function.
3. Returning from a function.

# Borrowing in Rust

Borrowing, in its literal sense, refers to receiving something with the promise of returning it. In the context of Rust, borrowing is a way of accessing value without claiming ownership of it, as it must be returned to its owner at some point.
Borrowing is done by using reference to a variable if we dont want to move the variable.

## Rules

1. The scope of the borrower cannot outlast the scope of the original owner.
2. There can be multiple immutable references, but only one mutable reference.
3. At any given time, you can have either one mutable reference or any number of immutable refrences. (We can have a mutable reference once the use of mutable reference is finished.)
4. References must always be valid (can’t be null). (Cannot return a reference for a value declared in the scope of the returning function. Compiler error)

References Are Immutable by Default. It can be made mutable with mut, but only if its owner is also mutable.

### Slices
Slices let you reference a contiguous sequence of elements within a collection instead of referencing the entire collection. Slices dont take ownership of the underlying data.