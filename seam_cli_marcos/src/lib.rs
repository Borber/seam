use std::vec;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

// TODO 尝试动态派发

#[proc_macro_derive(LivesToCommands)]
pub fn my_enum(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as DeriveInput);

    let expanded = match &expr.data {
        Data::Enum(ref data_enum) => {
            let mut add_filed = vec![];
            let mut node = vec![];
            let mut danmu = vec![];
            data_enum.variants.iter().for_each(|variant| {
                let variant_name = &variant.ident;
                let lower = variant_name.to_string().to_lowercase();
                let lower = syn::Ident::new(&lower, variant_name.span());
                let attrs = &variant.attrs;
                add_filed.push(quote! {
                    #(#attrs)*
                    #variant_name {
                        rid: String
                    }
                });
                node.push(quote! {
                    Commands::#variant_name{ rid } => {
                        let cli = seam_core::live::#lower::Client;
                        cli.get(&rid).await
                    }
                });
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
            }
        }
        _ => panic!("#[derive(LivesToCommands)] can only be used on enums."),
    };

    TokenStream::from(expanded)
}
