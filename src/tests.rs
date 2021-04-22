
use crate::expr;

#[test]
fn eval() {
    let func = expr!(x.sin());
    println!("{}", func);

    // use std::f64::consts::PI;
    // let a = (xmax + xmin) * 0.5;
    // let b = PI / (terms - 1) as f64;
    // let c = (xmax - xmin) * 0.5;
    // let mut xvalues = Vec::new();
    // let mut yvalues = Vec::new();
    // let mut interp = Interpreter::from_variables(args);
    // let xpath = parse_str("x")?;
    // for i in 0..terms {
    //     // *almost* Chebyshev nodes.
    //     let x = a - c * (i as f64 * b).cos();
    //     interp.set_var(&xpath, from_any::<f64>(x)?)?;
    //     let y = as_number(&interp.expr(&body).unwrap())?;
    //     println!("{:?} {:?}", x, y);
    //     xvalues.push(x);
    //     yvalues.push(y);
    // }
}