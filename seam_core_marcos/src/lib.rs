use std::path::Path;

use proc_macro::TokenStream;
use quote::quote;

/// 返回 `fn all() -> HashMap<String, Box<dyn Live>>` 函数
///
/// 通过扫描 live 文件夹，自动生成，返回所有直播平台对应的 hashmap
///
/// 需要引入:
///     - HashMap: std::collections::HashMap 或 hashbrown::HashMap 均可
///     - Live: seam_core::live::Live
///
/// 因为固定了扫描 live 文件夹，所以这个宏只能在 seam_core 中使用
#[proc_macro]
pub fn gen_all(_: TokenStream) -> TokenStream {
    // 获取 live 文件夹位置
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&root).join("src").join("live");
    let path = path.as_path();

    let mut lives = vec![];

    // 遍历 live 文件夹, 找出所有的rust源文件, 获取文件名
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file = entry.path();
        // 判断是否为文件 && 是否为rs文件
        if file.is_file() && file.extension().unwrap_or_default() == "rs" {
            let file = file.file_stem().unwrap().to_str().unwrap();
            if file != "mod" {
                lives.push(file.to_owned())
            }
        }
    }

    // 通过文件名, 生成对应的代码
    let lives = lives
        .into_iter()
        .map(|live| {
            let ident = syn::Ident::new(&live, proc_macro2::Span::call_site());
            quote! {
                map.insert(String::from(#live), Arc::new(crate::live::#ident::Client));
            }
        })
        .collect::<Vec<_>>();

    quote! {
        /// 返回 core 支持的所有平台
        ///
        /// 请参照 [Live](seam_core::live::Live) 的文档
        pub fn all() -> HashMap<String, Arc<dyn Live>> {
            let mut map: HashMap<String, Arc<dyn Live>> = HashMap::new();
            #(#lives)*
            map
        }
    }
    .into()
}

#[proc_macro]
pub fn gen_test(input: TokenStream) -> TokenStream {
    let arg: String = input.to_string();
    let code = quote! {
        mod tests {
            use super::*;
            #[tokio::test]
            async fn test_get() {
                let cli = Client;
                match cli.get(#arg).await {
                    Ok(node) => println!("{}", node.json()),
                    Err(e) => println!("{e}"),
                }
            }
        }
    };
    code.into()
}
