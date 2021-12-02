#![allow(unused_variables)]
// use syn::visit::Visit;
// use syn::visit;
// use syn::ItemFn;
use syn::*;
use syn::visit::*;
use syn::punctuated::Punctuated;
use proc_macro2::Span;

struct CVisitor {
    stack: Vec<String>,
}

impl CVisitor {
    fn new() -> Self {
        CVisitor {
            stack: Vec::new(),
        }
    }

    fn log(&mut self, msg: &str) {
        println!("log: {} {:?}", msg, self.stack);
    }

    fn tos(&mut self) -> String {
        self.stack.pop().unwrap()
    }
}

macro_rules! ret {
    ($self: expr, $($args:tt)*) => {
        $self.stack.push(format!($($args)*))
    }
}

impl<'ast> Visit<'ast> for CVisitor {

    fn visit_abi(&mut self, node: &'ast Abi) {
        self.log("visit_abi");
        if let Some(it) = &node.name {
            self.visit_lit_str(it);
            let lit_str = self.tos();
            ret!(self, "extern {:?}", lit_str);
        } else {
            ret!(self, "extern ");
        }
    }

    fn visit_angle_bracketed_generic_arguments(&mut self, node: &'ast AngleBracketedGenericArguments) {
        self.log("visit_angle_bracketed_generic_arguments");
        let mut args = Vec::new();
        for el in Punctuated::pairs(&node.args) {
            let (it, _) = el.into_tuple();
            self.visit_generic_argument(it);
            args.push(self.tos());
        }
        ret!(self, "<{}>", args.join(", "));
    }

    fn visit_arm(&mut self, node: &'ast Arm) {
        self.log("visit_arm");
        self.visit_pat(&node.pat);
        let pat = self.tos();
        if let Some(it) = &node.guard {
            self.visit_expr(&*(it).1);
            let _guard = self.tos();
        };
        self.visit_expr(&*node.body);
        let expr = self.tos();
        ret!(self, "if {} {{{}}}", pat, expr);
    }

    fn visit_attr_style(&mut self, node: &'ast AttrStyle) {
        self.log("visit_attr_style");
        self.stack.push("<<attr_style>>".to_owned());
        //visit_attr_style(self, node);
    }

    fn visit_attribute(&mut self, node: &'ast Attribute) {
        self.log("visit_attribute");
        self.stack.push("<<attribute>>".to_owned());
        //visit_attribute(self, node);
    }

    fn visit_bare_fn_arg(&mut self, node: &'ast BareFnArg) {
        self.log("visit_bare_fn_arg");
        self.stack.push("<<bare_fn_arg>>".to_owned());
        //visit_bare_fn_arg(self, node);
    }

    fn visit_bin_op(&mut self, node: &'ast BinOp) {
        self.log("visit_bin_op");
        let op = match node {
            BinOp::Add(_binding_0) => {
                "+"
            }
            BinOp::Sub(_binding_0) => {
                "-"
            }
            BinOp::Mul(_binding_0) => {
                "*"
            }
            BinOp::Div(_binding_0) => {
                "/"
            }
            BinOp::Rem(_binding_0) => {
                "%"
            }
            BinOp::And(_binding_0) => {
                "&&"
            }
            BinOp::Or(_binding_0) => {
                "||"
            }
            BinOp::BitXor(_binding_0) => {
                "^"
            }
            BinOp::BitAnd(_binding_0) => {
                "&"
            }
            BinOp::BitOr(_binding_0) => {
                "|"
            }
            BinOp::Shl(_binding_0) => {
                "<<"
            }
            BinOp::Shr(_binding_0) => {
                ">>"
            }
            BinOp::Eq(_binding_0) => {
                "=="
            }
            BinOp::Lt(_binding_0) => {
                "<"
            }
            BinOp::Le(_binding_0) => {
                "<="
            }
            BinOp::Ne(_binding_0) => {
                "!="
            }
            BinOp::Ge(_binding_0) => {
                ">="
            }
            BinOp::Gt(_binding_0) => {
                ">"
            }
            BinOp::AddEq(_binding_0) => {
                "+="
            }
            BinOp::SubEq(_binding_0) => {
                "-="
            }
            BinOp::MulEq(_binding_0) => {
                "*="
            }
            BinOp::DivEq(_binding_0) => {
                "/="
            }
            BinOp::RemEq(_binding_0) => {
                "%="
            }
            BinOp::BitXorEq(_binding_0) => {
                "^="
            }
            BinOp::BitAndEq(_binding_0) => {
                "&="
            }
            BinOp::BitOrEq(_binding_0) => {
                "|="
            }
            BinOp::ShlEq(_binding_0) => {
                "<<="
            }
            BinOp::ShrEq(_binding_0) => {
                ">>="
            }
        };
        ret!(self, "{}", op);
    }

    fn visit_binding(&mut self, node: &'ast Binding) {
        self.log("visit_binding");
        self.stack.push("<<binding>>".to_owned());
        //visit_binding(self, node);
    }

    fn visit_block(&mut self, node: &'ast Block) {
        self.log("visit_block");

        let mut stmts = Vec::new();
        for it in &node.stmts {
            self.visit_stmt(it);
            stmts.push(self.tos());
        }
        ret!(self, "{{\n{}\n}}", stmts.join(";\n"));
    }

    fn visit_bound_lifetimes(&mut self, node: &'ast BoundLifetimes) {
        self.log("visit_bound_lifetimes");
        self.stack.push("<<bound_lifetimes>>".to_owned());
        //visit_bound_lifetimes(self, node);
    }

    fn visit_const_param(&mut self, node: &'ast ConstParam) {
        self.log("visit_const_param");
        self.stack.push("<<const_param>>".to_owned());
        //visit_const_param(self, node);
    }

    fn visit_constraint(&mut self, node: &'ast Constraint) {
        self.log("visit_constraint");
        self.stack.push("<<constraint>>".to_owned());
        //visit_constraint(self, node);
    }

    fn visit_data(&mut self, node: &'ast Data) {
        self.log("visit_data");
        self.stack.push("<<data>>".to_owned());
        //visit_data(self, node);
    }

    fn visit_data_enum(&mut self, node: &'ast DataEnum) {
        self.log("visit_data_enum");
        self.stack.push("<<data_enum>>".to_owned());
        //visit_data_enum(self, node);
    }

    fn visit_data_struct(&mut self, node: &'ast DataStruct) {
        self.log("visit_data_struct");
        self.stack.push("<<data_struct>>".to_owned());
        //visit_data_struct(self, node);
    }

    fn visit_data_union(&mut self, node: &'ast DataUnion) {
        self.log("visit_data_union");
        self.stack.push("<<data_union>>".to_owned());
        //visit_data_union(self, node);
    }

    fn visit_derive_input(&mut self, node: &'ast DeriveInput) {
        self.log("visit_derive_input");
        self.stack.push("<<derive_input>>".to_owned());
        //visit_derive_input(self, node);
    }

    fn visit_expr(&mut self, node: &'ast Expr) {
        self.log("visit_expr");
        visit_expr(self, node);
    }

    fn visit_expr_array(&mut self, node: &'ast ExprArray) {
        self.log("visit_expr_array");
        // self.stack.push("<<expr_array>>".to_owned());
        // visit_expr_array(self, node);
        let mut elems = Vec::new();
        for el in Punctuated::pairs(&node.elems) {
            let (it, _) = el.into_tuple();
            self.visit_expr(it);
            elems.push(self.tos());
        }
        ret!(self, "{}", elems.join(", "));
    }

    fn visit_expr_assign(&mut self, node: &'ast ExprAssign) {
        self.log("visit_expr_assign");
        // self.stack.push("<<expr_assign>>".to_owned());
        self.visit_expr(&*node.left);
        let left = self.tos();
        self.visit_expr(&*node.right);
        let right = self.tos();
        ret!(self, "{} = {}", left, right);
    }

    fn visit_expr_assign_op(&mut self, node: &'ast ExprAssignOp) {
        self.log("visit_expr_assign_op");
        self.stack.push("<<expr_assign_op>>".to_owned());
        // visit_expr_assign_op(self, node);
        self.visit_expr(&*node.left);
        let left = self.tos();
        self.visit_bin_op(&node.op);
        let op = self.tos();
        self.visit_expr(&*node.right);
        let right = self.tos();
        ret!(self, "{} {} {}", left, op, right);
    }

    fn visit_expr_async(&mut self, node: &'ast ExprAsync) {
        self.log("visit_expr_async");
        self.stack.push("<<expr_async>>".to_owned());
        //visit_expr_async(self, node);
    }

    fn visit_expr_await(&mut self, node: &'ast ExprAwait) {
        self.log("visit_expr_await");
        self.stack.push("<<expr_await>>".to_owned());
        //visit_expr_await(self, node);
    }

    fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
        self.log("visit_expr_binary");
        // self.stack.push("<<expr_binary>>".to_owned());
        // for it in &node.attrs {
        //     self.visit_attribute(it);
        // }
        self.visit_expr(&*node.left);
        let lhs = self.tos();
        self.visit_bin_op(&node.op);
        let op = self.tos();
        self.visit_expr(&*node.right);
        let rhs = self.tos();
        ret!(self, "{} {} {}", lhs, op, rhs);
    }

    fn visit_expr_block(&mut self, node: &'ast ExprBlock) {
        self.log("visit_expr_block");
        self.stack.push("<<expr_block>>".to_owned());
        // visit_expr_block(self, node);
    }

    fn visit_expr_box(&mut self, node: &'ast ExprBox) {
        self.log("visit_expr_box");
        self.stack.push("<<expr_box>>".to_owned());
        //visit_expr_box(self, node);
    }

    fn visit_expr_break(&mut self, node: &'ast ExprBreak) {
        self.log("visit_expr_break");
        // self.stack.push("<<expr_break>>".to_owned());
        // visit_expr_break(self, node);
        ret!(self, "break;");
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        self.log("visit_expr_call");
        self.visit_expr(&*node.func);
        let expr = self.tos();
        let mut args = Vec::new();
        for el in Punctuated::pairs(&node.args) {
            let (it, _) = el.into_tuple();
            self.visit_expr(it);
            args.push(self.tos());
        }
        ret!(self, "{}({})", expr, args.join(", "));
    }

    fn visit_expr_cast(&mut self, node: &'ast ExprCast) {
        self.log("visit_expr_cast");
        // self.stack.push("<<expr_cast>>".to_owned());
        // visit_expr_cast(self, node);
        self.visit_expr(&*node.expr);
        let expr = self.tos();
        // tokens_helper(v, &node.as_token.span);
        self.visit_type(&*node.ty);
        let ty = self.tos();
        ret!(self, "({}){}", ty, expr);
    }

    fn visit_expr_closure(&mut self, node: &'ast ExprClosure) {
        self.log("visit_expr_closure");
        self.stack.push("<<expr_closure>>".to_owned());
        //visit_expr_closure(self, node);
    }

    fn visit_expr_continue(&mut self, node: &'ast ExprContinue) {
        self.log("visit_expr_continue");
        self.stack.push("<<expr_continue>>".to_owned());
        //visit_expr_continue(self, node);
    }

    fn visit_expr_field(&mut self, node: &'ast ExprField) {
        self.log("visit_expr_field");
        self.stack.push("<<expr_field>>".to_owned());
        //visit_expr_field(self, node);
    }

    fn visit_expr_for_loop(&mut self, node: &'ast ExprForLoop) {
        self.log("visit_expr_for_loop");
        self.stack.push("<<expr_for_loop>>".to_owned());
        //visit_expr_for_loop(self, node);
    }

    fn visit_expr_group(&mut self, node: &'ast ExprGroup) {
        self.log("visit_expr_group");
        self.stack.push("<<expr_group>>".to_owned());
        //visit_expr_group(self, node);
    }

    fn visit_expr_if(&mut self, node: &'ast ExprIf) {
        self.log("visit_expr_if");
        self.stack.push("<<expr_if>>".to_owned());
        //visit_expr_if(self, node);
    }

    fn visit_expr_index(&mut self, node: &'ast ExprIndex) {
        self.log("visit_expr_index");
        self.stack.push("<<expr_index>>".to_owned());
        //visit_expr_index(self, node);
    }

    fn visit_expr_let(&mut self, node: &'ast ExprLet) {
        self.log("visit_expr_let");
        self.stack.push("<<expr_let>>".to_owned());
        //visit_expr_let(self, node);
    }

    fn visit_expr_lit(&mut self, node: &'ast ExprLit) {
        self.log("visit_expr_lit");
        // self.stack.push("<<expr_lit>>".to_owned());
        // for it in &node.attrs {
        //     self.visit_attribute(it);
        // }
        self.visit_lit(&node.lit);
    }

    fn visit_expr_loop(&mut self, node: &'ast ExprLoop) {
        self.log("visit_expr_loop");
        self.stack.push("<<expr_loop>>".to_owned());
        //visit_expr_loop(self, node);
    }

    fn visit_expr_macro(&mut self, node: &'ast ExprMacro) {
        self.log("visit_expr_macro");
        self.stack.push("<<expr_macro>>".to_owned());
        //visit_expr_macro(self, node);
    }

    fn visit_expr_match(&mut self, node: &'ast ExprMatch) {
        self.log("visit_expr_match");
        self.stack.push("<<expr_match>>".to_owned());
        //visit_expr_match(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast ExprMethodCall) {
        self.log("visit_expr_method_call");
        // self.stack.push("<<expr_method_call>>".to_owned());
        // visit_expr_method_call(self, node);
        let mut args = Vec::new();
        self.visit_expr(&*node.receiver);
        args.push(self.tos());

        self.visit_ident(&node.method);
        let indent = self.tos();

        // if let Some(it) = &node.turbofish {
        //     self.visit_method_turbofish(it);
        // };

        for el in Punctuated::pairs(&node.args) {
            let (it, _) = el.into_tuple();
            self.visit_expr(it);
            args.push(self.tos());
        }

        ret!(self, "{}({})", indent, args.join(", "));
    }

    fn visit_expr_paren(&mut self, node: &'ast ExprParen) {
        self.log("visit_expr_paren");
        self.visit_expr(&*node.expr);
        let expr = self.tos();
        ret!(self, "({})", expr);
    }

    fn visit_expr_path(&mut self, node: &'ast ExprPath) {
        self.log("visit_expr_path");
        // self.stack.push("<<expr_path>>".to_owned());
        // visit_expr_path(self, node);
        // for it in &node.attrs {
        //     self.visit_attribute(it);
        // }
        if let Some(it) = &node.qself {
            self.visit_qself(it);
            return;
        };
        self.visit_path(&node.path);
        let path = self.tos();
        ret!(self, "{}", path);
    }

    fn visit_expr_range(&mut self, node: &'ast ExprRange) {
        self.log("visit_expr_range");
        self.stack.push("<<expr_range>>".to_owned());
        //visit_expr_range(self, node);
    }

    fn visit_expr_reference(&mut self, node: &'ast ExprReference) {
        self.log("visit_expr_reference");
        self.stack.push("<<expr_reference>>".to_owned());
        //visit_expr_reference(self, node);
    }

    fn visit_expr_repeat(&mut self, node: &'ast ExprRepeat) {
        self.log("visit_expr_repeat");
        self.stack.push("<<expr_repeat>>".to_owned());
        //visit_expr_repeat(self, node);
    }

    fn visit_expr_return(&mut self, node: &'ast ExprReturn) {
        self.log("visit_expr_return");
        self.stack.push("<<expr_return>>".to_owned());
        //visit_expr_return(self, node);
    }

    fn visit_expr_struct(&mut self, node: &'ast ExprStruct) {
        self.log("visit_expr_struct");
        self.stack.push("<<expr_struct>>".to_owned());
        //visit_expr_struct(self, node);
    }

    fn visit_expr_try(&mut self, node: &'ast ExprTry) {
        self.log("visit_expr_try");
        self.stack.push("<<expr_try>>".to_owned());
        //visit_expr_try(self, node);
    }

    fn visit_expr_try_block(&mut self, node: &'ast ExprTryBlock) {
        self.log("visit_expr_try_block");
        self.stack.push("<<expr_try_block>>".to_owned());
        //visit_expr_try_block(self, node);
    }

    fn visit_expr_tuple(&mut self, node: &'ast ExprTuple) {
        self.log("visit_expr_tuple");
        self.stack.push("<<expr_tuple>>".to_owned());
        //visit_expr_tuple(self, node);
    }

    fn visit_expr_type(&mut self, node: &'ast ExprType) {
        self.log("visit_expr_type");
        self.stack.push("<<expr_type>>".to_owned());
        //visit_expr_type(self, node);
    }

    fn visit_expr_unary(&mut self, node: &'ast ExprUnary) {
        self.log("visit_expr_unary");
        self.visit_un_op(&node.op);
        let op = self.tos();
        self.visit_expr(&*node.expr);
        let expr = self.tos();
        ret!(self, "{}{}", op, expr);
    }

    fn visit_expr_unsafe(&mut self, node: &'ast ExprUnsafe) {
        self.log("visit_expr_unsafe");
        self.stack.push("<<expr_unsafe>>".to_owned());
        //visit_expr_unsafe(self, node);
    }

    fn visit_expr_while(&mut self, node: &'ast ExprWhile) {
        self.log("visit_expr_while");
        self.stack.push("<<expr_while>>".to_owned());
        //visit_expr_while(self, node);
    }

    fn visit_expr_yield(&mut self, node: &'ast ExprYield) {
        self.log("visit_expr_yield");
        self.stack.push("<<expr_yield>>".to_owned());
        //visit_expr_yield(self, node);
    }

    fn visit_field(&mut self, node: &'ast Field) {
        self.log("visit_field");
        self.stack.push("<<field>>".to_owned());
        //visit_field(self, node);
    }

    fn visit_field_pat(&mut self, node: &'ast FieldPat) {
        self.log("visit_field_pat");
        self.stack.push("<<field_pat>>".to_owned());
        //visit_field_pat(self, node);
    }

    fn visit_field_value(&mut self, node: &'ast FieldValue) {
        self.log("visit_field_value");
        self.stack.push("<<field_value>>".to_owned());
        //visit_field_value(self, node);
    }

    fn visit_fields(&mut self, node: &'ast Fields) {
        self.log("visit_fields");
        self.stack.push("<<fields>>".to_owned());
        //visit_fields(self, node);
    }

    fn visit_fields_named(&mut self, node: &'ast FieldsNamed) {
        self.log("visit_fields_named");
        self.stack.push("<<fields_named>>".to_owned());
        //visit_fields_named(self, node);
    }

    fn visit_fields_unnamed(&mut self, node: &'ast FieldsUnnamed) {
        self.log("visit_fields_unnamed");
        self.stack.push("<<fields_unnamed>>".to_owned());
        //visit_fields_unnamed(self, node);
    }

    fn visit_file(&mut self, node: &'ast File) {
        self.log("visit_file");

        // skip!(node.shebang);
        // for it in &node.attrs {
        //     self.visit_attribute(it);
        // }

        let mut items = Vec::new();
        for it in &node.items {
            self.visit_item(it);
            items.push(self.tos());
        }
        self.stack.push(items.join("\n\n"));
    }

    fn visit_fn_arg(&mut self, node: &'ast FnArg) {
        self.log("visit_fn_arg");
        visit_fn_arg(self, node);
    }

    fn visit_foreign_item(&mut self, node: &'ast ForeignItem) {
        self.log("visit_foreign_item");
        self.stack.push("<<foreign_item>>".to_owned());
        //visit_foreign_item(self, node);
    }

    fn visit_foreign_item_fn(&mut self, node: &'ast ForeignItemFn) {
        self.log("visit_foreign_item_fn");
        self.stack.push("<<foreign_item_fn>>".to_owned());
        //visit_foreign_item_fn(self, node);
    }

    fn visit_foreign_item_macro(&mut self, node: &'ast ForeignItemMacro) {
        self.log("visit_foreign_item_macro");
        self.stack.push("<<foreign_item_macro>>".to_owned());
        //visit_foreign_item_macro(self, node);
    }

    fn visit_foreign_item_static(&mut self, node: &'ast ForeignItemStatic) {
        self.log("visit_foreign_item_static");
        self.stack.push("<<foreign_item_static>>".to_owned());
        //visit_foreign_item_static(self, node);
    }

    fn visit_foreign_item_type(&mut self, node: &'ast ForeignItemType) {
        self.log("visit_foreign_item_type");
        self.stack.push("<<foreign_item_type>>".to_owned());
        //visit_foreign_item_type(self, node);
    }

    fn visit_generic_argument(&mut self, node: &'ast GenericArgument) {
        self.log("visit_generic_argument");
        self.stack.push("<<generic_argument>>".to_owned());
        //visit_generic_argument(self, node);
    }

    fn visit_generic_method_argument(&mut self, node: &'ast GenericMethodArgument) {
        self.log("visit_generic_method_argument");
        self.stack.push("<<generic_method_argument>>".to_owned());
        //visit_generic_method_argument(self, node);
    }

    fn visit_generic_param(&mut self, node: &'ast GenericParam) {
        self.log("visit_generic_param");
        self.stack.push("<<generic_param>>".to_owned());
        //visit_generic_param(self, node);
    }

    fn visit_generics(&mut self, node: &'ast Generics) {
        self.log("visit_generics");
        self.stack.push("<<generics>>".to_owned());
        //visit_generics(self, node);
    }
    fn visit_ident(&mut self, node: &'ast Ident) {
        self.log("visit_ident");
        // self.stack.push("<<ident>>".to_owned());
        //// visit_ident(self, node);
        let name = node.to_string();

        let name = match name.as_str() {
            "i32" => "int".to_owned(),
            "i64" => "long long".to_owned(),
            "u32" => "unsigned int".to_owned(),
            "u64" => "unsigned long long".to_owned(),
            "f32" => "float".to_owned(),
            "f64" => "double".to_owned(),
            _ => name,
        };

        self.stack.push(name);
    }

    fn visit_impl_item(&mut self, node: &'ast ImplItem) {
        self.log("visit_impl_item");
        self.stack.push("<<impl_item>>".to_owned());
        //visit_impl_item(self, node);
    }

    fn visit_impl_item_const(&mut self, node: &'ast ImplItemConst) {
        self.log("visit_impl_item_const");
        self.stack.push("<<impl_item_const>>".to_owned());
        //visit_impl_item_const(self, node);
    }

    fn visit_impl_item_macro(&mut self, node: &'ast ImplItemMacro) {
        self.log("visit_impl_item_macro");
        self.stack.push("<<impl_item_macro>>".to_owned());
        //visit_impl_item_macro(self, node);
    }

    fn visit_impl_item_method(&mut self, node: &'ast ImplItemMethod) {
        self.log("visit_impl_item_method");
        self.stack.push("<<impl_item_method>>".to_owned());
        //visit_impl_item_method(self, node);
    }

    fn visit_impl_item_type(&mut self, node: &'ast ImplItemType) {
        self.log("visit_impl_item_type");
        self.stack.push("<<impl_item_type>>".to_owned());
        //visit_impl_item_type(self, node);
    }

    fn visit_index(&mut self, node: &'ast Index) {
        self.log("visit_index");
        self.stack.push("<<index>>".to_owned());
        //visit_index(self, node);
    }

    fn visit_item(&mut self, node: &'ast Item) {
        self.log("visit_item");
        visit_item(self, node);
    }

    fn visit_item_const(&mut self, node: &'ast ItemConst) {
        self.log("visit_item_const");
        // for it in &node.attrs {
        //     self.visit_attribute(it);
        // }
        // self.visit_visibility(&node.vis);
        // tokens_helper(v, &node.const_token.span);
        self.visit_ident(&node.ident);
        let ident = self.tos();
        // tokens_helper(v, &node.colon_token.spans);
        self.visit_type(&*node.ty);
        let ty = self.tos();
        // tokens_helper(v, &node.eq_token.spans);
        self.visit_expr(&*node.expr);
        let expr = self.tos();
        // tokens_helper(v, &node.semi_token.spans);
        ret!(self, "const {} {} = {};", ty, ident, expr);
    }

    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        self.log("visit_item_enum");
        self.stack.push("<<item_enum>>".to_owned());
        //visit_item_enum(self, node);
    }

    fn visit_item_extern_crate(&mut self, node: &'ast ItemExternCrate) {
        self.log("visit_item_extern_crate");
        self.stack.push("<<item_extern_crate>>".to_owned());
        //visit_item_extern_crate(self, node);
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        // for it in &node.attrs {
        //     self.visit_attribute(it);
        // }
        // self.visit_visibility(&node.vis);
    
        self.visit_signature(&node.sig);
        let sig = self.tos();
    
        self.visit_block(&*node.block);
        let block = self.tos();
    
        ret!(self, "{} {}", sig, block);
    }

    fn visit_item_foreign_mod(&mut self, node: &'ast ItemForeignMod) {
        self.log("visit_item_foreign_mod");
        self.stack.push("<<item_foreign_mod>>".to_owned());
        //visit_item_foreign_mod(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        self.log("visit_item_impl");
        self.stack.push("<<item_impl>>".to_owned());
        //visit_item_impl(self, node);
    }

    fn visit_item_macro(&mut self, node: &'ast ItemMacro) {
        self.log("visit_item_macro");
        self.stack.push("<<item_macro>>".to_owned());
        //visit_item_macro(self, node);
    }

    fn visit_item_macro2(&mut self, node: &'ast ItemMacro2) {
        self.log("visit_item_macro2");
        self.stack.push("<<item_macro2>>".to_owned());
        //visit_item_macro2(self, node);
    }

    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        self.log("visit_item_mod");
        self.stack.push("<<item_mod>>".to_owned());
        //visit_item_mod(self, node);
    }

    fn visit_item_static(&mut self, node: &'ast ItemStatic) {
        self.log("visit_item_static");
        self.stack.push("<<item_static>>".to_owned());
        //visit_item_static(self, node);
    }

    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        self.log("visit_item_struct");
        self.stack.push("<<item_struct>>".to_owned());
        //visit_item_struct(self, node);
    }

    fn visit_item_trait(&mut self, node: &'ast ItemTrait) {
        self.log("visit_item_trait");
        self.stack.push("<<item_trait>>".to_owned());
        //visit_item_trait(self, node);
    }

    fn visit_item_trait_alias(&mut self, node: &'ast ItemTraitAlias) {
        self.log("visit_item_trait_alias");
        self.stack.push("<<item_trait_alias>>".to_owned());
        //visit_item_trait_alias(self, node);
    }

    fn visit_item_type(&mut self, node: &'ast ItemType) {
        self.log("visit_item_type");
        // for it in &node.attrs {
        //     self.visit_attribute(it);
        // }
        // self.visit_visibility(&node.vis);
        // tokens_helper(v, &node.type_token.span);
        self.visit_ident(&node.ident);
        let name = self.tos();
        // self.visit_generics(&node.generics);
        // tokens_helper(v, &node.eq_token.spans);
        self.visit_type(&*node.ty);
        let ty = self.tos();

        ret!(self, "typedef {} {};", ty, name);
    }

    fn visit_item_union(&mut self, node: &'ast ItemUnion) {
        self.log("visit_item_union");
        self.stack.push("<<item_union>>".to_owned());
        //visit_item_union(self, node);
    }

    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        self.log("visit_item_use");
        self.stack.push("<<item_use>>".to_owned());
        //visit_item_use(self, node);
    }

    fn visit_label(&mut self, node: &'ast Label) {
        self.log("visit_label");
        self.stack.push("<<label>>".to_owned());
        //visit_label(self, node);
    }
    fn visit_lifetime(&mut self, node: &'ast Lifetime) {
        self.log("visit_lifetime");
        self.stack.push("<<lifetime>>".to_owned());
        //visit_lifetime(self, node);
    }

    fn visit_lifetime_def(&mut self, node: &'ast LifetimeDef) {
        self.log("visit_lifetime_def");
        self.stack.push("<<lifetime_def>>".to_owned());
        //visit_lifetime_def(self, node);
    }
    fn visit_lit(&mut self, node: &'ast Lit) {
        self.log("visit_lit");
        // self.stack.push("<<lit>>".to_owned());
        visit_lit(self, node);
    }
    fn visit_lit_bool(&mut self, node: &'ast LitBool) {
        self.log("visit_lit_bool");
        self.stack.push("<<lit_bool>>".to_owned());
        //visit_lit_bool(self, node);
    }
    fn visit_lit_byte(&mut self, node: &'ast LitByte) {
        self.log("visit_lit_byte");
        self.stack.push("<<lit_byte>>".to_owned());
        //visit_lit_byte(self, node);
    }
    fn visit_lit_byte_str(&mut self, node: &'ast LitByteStr) {
        self.log("visit_lit_byte_str");
        self.stack.push("<<lit_byte_str>>".to_owned());
        //visit_lit_byte_str(self, node);
    }
    fn visit_lit_char(&mut self, node: &'ast LitChar) {
        self.log("visit_lit_char");
        self.stack.push("<<lit_char>>".to_owned());
        //visit_lit_char(self, node);
    }
    fn visit_lit_float(&mut self, node: &'ast LitFloat) {
        self.log("visit_lit_float");
        let suffix = node.suffix();
        match suffix {
            _ => ret!(self, "{}", node.base10_digits())
        }
    }

    fn visit_lit_int(&mut self, node: &'ast LitInt) {
        self.log("visit_lit_int");
        self.stack.push("<<lit_int>>".to_owned());
        //visit_lit_int(self, node);
    }
    fn visit_lit_str(&mut self, node: &'ast LitStr) {
        self.log("visit_lit_str");
        self.stack.push("<<lit_str>>".to_owned());
        //visit_lit_str(self, node);
    }

    fn visit_local(&mut self, node: &'ast Local) {
        self.log("visit_local");
        // self.stack.push("<<local>>".to_owned());
        // for it in &node.attrs {
        //     self.visit_attribute(it);
        // }
        // tokens_helper(v, &node.let_token.span);

        self.visit_pat(&node.pat);
        let pat = self.tos();
        if let Some(it) = &node.init {
            self.visit_expr(&*(it).1);
            let expr = self.tos();
            ret!(self, "{} = {}", pat, expr);
        } else {
            ret!(self, "{}", pat);
        }
    }

    fn visit_macro(&mut self, node: &'ast Macro) {
        self.log("visit_macro");
        self.stack.push("<<macro>>".to_owned());
        //visit_macro(self, node);
    }

    fn visit_macro_delimiter(&mut self, node: &'ast MacroDelimiter) {
        self.log("visit_macro_delimiter");
        self.stack.push("<<macro_delimiter>>".to_owned());
        //visit_macro_delimiter(self, node);
    }

    fn visit_member(&mut self, node: &'ast Member) {
        self.log("visit_member");
        self.stack.push("<<member>>".to_owned());
        //visit_member(self, node);
    }

    fn visit_meta(&mut self, node: &'ast Meta) {
        self.log("visit_meta");
        self.stack.push("<<meta>>".to_owned());
        //visit_meta(self, node);
    }

    fn visit_meta_list(&mut self, node: &'ast MetaList) {
        self.log("visit_meta_list");
        self.stack.push("<<meta_list>>".to_owned());
        //visit_meta_list(self, node);
    }

    fn visit_meta_name_value(&mut self, node: &'ast MetaNameValue) {
        self.log("visit_meta_name_value");
        self.stack.push("<<meta_name_value>>".to_owned());
        //visit_meta_name_value(self, node);
    }

    fn visit_method_turbofish(&mut self, node: &'ast MethodTurbofish) {
        self.log("visit_method_turbofish");
        self.stack.push("<<method_turbofish>>".to_owned());
        //visit_method_turbofish(self, node);
    }

    fn visit_nested_meta(&mut self, node: &'ast NestedMeta) {
        self.log("visit_nested_meta");
        self.stack.push("<<nested_meta>>".to_owned());
        //visit_nested_meta(self, node);
    }

    fn visit_parenthesized_generic_arguments(&mut self, node: &'ast ParenthesizedGenericArguments) {
        self.log("visit_parenthesized_generic_arguments");
        self.stack.push("<<parenthesized_generic_arguments>>".to_owned());
        //visit_parenthesized_generic_arguments(self, node);
    }

    fn visit_pat(&mut self, node: &'ast Pat) {
        self.log("visit_pat");
        visit_pat(self, node);
    }

    fn visit_pat_box(&mut self, node: &'ast PatBox) {
        self.log("visit_pat_box");
        self.stack.push("<<pat_box>>".to_owned());
        //visit_pat_box(self, node);
    }

    fn visit_pat_ident(&mut self, node: &'ast PatIdent) {
        self.log("visit_pat_ident");
        // visit_pat_ident(self, node);

        self.visit_ident(&node.ident);
    }

    fn visit_pat_lit(&mut self, node: &'ast PatLit) {
        self.log("visit_pat_lit");
        self.stack.push("<<pat_lit>>".to_owned());
        //visit_pat_lit(self, node);
    }

    fn visit_pat_macro(&mut self, node: &'ast PatMacro) {
        self.log("visit_pat_macro");
        self.stack.push("<<pat_macro>>".to_owned());
        //visit_pat_macro(self, node);
    }

    fn visit_pat_or(&mut self, node: &'ast PatOr) {
        self.log("visit_pat_or");
        self.stack.push("<<pat_or>>".to_owned());
        //visit_pat_or(self, node);
    }

    fn visit_pat_path(&mut self, node: &'ast PatPath) {
        self.log("visit_pat_path");
        self.stack.push("<<pat_path>>".to_owned());
        //visit_pat_path(self, node);
    }

    fn visit_pat_range(&mut self, node: &'ast PatRange) {
        self.log("visit_pat_range");
        self.stack.push("<<pat_range>>".to_owned());
        //visit_pat_range(self, node);
    }

    fn visit_pat_reference(&mut self, node: &'ast PatReference) {
        self.log("visit_pat_reference");
        self.stack.push("<<pat_reference>>".to_owned());
        //visit_pat_reference(self, node);
    }

    fn visit_pat_rest(&mut self, node: &'ast PatRest) {
        self.log("visit_pat_rest");
        self.stack.push("<<pat_rest>>".to_owned());
        //visit_pat_rest(self, node);
    }

    fn visit_pat_slice(&mut self, node: &'ast PatSlice) {
        self.log("visit_pat_slice");
        self.stack.push("<<pat_slice>>".to_owned());
        //visit_pat_slice(self, node);
    }

    fn visit_pat_struct(&mut self, node: &'ast PatStruct) {
        self.log("visit_pat_struct");
        self.stack.push("<<pat_struct>>".to_owned());
        //visit_pat_struct(self, node);
    }

    fn visit_pat_tuple(&mut self, node: &'ast PatTuple) {
        self.log("visit_pat_tuple");
        self.stack.push("<<pat_tuple>>".to_owned());
        //visit_pat_tuple(self, node);
    }

    fn visit_pat_tuple_struct(&mut self, node: &'ast PatTupleStruct) {
        self.log("visit_pat_tuple_struct");
        self.stack.push("<<pat_tuple_struct>>".to_owned());
        //visit_pat_tuple_struct(self, node);
    }

    fn visit_pat_type(&mut self, node: &'ast PatType) {
        self.log("visit_pat_type");

        // for it in &node.attrs {
        //     self.visit_attribute(it);
        // }

        self.visit_pat(&*node.pat);
        let pat = self.tos();
        self.visit_type(&*node.ty);
        let ty = self.tos();
        ret!(self, "{} {}", ty, pat);
    }

    fn visit_pat_wild(&mut self, node: &'ast PatWild) {
        self.log("visit_pat_wild");
        self.stack.push("<<pat_wild>>".to_owned());
        //visit_pat_wild(self, node);
    }

    fn visit_path(&mut self, node: &'ast Path) {
        self.log("visit_path");
        use quote::ToTokens;
        let path = node.to_token_stream().to_string();
        self.stack.push(path);
    }

    fn visit_path_arguments(&mut self, node: &'ast PathArguments) {
        self.log("visit_path_arguments");
        self.stack.push("<<path_arguments>>".to_owned());
        //visit_path_arguments(self, node);
    }

    fn visit_path_segment(&mut self, node: &'ast PathSegment) {
        self.log("visit_path_segment");
        self.stack.push("<<path_segment>>".to_owned());
        //visit_path_segment(self, node);
    }

    fn visit_predicate_eq(&mut self, node: &'ast PredicateEq) {
        self.log("visit_predicate_eq");
        self.stack.push("<<predicate_eq>>".to_owned());
        //visit_predicate_eq(self, node);
    }

    fn visit_predicate_lifetime(&mut self, node: &'ast PredicateLifetime) {
        self.log("visit_predicate_lifetime");
        self.stack.push("<<predicate_lifetime>>".to_owned());
        //visit_predicate_lifetime(self, node);
    }

    fn visit_predicate_type(&mut self, node: &'ast PredicateType) {
        self.log("visit_predicate_type");
        self.stack.push("<<predicate_type>>".to_owned());
        //visit_predicate_type(self, node);
    }

    fn visit_qself(&mut self, node: &'ast QSelf) {
        self.log("visit_qself");
        self.stack.push("<<qself>>".to_owned());
        //visit_qself(self, node);
    }

    fn visit_range_limits(&mut self, node: &'ast RangeLimits) {
        self.log("visit_range_limits");
        self.stack.push("<<range_limits>>".to_owned());
        //visit_range_limits(self, node);
    }

    fn visit_receiver(&mut self, node: &'ast Receiver) {
        self.log("visit_receiver");
        self.stack.push("<<receiver>>".to_owned());
        //visit_receiver(self, node);
    }

    fn visit_return_type(&mut self, node: &'ast ReturnType) {
        self.log("visit_return_type");
        match node {
            ReturnType::Default => ret!(self, "void"),
            ReturnType::Type(_, ty) => {
                self.visit_type(&**ty);
            }
        };
    }

    fn visit_signature(&mut self, node: &'ast Signature) {
        self.log("visit_signature");
        // self.stack.push("<<signature>>".to_owned());
        //// visit_signature(self, node);
        // if let Some(it) = &node.constness {
        //     tokens_helper(v, &it.span);
        // };
        // if let Some(it) = &node.asyncness {
        //     tokens_helper(v, &it.span);
        // };
        // if let Some(it) = &node.unsafety {
        //     tokens_helper(v, &it.span);
        // };
        // if let Some(it) = &node.abi {
        //     self.visit_abi(it);
        // };
        // tokens_helper(v, &node.fn_token.span);

        self.visit_ident(&node.ident);
        let ident = self.tos();

        // self.visit_generics(&node.generics);
        // tokens_helper(v, &node.paren_token.span);

        let mut args = Vec::new();
        for el in Punctuated::pairs(&node.inputs) {
            let (it, p) = el.into_tuple();
            self.visit_fn_arg(it);
            args.push(self.tos());
            // if let Some(p) = p {
            //     tokens_helper(v, &p.spans);
            // }
        }
        let args = args.join(", ");

        // if let Some(it) = &node.variadic {
        //     self.visit_variadic(it);
        // };

        self.visit_return_type(&node.output);
        let ret = self.tos();

        ret!(self, "{} {}({})", ret, ident, args);
    }

    fn visit_span(&mut self, node: &Span) {
        self.log("visit_span");
        self.stack.push("<<span>>".to_owned());
        //visit_span(self, node);
    }

    fn visit_stmt(&mut self, node: &'ast Stmt) {
        self.log("visit_stmt");
        visit_stmt(self, node);
    }

    fn visit_trait_bound(&mut self, node: &'ast TraitBound) {
        self.log("visit_trait_bound");
        self.stack.push("<<trait_bound>>".to_owned());
        //visit_trait_bound(self, node);
    }

    fn visit_trait_bound_modifier(&mut self, node: &'ast TraitBoundModifier) {
        self.log("visit_trait_bound_modifier");
        self.stack.push("<<trait_bound_modifier>>".to_owned());
        //visit_trait_bound_modifier(self, node);
    }

    fn visit_trait_item(&mut self, node: &'ast TraitItem) {
        self.log("visit_trait_item");
        self.stack.push("<<trait_item>>".to_owned());
        //visit_trait_item(self, node);
    }

    fn visit_trait_item_const(&mut self, node: &'ast TraitItemConst) {
        self.log("visit_trait_item_const");
        self.stack.push("<<trait_item_const>>".to_owned());
        //visit_trait_item_const(self, node);
    }

    fn visit_trait_item_macro(&mut self, node: &'ast TraitItemMacro) {
        self.log("visit_trait_item_macro");
        self.stack.push("<<trait_item_macro>>".to_owned());
        //visit_trait_item_macro(self, node);
    }

    fn visit_trait_item_method(&mut self, node: &'ast TraitItemMethod) {
        self.log("visit_trait_item_method");
        self.stack.push("<<trait_item_method>>".to_owned());
        //visit_trait_item_method(self, node);
    }

    fn visit_trait_item_type(&mut self, node: &'ast TraitItemType) {
        self.log("visit_trait_item_type");
        self.stack.push("<<trait_item_type>>".to_owned());
        //visit_trait_item_type(self, node);
    }

    fn visit_type(&mut self, node: &'ast Type) {
        self.log("visit_type");
        visit_type(self, node);
    }

    fn visit_type_array(&mut self, node: &'ast TypeArray) {
        self.log("visit_type_array");
        self.stack.push("<<type_array>>".to_owned());
        //visit_type_array(self, node);
    }

    fn visit_type_bare_fn(&mut self, node: &'ast TypeBareFn) {
        self.log("visit_type_bare_fn");
        self.stack.push("<<type_bare_fn>>".to_owned());
        //visit_type_bare_fn(self, node);
    }

    fn visit_type_group(&mut self, node: &'ast TypeGroup) {
        self.log("visit_type_group");
        self.stack.push("<<type_group>>".to_owned());
        //visit_type_group(self, node);
    }

    fn visit_type_impl_trait(&mut self, node: &'ast TypeImplTrait) {
        self.log("visit_type_impl_trait");
        self.stack.push("<<type_impl_trait>>".to_owned());
        //visit_type_impl_trait(self, node);
    }

    fn visit_type_infer(&mut self, node: &'ast TypeInfer) {
        self.log("visit_type_infer");
        self.stack.push("<<type_infer>>".to_owned());
        //visit_type_infer(self, node);
    }

    fn visit_type_macro(&mut self, node: &'ast TypeMacro) {
        self.log("visit_type_macro");
        self.stack.push("<<type_macro>>".to_owned());
        //visit_type_macro(self, node);
    }

    fn visit_type_never(&mut self, node: &'ast TypeNever) {
        self.log("visit_type_never");
        self.stack.push("<<type_never>>".to_owned());
        //visit_type_never(self, node);
    }

    fn visit_type_param(&mut self, node: &'ast TypeParam) {
        self.log("visit_type_param");
        self.stack.push("<<type_param>>".to_owned());
        //visit_type_param(self, node);
    }

    fn visit_type_param_bound(&mut self, node: &'ast TypeParamBound) {
        self.log("visit_type_param_bound");
        self.stack.push("<<type_param_bound>>".to_owned());
        //visit_type_param_bound(self, node);
    }

    fn visit_type_paren(&mut self, node: &'ast TypeParen) {
        self.log("visit_type_paren");
        self.stack.push("<<type_paren>>".to_owned());
        //visit_type_paren(self, node);
    }

    fn visit_type_path(&mut self, node: &'ast TypePath) {
        self.log("visit_type_path");
        // if let Some(it) = &node.qself {
        //     self.visit_qself(it);
        // };
        self.visit_path(&node.path);
    }

    fn visit_type_ptr(&mut self, node: &'ast TypePtr) {
        self.log("visit_type_ptr");
        self.stack.push("<<type_ptr>>".to_owned());
        //visit_type_ptr(self, node);
    }

    fn visit_type_reference(&mut self, node: &'ast TypeReference) {
        self.log("visit_type_reference");
        self.stack.push("<<type_reference>>".to_owned());
        //visit_type_reference(self, node);
    }

    fn visit_type_slice(&mut self, node: &'ast TypeSlice) {
        self.log("visit_type_slice");
        self.stack.push("<<type_slice>>".to_owned());
        //visit_type_slice(self, node);
    }

    fn visit_type_trait_object(&mut self, node: &'ast TypeTraitObject) {
        self.log("visit_type_trait_object");
        self.stack.push("<<type_trait_object>>".to_owned());
        //visit_type_trait_object(self, node);
    }

    fn visit_type_tuple(&mut self, node: &'ast TypeTuple) {
        self.log("visit_type_tuple");
        self.stack.push("<<type_tuple>>".to_owned());
        //visit_type_tuple(self, node);
    }

    fn visit_un_op(&mut self, node: &'ast UnOp) {
        self.log("visit_un_op");
        let op = match node {
            UnOp::Deref(_) => {
                "*"
            }
            UnOp::Not(_) => {
                "!"
            }
            UnOp::Neg(_) => {
                "-"
            }
        };
        ret!(self, "{}", op);
    }

    fn visit_use_glob(&mut self, node: &'ast UseGlob) {
        self.log("visit_use_glob");
        self.stack.push("<<use_glob>>".to_owned());
        //visit_use_glob(self, node);
    }

    fn visit_use_group(&mut self, node: &'ast UseGroup) {
        self.log("visit_use_group");
        self.stack.push("<<use_group>>".to_owned());
        //visit_use_group(self, node);
    }

    fn visit_use_name(&mut self, node: &'ast UseName) {
        self.log("visit_use_name");
        self.stack.push("<<use_name>>".to_owned());
        //visit_use_name(self, node);
    }

    fn visit_use_path(&mut self, node: &'ast UsePath) {
        self.log("visit_use_path");
        self.stack.push("<<use_path>>".to_owned());
        //visit_use_path(self, node);
    }

    fn visit_use_rename(&mut self, node: &'ast UseRename) {
        self.log("visit_use_rename");
        self.stack.push("<<use_rename>>".to_owned());
        //visit_use_rename(self, node);
    }

    fn visit_use_tree(&mut self, node: &'ast UseTree) {
        self.log("visit_use_tree");
        self.stack.push("<<use_tree>>".to_owned());
        //visit_use_tree(self, node);
    }

    fn visit_variadic(&mut self, node: &'ast Variadic) {
        self.log("visit_variadic");
        self.stack.push("<<variadic>>".to_owned());
        //visit_variadic(self, node);
    }

    fn visit_variant(&mut self, node: &'ast Variant) {
        self.log("visit_variant");
        self.stack.push("<<variant>>".to_owned());
        //visit_variant(self, node);
    }

    fn visit_vis_crate(&mut self, node: &'ast VisCrate) {
        self.log("visit_vis_crate");
        self.stack.push("<<vis_crate>>".to_owned());
        //visit_vis_crate(self, node);
    }

    fn visit_vis_public(&mut self, node: &'ast VisPublic) {
        self.log("visit_vis_public");
        self.stack.push("<<vis_public>>".to_owned());
        //visit_vis_public(self, node);
    }

    fn visit_vis_restricted(&mut self, node: &'ast VisRestricted) {
        self.log("visit_vis_restricted");
        self.stack.push("<<vis_restricted>>".to_owned());
        //visit_vis_restricted(self, node);
    }

    fn visit_visibility(&mut self, node: &'ast Visibility) {
        self.log("visit_visibility");
        self.stack.push("<<visibility>>".to_owned());
        //visit_visibility(self, node);
    }

    fn visit_where_clause(&mut self, node: &'ast WhereClause) {
        self.log("visit_where_clause");
        self.stack.push("<<where_clause>>".to_owned());
        //visit_where_clause(self, node);
    }

    fn visit_where_predicate(&mut self, node: &'ast WherePredicate) {
        self.log("visit_where_predicate");
        self.stack.push("<<where_predicate>>".to_owned());
        //visit_where_predicate(self, node);
    }
}

/// Translate a Rust file into C.
pub fn to_c(file: &syn::File) -> String {
    let mut cv = CVisitor::new();
    cv.visit_file(file);
    cv.tos()
}

#[test]
fn test() {
    let code : syn::File = parse_quote! {
        #[allow (non_camel_case_types)]
        type fty =f64 ;
        
        const RECIP_2PI :fty =0.1591549430918953357688837633725143620345;
        
        pub fn sin (arg : fty)->fty {
          let scaled :fty =arg *RECIP_2PI ;
          let x :fty =scaled -scaled .round ();
          (- 0.0000795978135564681619446994463825844449 as f64).mul_add (x * x , 0.0011251039233483632093906670512638370694 as f64).mul_add (x * x , - 0.0120293093815837587083929079549229916291 as f64).mul_add (x * x , 0.1042285941703196255136347927732329854039 as f64).mul_add (x * x , - 0.7181222077484850721256518873820977766730 as f64).mul_add (x * x , 3.8199525744232107661057457125514942650559 as f64).mul_add (x * x , - 15.0946425760590780811041894566331393312522 as f64).mul_add (x * x , 42.0586939448620164904399014801571486455824 as f64).mul_add (x * x , - 76.7058597530604003747392381683185548195638 as f64).mul_add (x * x , 81.6052492760750400501824780624242989421231 as f64).mul_add (x * x , - 41.3417022403997601538762424010531941752307 as f64).mul_add (x * x , 6.2831853071795864768497321650524941104931 as f64)*x 
        }
    };

    println!("res\n{}", to_c(&code));
}


