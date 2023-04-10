use proc_macro::TokenStream;
use quote::quote;
use litrs::StringLit;
use std::fmt::Write;
use std::str::FromStr;


#[proc_macro]
pub fn f(input: TokenStream) -> TokenStream {
    let input = input.into_iter().collect::<Vec<_>>();

    let input = if input.len() != 1 {
        let msg = format!("expected exactly one input token, got {}", input.len());
        return quote! { compile_error!(#msg) }.into();
    } else {
        match StringLit::try_from(&input[0]) {
            Err(e) => return e.to_compile_error(),
            Ok(lit) => lit.to_string(),
        }
    };
    let (mut res, mut args) = (String::new(), String::new());
    let (
        mut is_open, mut is_open_fresh, mut is_escape,
        mut chr_open, mut str_open
    ) = (false, false, false, false, false);
    let mut amt_open = 0;
    for tok in input.chars() {
        match &tok {
            '{' if !is_open || is_open_fresh => {
                if is_open_fresh {
                    (is_open, is_open_fresh) = (false, false);
                    write!(&mut res, "{{{{").unwrap();
                } else {
                    (is_open, is_open_fresh) = (true, true);
                }
            }
            '}' => {
                if is_open {
                    if amt_open == 0 {
                        // if let Some(new_args) = args.strip_suffix('=') {
                        //     args = new_args.to_string();
                        //     res = format!("{} = {{{}:#?}}", new_args, res.strip_suffix("{}").unwrap());
                        // }

                        if let Some(new_args) = args.strip_suffix(":?") {
                            args = new_args.to_string();
                            res = format!("{}:?}}", res.strip_suffix('}').unwrap());
                        } else if let Some(new_args) = args.strip_suffix(":#?") {
                            args = new_args.to_string();
                            res = format!("{}:#?}}", res.strip_suffix('}').unwrap());
                        }
                        is_open = false;
                    } else {
                        amt_open -= 1;
                    }
                } else {
                    write!(&mut res, "}}").unwrap()
                }
            }
            _ => {
                if is_open {
                    if is_escape { is_escape = false } else {
                        match tok {
                            '\'' => chr_open = !chr_open,
                            '"' => str_open = !str_open,
                            '{' => amt_open += 1,
                            _ => ()
                        }
                    }
                    if is_open_fresh {
                        is_open_fresh = false;
                        write!(&mut res, "{{}}").unwrap();
                        write!(&mut args, ", {tok}").unwrap()
                    } else {
                        write!(&mut args, "{tok}").unwrap()
                    }
                } else {
                    write!(&mut res, "{tok}").unwrap()
                }
            }, //3 dont need to do this one at a time
        }
    }
    let res = format!("format!({res}{args})");
    // panic!("{res}");
    TokenStream::from_str(&res).unwrap()
}
