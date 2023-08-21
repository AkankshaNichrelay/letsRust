// https://dzone.com/articles/the-go-developers-quickstart-guide-to-rust

// use use to bring something into scope
use std::time::{SystemTime, Duration};

/*  to use an external library:
1. tell cargo that we need it to get the library for us.
2. tell the compiler/linker that we are using this external library (in main.rs ).
3. then 'use' the library.
 */

/*
rust uses a standard package management system in which packages (crates) can be
referenced by name and automatically tracked by version. In the rust world,
the idiomatic place to find packages is crates.io 
 */

extern crate chrono;
use chrono::prelude::*;

fn main() {
    let mut now = SystemTime::now();
    now = now + Duration::from_secs(2);
    println!("it is now {:?}", now);
    println!("{} {:?} {2} {last}", "first", "second", "third", last="fourth");

    let local_now = Local::now();
    println!("it is local now {:?}", local_now);

    let (is_pm, _) = local_now.hour12();
    if is_pm {
        println!("good evening, world!");
    } else {
        println!("good morning, world!");
    }

    // like the tertiary operator in c/c++
    let am_pm = if is_pm {
        "evening"
    } else {
        "morning"
    };
    println!("tert: good {}, world!", am_pm);

    // match is like the case statement. A match must cover all possible cases.
    let am_or_pm = match local_now.hour12() {
        (false, _) => "morning",
        (_, _) => "evening",
    };
    println!("match: good {}, world!", am_or_pm);

    let am_pm_idx = if local_now.hour12().0  { "evening" } else { "morning" };
    println!("tupleIdx: good {}, world!", am_pm_idx);
}

/*
Traits are like supercharged go interfaces. Implementing a trait in rust produces
approximately the same effect as implementing an interface in Go. In rust, though,
implementing a trait must be done explicitly ( impl mytrait for mytype {} ).
 */