use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn gen_test(input: TokenStream) -> TokenStream {
    let arg = input.to_string();
    let code = quote! {
        #[cfg(test)]
        mod tests {
            use super::*;
            #[tokio::test]
            async fn test_get() {
                match Client::get(#arg).await {
                    Ok(node) => println!("{}", node.json()),
                    Err(e) => println!("{e}"),
                }
            }
        }
    };
    code.into()
}
