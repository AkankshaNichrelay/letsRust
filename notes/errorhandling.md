# Error Handling

Chaining results using `match` can get pretty untidy; luckily, the `?` operator can be used to make things pretty again. `?` is used at the end of an expression returning a `Result`, and is equivalent to a match expression, where the Err(err) branch expands to an early `return Err(From::from(err))`, and the `Ok(ok)` branch expands to an `ok` expression.