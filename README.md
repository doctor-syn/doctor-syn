# doctor-syn

## rationale

A computer algebra system for rust.

This crate is mostly for generating mathematical code at compile time.
The focus is largely on numerical apprimation of transcendental and
statistical functions to make them vectorisable.

For example, say we want to make an approximation to `-cos(x*2π)` over the
domain `-0.5..0.5`, we can use the approx function to transform an expression.

```
fn gen_cos() -> proc_macro2::TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!((x * 3.1415926535897932384626433 * 2.0).cos() * -1.0)
        .approx(13, xmin, xmax, name!(x), Parity::Even)
        .unwrap()
        .use_suffix(Some("f32".to_string()))
        .unwrap()
        .into_inner();

    quote!(
        fn cos(x: f32) -> f32 {
            let x = x * (1.0 / (std::f32::consts::PI * 2.0));
            let x = x - x.floor() - 0.5;
            #approx
        }
    )
}
```

## origin of the project name

Doctor Syn is the eponymous hero of the Russell Thorndike's novel
and also a play on the **syn** rust library used to implement it.

https://en.wikipedia.org/wiki/Doctor_Syn

Doctor Syn was both a priest and a smuggler of the parish of Dymchurch
on the Romney marshes.
