use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn gen_test(input: TokenStream) -> TokenStream {
    let arg = input.to_string();
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
