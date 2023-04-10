use proc_macro::{TokenStream, TokenTree, Spacing};
use std::collections::HashSet;
use std::fmt::Write;
use std::str::FromStr;


#[proc_macro]
pub fn comp(input: TokenStream) -> TokenStream {
    let (value, is_hm, loops, cond)
        = parse_comprehension(input);
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
        write!(&mut res, "res.insert({});", value).unwrap();
    } else {
        write!(&mut res, "res.push({});", value).unwrap();
    }
    write!(&mut res, "{} res }}", "}".repeat(close)).unwrap();
    TokenStream::from_str(&res).unwrap()
}

#[proc_macro]
pub fn i_comp(input: TokenStream) -> TokenStream {
    let (mut value, is_hm, loops, cond)
        = parse_comprehension(input);

    // let mut res = get_until(loop, {*index+=1; index}, HashSet::from(["while", "for", "if"]));
    if is_hm {
        value = format!("({})", value);
    }
    if let Some(cond) = cond {
        write!(&mut res,
               ".filter_map(|{}| {} {{ Some({}) }} else {{ None }})",
               iter.var, cond, value
        ).unwrap();
    } else {
        write!(&mut res, ".map(|{}| {})", iter.var, value).unwrap();
    }
    TokenStream::from_str(&res).unwrap()
}

fn parse_comprehension(input: TokenStream) -> (String, bool, Vec<String>, Option<String>) {
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
    'loops: while index < input.len() && input[index].to_string().as_str() != "if" {
        let mut cur_for = format!("{} ", input[index].to_string());
        index += 1;
        while index < input.len() &&
            !matches!(input[index].to_string().as_str(), "while" | "for" | "if")
        {
            // if is_iter {
            //     iter_var = Some(get_iter_var(&input, &mut index));
            //     if index < input.len() && input[index].to_string().as_str() != "if" {
            //         panic!("can't have more than one loop in `i_comp` try using `comp` instead");
            //     }
            //     break 'loops;
            // }
            write_token(&mut cur_for, &input[index]);
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
    (map, is_hm, loops, cond)
}

struct LoopParts {
    iter: String,
    var: String
}

fn get_iter_var(input: &Vec<TokenTree>, index: &mut usize) -> LoopParts {
    LoopParts {
        var:  get_until(input, index, HashSet::from(["in"])),
        iter: get_until(input, {*index+=1; index}, HashSet::from(["while", "for", "if"]))
    }
}

fn get_until(input: &Vec<TokenTree>, index: &mut usize, until: HashSet<&str>) -> String {
    let mut str = String::new();
    while *index < input.len() && !until.contains(input[*index].to_string().as_str()) {
        write_token(&mut str, &input[*index]);
        *index += 1;
    }
    str
}

fn is_lone_colon(input: &Vec<TokenTree>, index: usize, token: &TokenTree, token_st: &String) -> bool {
    token_st == ":"
        && is_spaced(token)
        && (index == 0 || is_spaced(&input[index - 1]))
}

fn is_spaced(token: &TokenTree) -> bool {
    if let TokenTree::Punct(punct) = token {
        if let Spacing::Joint = punct.spacing() {
            return false;
        }
    }
    true
}

fn write_token(str: &mut String, token: &TokenTree){
    if is_spaced(&token) {
        write!(str, "{} ", token.to_string()).unwrap();
    } else {
        write!(str, "{}",  token.to_string()).unwrap();
    }
}