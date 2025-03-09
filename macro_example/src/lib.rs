use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析属性参数（例如日志消息）
    let args_str = args.to_string();

    // 解析函数定义
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident; // 获取函数名
    let fn_body = &input_fn.block; // 获取函数体
    let fn_vis = &input_fn.vis; // 函数可见性
    let fn_sig = &input_fn.sig; // 函数签名

    // 生成新的代码，包含日志功能
    let expanded = quote! {
        #fn_vis #fn_sig {
            // 在函数执行前打印日志
            println!("Entering function '{}', message: {}", stringify!(#fn_name), #args_str);
            let result = (|| #fn_body)(); // 执行原函数体
            // 在函数结束后打印日志
            println!("Exiting function '{}'", stringify!(#fn_name));
            result // 返回结果（如果有）
        }
    };

    // 转换为 TokenStream 并返回
    TokenStream::from(expanded)
}