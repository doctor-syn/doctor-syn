#![allow(unused_variables)]

//! This module translates Rust expressions and items into C
//!
//!

use crate::error::{Error, Result};

use std::cell::RefCell;

use quote::ToTokens;
// use crate::Expression;
use syn::{
    BinOp, Expr,
    Item, Local, Pat, Path, Type, UnOp,
};

use syn::Lit;
use proc_macro2::{Ident, Literal};
use syn::{FnArg, ReturnType, Signature, Stmt};
use syn::{
    ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemForeignMod, ItemImpl, ItemMacro, ItemMacro2,
    ItemMod, ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, ItemUse,
};
use syn::{
    TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeInfer, TypeMacro, TypeNever, TypeParen,
    TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple,
};
use syn::{ExprArray, ExprAssign, ExprAssignOp, ExprAsync, ExprAwait, ExprBinary, ExprBlock, ExprBox, ExprBreak, ExprCall, ExprCast, ExprClosure, ExprContinue, ExprField, ExprForLoop, ExprGroup, ExprIf, ExprIndex, ExprLet, ExprLit, ExprLoop, ExprMacro, ExprMatch, ExprMethodCall, ExprParen, ExprPath, ExprRange, ExprReference, ExprRepeat, ExprReturn, ExprStruct, ExprTry, ExprTryBlock, ExprTuple, ExprType, ExprUnary, ExprUnsafe, ExprWhile, ExprYield};

use std::collections::HashMap;

pub struct State {
    depth: usize,
    var_aliases: HashMap<String, String>,
    type_aliases: HashMap<String, String>,
    default_type: String,
}

impl State {
    fn new() -> Self {
        Self { depth: 0, var_aliases: HashMap::new(), type_aliases: HashMap::new(), default_type: "f32".into() }
    }

    // Called on every let statement to alias a variable.
    pub fn set_var_alias(&mut self, rust_name: String, c_name: &str) -> String {
        if !self.var_aliases.contains_key(rust_name.as_str()) {
            self.var_aliases.insert(rust_name.clone(), c_name.into());
            rust_name
        } else {
            // Add a trailing underscore if not unique.
            let entry = self.var_aliases.get_mut(rust_name.as_str()).unwrap();
            *entry += "_";
            entry.clone()
        }
    }

    pub fn set_type_alias(&mut self, rust_name: String, c_name: &str) -> String {
        if !self.type_aliases.contains_key(rust_name.as_str()) {
            self.type_aliases.insert(rust_name.clone(), c_name.into());
            rust_name
        } else {
            // Add a trailing underscore if not unique.
            let entry = self.type_aliases.get_mut(rust_name.as_str()).unwrap();
            *entry += "_";
            entry.clone()
        }
    }

    // Given a Rust variable, what is its C name.
    pub fn var_alias(&self, rust_name: &str) -> Result<String> {
        if !self.var_aliases.contains_key(rust_name) {
            Ok(rust_name.into())
        } else {
            self.var_aliases.get(rust_name).ok_or(Error::UndefinedVariable(rust_name.into())).map(|s| s.clone())
        }
    }

    // Given a Rust type, what is its C name.
    pub fn type_alias(&self, rust_name: &str) -> Result<String> {
        if !self.type_aliases.contains_key(rust_name) {
            Ok(rust_name.into())
        } else {
            self.type_aliases.get(rust_name).ok_or(Error::UndefinedVariable(rust_name.into())).map(|s| s.clone())
        }
    }
}

pub struct Context(RefCell<State>);

impl Context {
    pub fn new() -> Self {
        Self(RefCell::new(State::new()))
    }

    pub fn begin(&self) -> ContextGuard {
        self.0.borrow_mut().depth += 1;
        ContextGuard { context: self }
    }

    pub fn end(&self) {
        self.0.borrow_mut().depth += 1;
    }

    pub fn ind(&self) -> &str {
        let tabs = "                                                                                                                        ";
        &tabs[0..self.0.borrow().depth*2]
    }

    pub fn default_type(&self) -> String {
        self.0.borrow().default_type.clone()
    }

    pub fn clear_vars(&self) {
        self.0.borrow_mut().var_aliases = HashMap::new();
        self.0.borrow_mut().type_aliases = HashMap::new();
    }

    pub fn set_var_alias(&self, rust_name: String, c_name: &str) -> String {
        self.0.borrow_mut().set_var_alias(rust_name, c_name)
    }

    pub fn var_alias(&self, rust_name: &str) -> Result<String> {
        self.0.borrow().var_alias(rust_name)
    }

    pub fn set_type_alias(&self, rust_name: String, c_name: &str) -> String {
        self.0.borrow_mut().set_type_alias(rust_name, c_name)
    }

    pub fn type_alias(&self, rust_name: &str) -> Result<String> {
        self.0.borrow().type_alias(rust_name)
    }

}

pub struct ContextGuard<'a> {
    context: &'a Context,
}

impl<'a> std::ops::Drop for ContextGuard<'a> {
    fn drop(&mut self) {
        self.context.end();
    }
}

fn make_err<T: ToTokens>(value: T) -> Result<String> {
    // #[cfg(debug_codegen)]
    use std::io::Write;
    std::io::stdout().flush().unwrap();
    panic!("error {}", value.to_token_stream().to_string());
    #[cfg(not(debug_codegen))]
    Err(Error::UnsupportedCodegen(
        value.to_token_stream().to_string(),
    ))
}

fn log<T: ToTokens + std::fmt::Debug>(value: T) {
    // #[cfg(debug_codegen)]
    println!("log {}", value.to_token_stream().to_string());
    //println!("log {:?}", value);
}

pub trait AsC {
    fn as_c(&self, context: &Context) -> Result<String>;
}

impl AsC for Local {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        let pat = self.pat.as_c(context)?;
        let alias = context.set_var_alias(pat.clone(), pat.as_str());
        if let Some((_, init)) = &self.init {
            Ok(format!("{} = {}", pat, init.as_c(context)?))
        } else {
            Ok(pat)
        }
    }
}

impl AsC for Item {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        use Item::*;
        match self {
            Const(value) => value.as_c(context),
            Enum(value) => value.as_c(context),
            ExternCrate(value) => value.as_c(context),
            Fn(value) => value.as_c(context),
            ForeignMod(value) => value.as_c(context),
            Impl(value) => value.as_c(context),
            Macro(value) => value.as_c(context),
            Macro2(value) => value.as_c(context),
            Mod(value) => value.as_c(context),
            Static(value) => value.as_c(context),
            Struct(value) => value.as_c(context),
            Trait(value) => value.as_c(context),
            TraitAlias(value) => value.as_c(context),
            Type(value) => value.as_c(context),
            Union(value) => value.as_c(context),
            Use(value) => value.as_c(context),
            Verbatim(_value) => make_err(self),
            #[cfg(test)]
            Item::__TestExhaustive(_) => unimplemented!(),
            #[cfg(not(test))]
            _ => make_err(self),
        }
    }
}

impl AsC for Expr {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        use Expr::*;
        match self {
            Array(value) => value.as_c(context),
            Assign(value) => value.as_c(context),
            AssignOp(value) => value.as_c(context),
            Async(value) => value.as_c(context),
            Await(value) => value.as_c(context),
            Binary(value) => value.as_c(context),
            Block(value) => value.as_c(context),
            Box(value) => value.as_c(context),
            Break(value) => value.as_c(context),
            Call(value) => value.as_c(context),
            Cast(value) => value.as_c(context),
            Closure(value) => value.as_c(context),
            Continue(value) => value.as_c(context),
            Field(value) => value.as_c(context),
            ForLoop(value) => value.as_c(context),
            Group(value) => value.as_c(context),
            If(value) => value.as_c(context),
            Index(value) => value.as_c(context),
            Let(value) => value.as_c(context),
            Lit(value) => value.as_c(context),
            Loop(value) => value.as_c(context),
            Macro(value) => value.as_c(context),
            Match(value) => value.as_c(context),
            MethodCall(value) => value.as_c(context),
            Paren(value) => value.as_c(context),
            Path(value) => value.as_c(context),
            Range(value) => value.as_c(context),
            Reference(value) => value.as_c(context),
            Repeat(value) => value.as_c(context),
            Return(value) => value.as_c(context),
            Struct(value) => value.as_c(context),
            Try(value) => value.as_c(context),
            TryBlock(value) => value.as_c(context),
            Tuple(value) => value.as_c(context),
            Type(value) => value.as_c(context),
            Unary(value) => value.as_c(context),
            Unsafe(value) => value.as_c(context),
            Verbatim(value) => make_err(self),
            While(value) => value.as_c(context),
            Yield(value) => value.as_c(context),
            #[cfg(test)]
            __TestExhaustive(_) => unimplemented!(),
            #[cfg(not(test))]
            _ => make_err(self),
        }
    }
}

impl AsC for ItemConst {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        Ok(format!("const {} {} = {};", self.ty.as_c(context)?, self.ident.as_c(context)?, self.expr.as_c(context)?))
    }
}

impl AsC for ItemEnum {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ItemExternCrate {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ItemFn {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);

        context.clear_vars();

        let inputs = self
        .sig
        .inputs
        .iter()
        .map(|arg| arg.as_c(context))
        .collect::<Result<Vec<_>>>()?
        .join(", ");

        let output = self.sig.output.as_c(context)?;
        let mut res = format!("{}{} {}_{}({}) {{\n", context.ind(), output, context.default_type(), self.sig.ident.to_string(), inputs);

        {
            let g = context.begin();
            let ident = self.sig.ident.as_c(context)?;
    
            for stmt in self.block.stmts.iter() {
                if let Stmt::Expr(expr) = stmt {
                    let s = format!("{}return {};\n", context.ind(), expr.as_c(context)?);
                    res.extend(s.chars());
                } else {
                    let s = format!("{}{};\n", context.ind(), stmt.as_c(context)?);
                    res.extend(s.chars());
                }
            }
        }
        res.extend(format!("}}\n\n").chars());
        Ok(res)
    }
}

impl AsC for ItemForeignMod {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ItemImpl {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ItemMacro {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ItemMacro2 {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ItemMod {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ItemStatic {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ItemStruct {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ItemTrait {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ItemTraitAlias {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ItemType {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ItemUnion {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ItemUse {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        Ok(String::new())
    }
}

impl AsC for ExprArray {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprAssign {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprAssignOp {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprAsync {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprAwait {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprBinary {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        let left = self.left.as_c(context)?;
        let right = self.right.as_c(context)?;
        use BinOp::*;
        match self.op {
            Add(_) => Ok(format!("{} + {}", left, right)),
            Sub(_) => Ok(format!("{} - {}", left, right)),
            Mul(_) => Ok(format!("{} * {}", left, right)),
            Div(_) => Ok(format!("{} / {}", left, right)),
            Rem(_) => Ok(format!("{} % {}", left, right)),
            And(_) => Ok(format!("{} && {}", left, right)),
            Or(_) => Ok(format!("{} || {}", left, right)),
            BitXor(_) => Ok(format!("{} ^ {}", left, right)),
            BitAnd(_) => Ok(format!("{} & {}", left, right)),
            BitOr(_) => Ok(format!("{} | {}", left, right)),
            Shl(_) => Ok(format!("{} << {}", left, right)),
            Shr(_) => Ok(format!("{} >> {}", left, right)),
            Eq(_) => Ok(format!("{} == {}", left, right)),
            Lt(_) => Ok(format!("{} < {}", left, right)),
            Le(_) => Ok(format!("{} <= {}", left, right)),
            Ne(_) => Ok(format!("{} != {}", left, right)),
            Ge(_) => Ok(format!("{} >= {}", left, right)),
            Gt(_) => Ok(format!("{} > {}", left, right)),
            AddEq(_) => Ok(format!("{} += {}", left, right)),
            SubEq(_) => Ok(format!("{} -= {}", left, right)),
            MulEq(_) => Ok(format!("{} *= {}", left, right)),
            DivEq(_) => Ok(format!("{} /= {}", left, right)),
            RemEq(_) => Ok(format!("{} %= {}", left, right)),
            BitXorEq(_) => Ok(format!("{} ^= {}", left, right)),
            BitAndEq(_) => Ok(format!("{} &= {}", left, right)),
            BitOrEq(_) => Ok(format!("{} |= {}", left, right)),
            ShlEq(_) => Ok(format!("{} <<= {}", left, right)),
            ShrEq(_) => Ok(format!("{} >>= {}", left, right)),
        }
    }
}

impl AsC for ExprBlock {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprBox {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprBreak {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprCall {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        let func = self.func.as_c(context)?;
        let func = match &*func {
            "f32 :: from_bits" => "f32_from_bits".into(),
            "f64 :: from_bits" => "f64_from_bits".into(),
            f => format!("{}_{}", context.default_type(), f),
        };
        let args = self
            .args
            .iter()
            .map(|a| a.as_c(context))
            .collect::<Result<Vec<_>>>()?
            .join(", ");
        Ok(format!("{}({})", func, args))
    }
}
impl AsC for ExprCast {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        Ok(format!("({}){}", self.ty.as_c(context)?, self.expr.as_c(context)?))
    }
}
impl AsC for ExprClosure {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprContinue {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprField {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprForLoop {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprGroup {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprIf {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);

        // pub attrs: Vec<Attribute>,
        // pub if_token: Token![if],
        // pub cond: Box<Expr>,
        // pub then_branch: Block,
        // pub else_branch: Option<(Token![else], Box<Expr>)>,

        // format!(""self.
        make_err(self)
    }
}
impl AsC for ExprIndex {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprLet {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprLit {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        use Lit::*;
        match &self.lit {
            Str(value) => Ok(format!("{}", value.to_token_stream())),
            ByteStr(value) => Ok(format!("{}", value.to_token_stream())),
            Byte(value) => Ok(format!("{}", value.to_token_stream())),
            Char(value) => Ok(format!("{}", value.to_token_stream())),
            Int(value) => Ok(value.base10_digits().into()),
            Float(value) => Ok(value.base10_digits().into()),
            Bool(value) => Ok(format!("{}", value.to_token_stream())),
            Verbatim(value) => Ok(format!("{}", value.to_token_stream()))
        }
    }
}

impl AsC for ExprLoop {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprMacro {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprMatch {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprMethodCall {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        let receiver = self.receiver.as_c(context)?;
        let args = self
            .args
            .iter()
            .map(|a| a.as_c(context))
            .collect::<Result<Vec<_>>>()?
            .join(", ");
        let method = format!("{}_{}", context.default_type(), self.method.as_c(context)?);
        if args.is_empty() {
            Ok(format!("{}({})", method, receiver))
        } else {
            Ok(format!("{}({}, {})", method, receiver, args))
        }
    }
}

impl AsC for ExprParen {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        let e = self.expr.as_c(context)?;
        Ok(format!("({})", e))
    }
}

impl AsC for ExprPath {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        if let Some(qself) = &self.qself {
            return make_err(self);
        }
        let path = self.to_token_stream().to_string();
        let c = match &*path {
            "std :: f32 :: consts :: PI" => "M_PI",
            c => c,
        };
        // TODO: mangle
        Ok(c.to_string())
    }
}
impl AsC for ExprRange {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprReference {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprRepeat {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprReturn {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprStruct {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprTry {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprTryBlock {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for ExprTuple {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        let elems = self
            .elems
            .iter()
            .enumerate()
            .map(|(t, e)| -> Result<String> {Ok(format!("t{}: {}", t, e.as_c(context)?))})
            .collect::<Result<Vec<_>>>()?
            .join(", ");
        Ok(format!("(struct Tuple){{{}}}", elems))
    }
}
impl AsC for ExprType {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ExprUnary {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        let e = self.expr.as_c(context)?;
        match self.op {
            UnOp::Deref(_) => Ok(format!("*{}", e)),
            UnOp::Not(_) => Ok(format!("!{}", e)),
            UnOp::Neg(_) => Ok(format!("-{}", e)),
        }
    }
}

impl AsC for ExprUnsafe {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ExprWhile {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ExprYield {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}


impl AsC for Signature {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}

impl AsC for ReturnType {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        use ReturnType::*;
        match self {
            Default => Ok("()".into()),
            Type(_, t) => t.as_c(context),
        }
    }
}

impl AsC for Type {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        use Type::*;
        match self {
            Array(value) => value.as_c(context),
            BareFn(value) => value.as_c(context),
            Group(value) => value.as_c(context),
            ImplTrait(value) => value.as_c(context),
            Infer(value) => value.as_c(context),
            Macro(value) => value.as_c(context),
            Never(value) => value.as_c(context),
            Paren(value) => value.as_c(context),
            Path(value) => value.as_c(context),
            Ptr(value) => value.as_c(context),
            Reference(value) => value.as_c(context),
            Slice(value) => value.as_c(context),
            TraitObject(value) => value.as_c(context),
            Tuple(value) => value.as_c(context),
            Verbatim(value) => make_err(self),
            #[cfg(test)]
            __TestExhaustive(_) => unimplemented!(),
            #[cfg(not(test))]
            _ => make_err(self),
        }
    }
}

impl AsC for FnArg {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        use FnArg::*;
        match self {
            Receiver(value) => Ok("self".to_string()),
            Typed(value) => Ok(format!(
                "{} {}",
                value.ty.as_c(context)?,
                value.pat.as_c(context)?
            )),
        }
    }
}

impl AsC for Stmt {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        use Stmt::*;
        match self {
            Local(value) => value.as_c(context),
            Item(value) => value.as_c(context),
            Expr(value) => value.as_c(context),
            Semi(value, _) => value.as_c(context),
        }
    }
}

impl AsC for Pat {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        use Pat::*;
        match self {
            Box(value) => make_err(self),
            Ident(ident) => ident.ident.as_c(context),
            Lit(value) => make_err(self),
            Macro(value) => make_err(self),
            Or(value) => make_err(self),
            Path(path) => path.path.as_c(context),
            Range(value) => make_err(self),
            Reference(value) => make_err(self),
            Rest(value) => make_err(self),
            Slice(value) => make_err(self),
            Struct(value) => make_err(self),
            Tuple(value) => make_err(self),
            TupleStruct(value) => make_err(self),
            Type(value) => Ok(format!("{} {}", value.ty.as_c(context)?, value.pat.as_c(context)?)),
            Verbatim(value) => make_err(self),
            Wild(value) => make_err(self),
            _ => return make_err(self),
        }
    }
}

impl AsC for Path {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        let s = self.to_token_stream().to_string();
        context.var_alias(s.as_str())
    }
}

impl AsC for Ident {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        let s = self.to_string();
        //context.var_alias(s.as_str())
        Ok(s)
    }
}

impl AsC for Literal {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        // TODO:
        Ok(self.to_string())
    }
}

impl AsC for TypeArray {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for TypeBareFn {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for TypeGroup {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for TypeImplTrait {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for TypeInfer {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for TypeMacro {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for TypeNever {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for TypeParen {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for TypePath {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        if let Some(qself) = &self.qself {
            return make_err(self);
        }
        Ok(self.path.as_c(context)?)
    }
}
impl AsC for TypePtr {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for TypeReference {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for TypeSlice {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for TypeTraitObject {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        make_err(self)
    }
}
impl AsC for TypeTuple {
    fn as_c(&self, context: &Context) -> Result<String> {
        log(self);
        let elems = self
            .elems
            .iter()
            .enumerate()
            .map(|(t, e)| -> Result<String> {Ok(format!("{} t{}", e.as_c(context)?, t))})
            .collect::<Result<Vec<_>>>()?
            .join("; ");
        Ok(format!("struct Tuple{{ {} }}", elems))
    }
}


