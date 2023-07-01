use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);

    let mut tlsf_init = quote! {};
    // FIXME: これは引数ではなくてcfgで見た方がいいかも
    for a in args {
        match a {
            NestedMeta::Lit(Lit::Str(l)) => {
                println!("{}", l.value().to_string());
                if l.value().eq("alloc") {
                    tlsf_init = quote!(
                    solo5_rs::tlsf::GLOBAL.init(start.heap_start, start.heap_size as usize);
                        )
                }
            }
            _ => {
                return TokenStream::from(
                    quote! {compile_error!("Invalid attribute is specified for `solo5_main`");},
                )
            }
        }
    }

    let body = item.clone();
    let body = parse_macro_input!(body as ItemFn);
    let ident = body.sig.ident;

    let entry = quote! {
    #[no_mangle]
    pub extern "C" fn solo5_app_main(start: &solo5_sys::solo5_start_info) -> u64 {
        #tlsf_init
        #ident();
        return 0;
    }
        };

    let mut entry = TokenStream::from(entry);
    entry.extend(item);

    entry
}
