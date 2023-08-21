# Generics, Traits and Lifetimes

Generics, traits and lifetimes are all ways to reduce code duplication.

Generics allow us to abstract concrete type to reduce code duplication. Your program doesn't take a performance hit as compiler actually does the duplication for you at compile time.

Traits allow us to define a shared behavior (a set of methods that are shared) across different types.

Lifetimes or Generic Lifetime Annotation describe the relationship between the lifetimes of multiple references and how they relate to each other. These annotations don't really change the lifetimes of these references but rather explain the relationship between different lifetimes. It helps compiler to detect dangling references.


( Say we have a function taking references to two string and conditionally returning a reference to one of the string. Borrower checker doesn't know for sure the lifetime of the returned reference as the lifetime of x and y could vary.)

```
fn Longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

```

This tells the complier that the lifetime of the returned value will be the same as the smallest lifetime of the arguments.

* The lifetime of the returned reference has to be tied to the lifetime of one of the passed in reference. That is because if we are returning a reference, it has to be a reference to one of the parameters passed into the function. We cannot return reference to a local variable as the local variables get destroyed once the function has finished executing. We can also return an own type, for example we can return a String created in the function, that is because we will be transferring ownership to the calling function.

## Structs and Lifetimes
 We need to define lifetime for a struct if we are using references in the struct.

 ```
struct ReferencingStruct<'a> {
    my_ref: &'a str,
}
 ```

The lifetime annotation is saying that our struct cannot outlive the reference passed into my_ref. So if my_ref refers to the string `Hello` in main function, then our struct instance will not outlive the `Hello` variable.


## Lifetime Elision Rules

Lifetimes on function parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.
The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations.

1. Each parameter that is a reference gets its own lifetime parameter. 
The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32);` a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.

2. If there is only one input parameter, then the lifetime is assigned to all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of self is assigned to all output parameters.

 If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to fn definitions as well as impl blocks.

## Static Lifetime
Static lifetime means that the reference could live as long as the duration of the program. All string literals have a static lifetime, that is because string literals are stored in the program binary.