# Smart Pointers

Smart pointers are data structures that act like a pointer but have metadata and extra capabilities tacked on. One example is a reference counting smart pointer which allows a piece of data to have multiple owners by keeping track of the owners and once there are no more owners cleaning up the data.

In many cases, smart pointers own the data that they point to unlike references which just simply borrows the values. Strings and Vectors are smart pointers.

## Deref and Drop Traits 

Unlike regular structs, smart pointers implement the deref and drop traits. The deref trait allows the instances of your smart pointer struct to be treated like references, so you can write code that uses references or smart pointers. The drop trait allows you to customize the code that is run when an instance of your smart pointer goes out of scope.

## Box
Box is a smart pointer within standard Rust library that allows you to allocate memory on heap. 

#### Use case for box
* When you have a type whose size cannot be known at the compile time and you want to use a value of that type in a context that requires knowing its exact size.
* When you have a large amount of data and you want to transfer ownership of that data but you want to make sure that the data isn't copied.
* When you own a value and you only care that the value implements a specific trait rather than it being a specific type. This is known as a **trait object**.

## Implicit Deref Coercion

Implicit deref coercion with functions and methods is a convinence feature that happens in Rust automatically for the types that implement the deref trait. Deref coercion will convert a reference of one type to a reference of another different type. 

## Reference Counting Pointer (Rc)
Used to keep track of owners of a variable. The last owner to exit will drop the variable. Think Graph with mutliple edges pointing to one node, all are owners to the node.

## Interior Mutability
Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that dat which is typically disallowed by the borrowing rules. To mutate data, this pattern uses unsafe code inside a data structure to bypass the typical rules around mutation and borrowing.

Unsafe code is code that is not checked at compile time for memory safety. Even though borrowing rules are not enforced at compile time, we can still enforce them at runtime.
Why would you want to enforce memory safety at runtime instead of compile time?

## RefCell
The RefCell smart pointer represents single ownership over the data it holds, much like a box smart pointer. The difference is that the box smart pointer enforces the borrowing rules at compile time whereas the refCell smart pointer enforces borrowing rules at runtime. This means that if you break borrowing rules at runtime, your program will panic and exit. 

The advantages of checking borrowing rules at compile time (default) is that errors will be caught sooner in the development cycle and there's no runtime performance cost. The advantage of checking borrowing rules at runtime is that certain memory safe scenarios are allowed whereas they would be disallowed at compile time. This is because certain properties of a program are impossible to detect using static analysis.

The most famous example of this is the halting problem. A classic example of an unsolvable algorithmic problem is the halting problem, which states that no program can be written that can predict whether or not any other program halts after a finite number of steps. The Halting Problem tells that it is not easy to write a computer program that executes in the limited time that is capable of deciding whether a program halts for an input. In addition to that the Halting Problem never says that it is not practicable to determine whether a given random program is going to halt (stop).