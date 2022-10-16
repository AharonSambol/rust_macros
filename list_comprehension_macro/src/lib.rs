use proc_macro::{TokenStream, TokenTree, Spacing};
use std::fmt::Write;
use std::str::FromStr;


#[proc_macro]
pub fn comp(input: TokenStream) -> TokenStream {
    let input: Vec<TokenTree> = input.into_iter().collect();
    let mut map = String::new();
    let mut index = 0;
    let mut is_hm = false;
    while index < input.len() {
        let token = &input[index];
        let token_st = token.to_string();
        if let "for" | "while" = token_st.as_str() {
            break;
        }
        if is_lone_colon(&input, index, token, &token_st) {
            is_hm = true;
            write!(&mut map, ", ").unwrap();
        } else {
            write_token(&mut map, token);
        }
        index += 1;
    }
    if index == input.len() {
        panic!("list comprehension needs a `for` but none were found")
    }
    let mut loops = Vec::new();
    while index < input.len() &&
        matches!(input[index].to_string().as_str(), "for" | "while")
    {
        let mut cur_for = String::from(input[index].to_string() + " ");
        index += 1;
        while index < input.len() {
            let token = &input[index];
            if let "while" | "for" | "if" = token.to_string().as_str() {  break; }
            write_token(&mut cur_for, &token);
            index += 1;
        }
        loops.push(cur_for);
    }
    let mut cond = None;
    if index != input.len() {
        let mut condition = String::new();
        for token in input.iter().skip(index) {
            write_token(&mut condition, &token);
        }
        cond = Some(condition);
    }
    let mut res = String::from(
        if is_hm {  "{ let mut res = std::collections::HashMap::new();" }
        else     {  "{ let mut res = Vec::new();"    }
    );
    let mut close = 0;
    for loop_ in loops {
        write!(&mut res, "{} {{", loop_).unwrap();
        close += 1;
    }
    if let Some(cond) = cond {
        write!(&mut res, "{} {{", cond).unwrap();
        close += 1;
    }
    if is_hm {
        write!(&mut res, "res.insert({});", map).unwrap();
    } else {
        write!(&mut res, "res.push({});", map).unwrap();
    }
    write!(&mut res, "{} res }}", "}".repeat(close)).unwrap();
    dbg!(&res);
    TokenStream::from_str(&res).unwrap()
}

fn is_lone_colon(input: &Vec<TokenTree>, index: usize, token: &TokenTree, token_st: &String) -> bool {
    token_st == ":"
        && is_space(token)
        && (index == 0 || is_space(&input[index - 1]))
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
        write!(str, "{}",  token.to_string()).unwrap();
    }
}