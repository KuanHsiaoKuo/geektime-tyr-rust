mod builder;
mod builder_with_attr;
mod raw_builder;

use proc_macro::TokenStream;
use raw_builder::BuilderContext;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    // 只有修改代码之后再次编译才会执行
    println!("{:#?}", input);
    "fn hello() { println!(\"Hello world!\"); }"
        .parse()
        .unwrap()
}

#[proc_macro_derive(RawBuilder)]
pub fn derive_raw_builder(input: TokenStream) -> TokenStream {
    // 只有修改代码之后再次编译才会执行
    println!("{:#?}", input);
    BuilderContext::render(input).unwrap().parse().unwrap()
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("{:#?}", input);
    builder::BuilderContext::from(input).render().into()
}

#[proc_macro_derive(BuilderWithAttr, attributes(builder))]
pub fn derive_builder_with_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("{:#?}", input);
    builder_with_attr::BuilderContext::from(input)
        .render()
        .into()
}
