use syn::{LitInt, LitStr, Token, punctuated::Punctuated, parse_macro_input, DeriveInput, parenthesized, ItemStruct};
use quote::{quote, ToTokens};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::parse::{ParseBuffer, Parser};
use syn::token::Token;

struct NumberList {
    numbers: Punctuated<LitInt, Token![,]>,
}

impl syn::parse::Parse for NumberList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(NumberList {
            numbers: Punctuated::parse_terminated(input)?
        })
    }
}

#[proc_macro_derive(Simple, attributes(numbers, strings))]
pub fn simple(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, attrs, .. } = parse_macro_input!(input);

    let mut numbers = vec![];

    for attr in attrs {
        if attr.path().is_ident("numbers") {
            let number_list = attr.parse_args::<NumberList>().unwrap().numbers;
            // let number_list = attr.parse_args_with(
            //     Punctuated::<LitInt, Token![,]>::parse_terminated
            // ).unwrap();

            for number in number_list {
                numbers.push(number.base10_parse::<i32>().unwrap());
            }
        }
    }

    let output = quote! {
        impl #ident {
            pub fn output() {
                println!("{:?}", [ #( #numbers ),* ]);
            }
        }
    };

    TokenStream::from(output)
}

struct ValuesList {
    pub items: Vec<(String, String)>,
}

impl syn::parse::Parse for ValuesList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = vec![];

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let value: Ident = input.parse()?;

            items.push((key.to_string(), value.to_string()));

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { items })
    }
}

#[proc_macro_derive(Middle, attributes(values))]
pub fn middle(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, attrs, .. } = parse_macro_input!(input);

    let mut items = vec![];

    for attr in attrs {
        if attr.path().is_ident("values") {
            let items_list = attr.parse_args::<ValuesList>().unwrap();

            for item in items_list.items {
                items.push(item.0);
                items.push(item.1);
            }
        }
    }

    let items = items.join(" ");

    let output = quote! {
        impl #ident {
            pub fn output() {
                println!("{:?}", #items);
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_derive(Hard, attributes(values, value))]
pub fn hard(input: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, attrs, .. } = parse_macro_input!(input);

    let mut items = vec![];
    let mut values = vec![];

    for attr in attrs {
        if attr.path().is_ident("values") {
            let items_list = attr.parse_args::<ValuesList>().unwrap();

            for item in items_list.items {
                items.push(item.0);
                items.push(item.1);
            }
        }
    }

    for field in fields.iter() {
        field.attrs.iter().for_each(|attr| {
            if attr.path().is_ident("value") {
                let value = attr.parse_args::<LitStr>().unwrap();
                values.push(value.value());
            }
        });
    }

    let items = items.join(" ");
    let values = values.join(" ");

    let output = quote! {
        impl #ident {
            pub fn output() {
                println!("{:?}", #items);
                println!("{:?}", #values);
            }
        }
    };

    TokenStream::from(output)
}