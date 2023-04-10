    use std::collections::HashMap;
use std::str::FromStr;
use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn base(input: TokenStream) -> TokenStream {
   let token_stream: Vec<TokenTree> = input.into_iter().collect();

   if token_stream.len() != 3 {
       let comma_count = token_stream.iter().filter(|x| x.to_string() == ",").count();
       if comma_count == 0 {
           panic!("expected 2 args found {}", if token_stream.len() > 0 { '1' } else { '0' })
       } else if comma_count > 1 {
           panic!("expected 2 args found {}", comma_count + 1)
       }
       panic!("expected 3 tokens found {}", token_stream.len())
   }

   let mut map = HashMap::new();

   let second_arg = token_stream[2].to_string();
   let base;
   if second_arg.starts_with('"') && second_arg.ends_with('"') {
       base = second_arg.len() - 2;
       second_arg[1..second_arg.len()-1].chars()
   } else {
       base = second_arg.parse().unwrap_or_else(
           |_| panic!("base must be a `usize`")
       );
       "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars()
   }.enumerate().for_each(|(i, c)| {    map.insert(c, i); });

   let comma = token_stream[1].to_string();
   if comma != "," { panic!("Expected `,` found `{}`", comma) }
   let num = token_stream[0].to_string();

   let mut res = 0;
   for (i, digit) in num.chars().rev().enumerate(){
       res += map.get(&digit).unwrap_or_else(
           || panic!("number contains unknown char `{}`", digit)
       ) * base.pow(i as u32);
   }
   TokenStream::from_str(res.to_string().as_str()).unwrap()
}
