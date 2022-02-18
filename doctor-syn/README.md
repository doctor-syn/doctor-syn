# doctor-syn
[![Build Status](https://github.com/extendr/doctor-syn/workflows/CI/badge.svg)](https://github.com/extendr/doctor-syn/actions)

## rationale

A computer algebra system for rust.

This crate is mostly for generating mathematical code at compile time.
The focus is largely on numerical approximation of transcendental and
statistical functions to make them vectorisable.

For example, say we want to make an approximation to `-cos(x*2π)` over the
domain `-0.5..0.5`, we can use the approx function to transform an expression.

```
fn gen_cos() -> TokenStream {
    let xmin = -0.5;
    let xmax = 0.5;

    let approx = expr!((x * PI * 2.0).cos() * -1.0)
        .approx(13, xmin, xmax, name!(x), Parity::Even)
        .unwrap()
        .use_number_type(Some("f32".to_string()))
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

## libmgen

You can install the `libmgen` binary with:

```
cargo install --path . --bin libmgen
```

This can then be run as `libmgen` to generate
a libm for various precisions, languages and data types.

## Roadmap

Currently, Doctor Syn exists purely for libmgen but it would be nice to build
syntax transformation passes and mathematical transformations as independent
units.

* Drop `Expression` and work solely on naked syn types.
* Use syn visitors where possible.

Planned new crates:

* ds_makeimpl -> convert items to a trait.
* ds_prettyprint -> pretty print syn trees (like rustfmt).
* ds_subst -> match and substitute expressions and syntax elements.
* ds_tosimd -> convert scalar to vector functions.
* ds_differentiate -> differentiate rust expressions.
* ds_typeinfetence -> annotate items and expressions with types.
* ds_eval -> evaluate expressions with various number types.
* ds_toc -> convert simple rust to C.
* ds_toasm -> convert simple rust to asm.
* ds_tostatemachine -> convert simple rust to a state machine.
* ds_toverilog -> convert simple rust to verilog.
* ds_exec -> execute simple rust code by converting to machine code.
* ds_approx -> approximate functions.
