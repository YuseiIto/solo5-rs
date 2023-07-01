use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(feature = "alloc")]
    let tlsf_init = quote! {
    solo5_rs::tlsf::GLOBAL.init(start.heap_start, start.heap_size as usize);
    };
    #[cfg(not(feature = "alloc"))]
    let tlsf_init = quote! {};

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
