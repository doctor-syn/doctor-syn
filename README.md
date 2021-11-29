# Doctor Syn and Libmgen

Sponsored by Atomic Increment Ltd and Embecosm Ltd.

This project is focused on generating the highest
performing mathematical functions for Rust, C, C++ and Fortran.

The libmgen command line tool and associated library doctor_syn
generate polynomial approximations of mathematical functions
which will parallelise and vectorise.

The goal is to replace mathematical functions in R, Numpy and GNU octave
with much faster versions.

[See](https://atomicincrement.github.io/maths/2021/11/18/polynomial-approximation.html)

To run libmgen, first install [Rust using rustup](https://www.rust-lang.org/tools/install)
and then use

```
cargo install libmgen
```

to install the command line tool locally. Cargo will download and build
the library for your platform.

```
libmgen --help
```

Gives the options.

```
libmgen --functions sin -o sin.rs
```

Generates `sin(x)` in Rust for 64 bit float.

```Rust
#[allow (non_camel_case_types)]
type fty =f64 ;

const RECIP_2PI :fty =0.1591549430918953357688837633725143620345;

pub fn sin (arg : fty)->fty {
  let scaled :fty =arg *RECIP_2PI ;
  let x :fty =scaled -scaled .round ();
  (- 0.0000795978135564681619446994463825844449 as f64).mul_add (x * x , 0. ...
}

```

## Milestones

[ ] Rust codegen complete for all IEEE functions.
[ ] C/C++ codegen complete.
[ ] Fortran codegen.

