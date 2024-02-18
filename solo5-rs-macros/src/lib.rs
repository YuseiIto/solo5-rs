#![feature(box_patterns)]
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, FnArg, Ident, ItemFn, PatType, ReturnType, Type, TypePath};

/* NOTE:
 * This is one of the cheap wrapper macro and is not considered to be a good interface,
 * but is adopted because I cannot come up with a better way to do it at the moment.
*/

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    #[cfg(feature = "alloc")]
    let tlsf_init = quote! {
     // solo5_rs::tlsf::GLOBAL.init(start.heap_start, start.heap_size as usize);
      unsafe {
        solo5_rs::ALLOCATOR.lock().init(start.heap_start as *mut u8, start.heap_size as usize);
    }
    };

    #[cfg(not(feature = "alloc"))]
    let tlsf_init = quote! {};

    let body = item.clone();
    let body = parse_macro_input!(body as ItemFn);
    let ident = body.sig.ident.clone();
    let inputs = body.sig.inputs.clone();
    let solo5_main_arg = match inputs.len() {
        0 => {
            quote! {}
        }
        1 => {
            let arg = inputs.first().unwrap();
            match arg {
                FnArg::Typed(PatType {
                    attrs: _,
                    pat: _,
                    colon_token: _,
                    ty: box Type::Path(TypePath { qself: _, path }),
                }) => {
                    if path.get_ident().unwrap().to_owned()
                        == Ident::new("Solo5StartInfo", Span::call_site())
                    {
                        quote! {
                            solo5_rs::Solo5StartInfo::from(start)
                        }
                    } else {
                        let entry = quote! {
                            compile_error!("The only argument of solo5_rs::main function must be of type Solo5StartInfo.");
                            #body
                        };

                        return TokenStream::from(entry);
                    }
                }
                _ => {
                    let entry = quote! {
                        compile_error!("solo5_rs::main function does not take `self`.");
                        #body
                    };

                    return TokenStream::from(entry);
                }
            }
        }
        _ => {
            let entry = quote! {
                compile_error!("solo5_rs::main function has too many arguments.");
                #body
            };

            return TokenStream::from(entry);
        }
    };

    let call_main = match body.sig.output.clone() {
        ReturnType::Default => quote! {
        #ident(#solo5_main_arg);
                return 0;
            },
        ReturnType::Type(_, _) => quote! {
        u64::from(#ident(#solo5_main_arg))
            },
    };

    let entry = quote! {
    #[no_mangle]
    pub extern "C" fn solo5_app_main(start: &solo5_sys::solo5_start_info) -> u64 {
        #tlsf_init
        #call_main
    }
        };

    let mut entry = TokenStream::from(entry);
    entry.extend(item);

    entry
}
