use std::vec;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

// TODO 若后续不再使用, 则删除
#[proc_macro_derive(LivesToCommands)]
pub fn my_enum(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as DeriveInput);

    let expanded = match &expr.data {
        Data::Enum(ref data_enum) => {
            let mut add_filed = vec![];
            let mut node = vec![];
            let mut danmu = vec![];
            let mut name = vec![];
            data_enum.variants.iter().for_each(|variant| {
                // 获取每个枚举元素
                let variant_name = &variant.ident;
                // 获取每个枚举元素的名称
                let lower_s = variant_name.to_string().to_lowercase();
                let lower = syn::Ident::new(&lower_s, variant_name.span());
                let attrs = &variant.attrs;
                // 生成新的枚举元素
                add_filed.push(quote! {
                    #(#attrs)*
                    #variant_name {
                        rid: String
                    }
                });
                // 每个枚举元素对应的名称
                name.push(quote! {
                    Commands::#variant_name{ rid } => #lower_s
                });
                // 对应名称平台的 Client 对象
                node.push(quote! {
                    Commands::#variant_name{ rid } => {
                        let cli = seam_core::live::#lower::Client;
                        cli.get(&rid).await
                    }
                });

                // 对应平台名称的 Danmu 对象
                danmu.push(quote! {
                    Commands::#variant_name{ rid } => seam_danmu::danmu::#lower::Danmu::start(&rid, recorder).await
                });
            });

            quote! {
                #[derive(clap::Subcommand, Clone)]
                pub enum Commands {
                    #(#add_filed),*
                }

                use seam_core::live::Live;
                impl Commands {
                    pub async fn get(&self) -> seam_core::error::Result<seam_core::live::Node>  {
                        match self {
                            #(#node),*
                        }
                    }

                    pub async fn danmu(&self, recorder: Vec<&dyn DanmuRecorder>) -> seam_danmu::error::Result<()>{
                        match self {
                            #(#danmu),*
                        }
                    }
                }

                impl std::fmt::Display for Commands {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let s = match self {
                            #(#name),*
                        };
                        write!(f, "{}", s)
                    }
                }
            }
        }
        _ => panic!("#[derive(LivesToCommands)] can only be used on enums."),
    };

    TokenStream::from(expanded)
}
