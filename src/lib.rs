use std::ops::Deref;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, BinOp, Expr, Lit, Token, Type, UnOp,
};

struct CalculateExpr {
    expr: Expr,
    result_type: Type,
}

impl Parse for CalculateExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr: Expr = input.parse()?;
        input.parse::<Token![;]>()?;
        let result_type: Type = input.parse()?;
        Ok(Self { expr, result_type })
    }
}

#[proc_macro]
pub fn calc(input: TokenStream) -> TokenStream {
    let CalculateExpr { expr, result_type } = parse_macro_input!(input as CalculateExpr);
    let evaluated_expr = evaluate_expression(&expr, &result_type);
    let result = quote! {
        {
            let result: #result_type = #evaluated_expr;
            result
        }
    };
    result.into()
}

fn evaluate_expression(expr: &Expr, result_type: &Type) -> TokenStream2 {
    match expr {
        Expr::Binary(op) => {
            let left = evaluate_expression(&op.left, result_type);
            let right = evaluate_expression(&op.right, result_type);
            match op.op {
                BinOp::Add(_) => quote! { #left + #right },
                BinOp::Sub(_) => quote! { #left - #right },
                BinOp::Mul(_) => quote! { #left * #right },
                BinOp::Div(_) => quote! { #left / #right },
                BinOp::Rem(_) => quote! { #left % #right },
                BinOp::BitXor(_) => quote! { #left ^ #right },
                BinOp::BitAnd(_) => quote! { #left & #right },
                BinOp::BitOr(_) => quote! { #left | #right },
                BinOp::Shl(_) => quote! { #left << #right },
                BinOp::Shr(_) => quote! { #left >> #right },
                // 可以添加更多二元操作符
                _ => panic!("Unsupported binary operation"),
            }
        }
        Expr::Unary(op) => {
            let expr = evaluate_expression(&op.expr, result_type);
            match op.op {
                UnOp::Neg(_) => quote! { - #expr },
                // 可以添加更多一元操作符
                _ => panic!("Unsupported unary operation"),
            }
        }
        Expr::Lit(lit) => match &lit.lit {
            Lit::Int(int_lit) => {
                let int_value = int_lit.base10_parse::<i128>().unwrap();
                let int_value: TokenStream2 = match result_type {
                    Type::Path(path) if path.path.is_ident("u16") => quote! { #int_value as u16 },
                    Type::Path(path) if path.path.is_ident("u32") => quote! { #int_value as u32 },
                    Type::Path(path) if path.path.is_ident("u64") => quote! { #int_value as u64 },
                    Type::Path(path) if path.path.is_ident("u128") => quote! { #int_value as u128 },
                    Type::Path(path) if path.path.is_ident("i8") => quote! { #int_value as i8 },
                    Type::Path(path) if path.path.is_ident("i16") => quote! { #int_value as i16 },
                    Type::Path(path) if path.path.is_ident("i32") => quote! { #int_value as i32 },
                    Type::Path(path) if path.path.is_ident("i64") => quote! { #int_value as i64 },
                    Type::Path(path) if path.path.is_ident("i128") => quote! { #int_value as i128 },
                    // 可以添加更多整数类型处理
                    _ => panic!("Unsupported result type for integer literal"),
                };
                int_value
            }
            Lit::Float(float_lit) => {
                let float_value = float_lit.base10_parse::<f64>().unwrap();
                let float_value: TokenStream2 = match result_type {
                    Type::Path(path) if path.path.is_ident("f32") => quote! { #float_value as f32 },
                    Type::Path(path) if path.path.is_ident("f64") => quote! { #float_value as f64 },
                    // 可以添加更多浮点数类型处理
                    _ => panic!("Unsupported result type for float literal"),
                };
                float_value
            }
            Lit::Bool(bool_lit) => quote! { #bool_lit },
            // 可以添加更多字面量类型
            _ => panic!("Unsupported literal type"),
        },
        Expr::Paren(paren) => {
            let paren = evaluate_expression(&paren.expr, result_type);
            quote! { (#paren) }
        }
        Expr::Call(call) => {
            let func = match call.func.deref() {
                Expr::Path(path) => path.path.get_ident().unwrap().to_string(),
                _ => panic!("Unsupported function call"),
            };
            let args = call
                .args
                .iter()
                .map(|arg| evaluate_expression(arg, result_type))
                .collect::<Vec<_>>();
            match func.as_str() {
                "max" => {
                    let [ref arg0, ref arg1] = args[..] else {
                        panic!("Unsupported function call");
                    };
                    quote! { ::core::cmp::max(#arg0, #arg1) }
                }
                "min" => {
                    let [ref arg0, ref arg1] = args[..] else {
                        panic!("Unsupported function call");
                    };
                    quote! { ::core::cmp::min(#arg0, #arg1) }
                }
                // 可以添加更多函数支持
                _ => panic!("Unsupported function call"),
            }
        }
        Expr::Path(path) => {
            // 将变量转换为目标类型
            match result_type {
                Type::Path(_) => quote! { #path as #result_type },
                _ => panic!("Unsupported result type for variable"),
            }
        }
        // 处理其他可能的表达式类型，如索引访问、字段访问等
        _ => panic!("Unsupported expression type"),
    }
}
