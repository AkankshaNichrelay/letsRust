# Concurrency

## Threads

```
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    Builder::new().spawn(f).expect("failed to spawn thread")
}
```

Spawn takes an argument called f which is of generic type F that has a few trait bounds. The first trait bound is `FnOnce()` which is a closure trait bound specifying that this is a closure that takes ownership of the values in its environment. We also have the `send` trait bound so that we can transfer the closure from one thread to another. And then we have static lifetime which menas that the receiver can hold on to this type as long as they need to and the type will be valid until the receiver drops it.

## Mutex

Mutexes use interior mutability. Mutexes comes with the risk of creating deadlocks.

- Send and Sync traits

## Arc
Atomic RCs, thread safe.