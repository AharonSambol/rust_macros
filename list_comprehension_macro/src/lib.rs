use proc_macro::{TokenStream, TokenTree, Spacing};
use std::fmt::Write;
use std::str::FromStr;

#[proc_macro]
pub fn comp(input: TokenStream) -> TokenStream {
    let input: Vec<TokenTree> = input.into_iter().collect();
    let mut map = String::new();
    let mut index = 0;
    while index < input.len() {
        let token = &input[index];
        if token.to_string() == "for" {
            break;
        }
        write_token(&mut map, token);
        index += 1;
    }
    if index == input.len() {
        panic!("list comprehension needs a `for` but none were found")
    }
    let mut for_loops = Vec::new();
    while index < input.len() && input[index].to_string() == "for" {
        let mut cur_for = String::new();
        index += 1;
        while index < input.len() {
            let token = &input[index];
            if let "for" | "if" = token.to_string().as_str() {  break; }
            write_token(&mut cur_for, &token);
            index += 1;
        }
        for_loops.push(cur_for);
    }
    let mut cond = None;
    if index != input.len() {
        let mut condition = String::new();
        for token in input.iter().skip(index) {
            write_token(&mut condition, &token);
        }
        cond = Some(condition);
    }

    let mut res = String::from("{ let mut res = Vec::new();");
    let mut close = 0;
    for for_loop in for_loops {
        write!(&mut res, "for {} {{", for_loop).unwrap();
        close += 1;
    }
    if let Some(cond) = cond {
        write!(&mut res, "{} {{", cond).unwrap();
        close += 1;
    }
    write!(&mut res, "res.push({});", map).unwrap();
    write!(&mut res, "{} res }}", "}".repeat(close)).unwrap();
    // dbg!(&res);
    TokenStream::from_str(&res).unwrap()
}

fn is_space(token: &TokenTree) -> bool {
    if let TokenTree::Punct(punct) = token {
        if let Spacing::Joint = punct.spacing() {
            return false;
        }
    }
    true
}

fn write_token(str: &mut String, token: &TokenTree){
    if is_space(&token) {
        write!(str, "{} ", token.to_string()).unwrap();
    } else {
        write!(str, "{}", token.to_string()).unwrap();
    }
}