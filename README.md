# doctor-syn

## rationale

A set of mathematical syntax transformation tools.

This crate is mostly for generating mathematical code at compile time.
The focus is largely on numerical apprimation of transcendental and
statistical functions to make them vectorisable.

For example, say we want to make an approximation to `sin(x)` over the
domain `0..2π`, we can use the approx! macro to transform a lambda:

```
fn my_sin(x: f64) -> f64 {
    approx!(x=0..2*f64::PI, terms=6, |x| x.sin())
}
```

## origin of the project name

Doctor Syn is the eponymous hero of the Russell Thorndike's novel
and also a play on the **syn** rust library used to implement it.

https://en.wikipedia.org/wiki/Doctor_Syn

Doctor Syn was both a priest and a smuggler of the parish of Dymchurch
on the Romney marshes.
