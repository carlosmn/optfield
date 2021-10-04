use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Fields, Ident, Index, ItemStruct};

use crate::args::{Args, MergeFnName};
use crate::fields;

const DEFAULT_FN_NAME: &str = "merge_opt";

pub fn generate(item: &ItemStruct, opt_item: &ItemStruct, args: &Args) -> TokenStream {
    if let Some(merge_fn) = &args.merge {
        let fn_name = match &merge_fn.name {
            MergeFnName::Custom(n) => n.clone(),
            MergeFnName::Default => Ident::new(DEFAULT_FN_NAME, Span::call_site()),
        };

        let fn_vis = &merge_fn.visibility;

        let item_name = &item.ident;
        let item_generics = &item.generics;

        let opt_name = &opt_item.ident;
        let opt_generics = &opt_item.generics;

        let fields = field_bindings(&item.fields, args, &fn_name);

        quote! {
            impl#item_generics #item_name#item_generics {
                #fn_vis fn #fn_name(&mut self, opt: #opt_name#opt_generics) {
                    #fields
                }
            }
        }
    } else {
        TokenStream::new()
    }
}

fn field_bindings(fields: &Fields, args: &Args, fn_name: &Ident) -> TokenStream {
    let mut tokens = TokenStream::new();

    for (i, field) in fields.iter().enumerate() {
        let field_name = match &field.ident {
            // means that original item is a tuple struct
            None => {
                let index = Index::from(i);

                quote!(#index)
            }
            Some(ident) => quote!(#ident),
        };

        let field_tokens = if fields::is_option(field) && !args.rewrap {
            quote! {
                if opt.#field_name.is_some() {
                    self.#field_name = opt.#field_name;
                }
            }
        } else if fields::find_rename(field, args).is_some() {
            quote! {
                self.#field_name.#fn_name(opt.#field_name);
            }
        } else {
            quote! {
                if let Some(value) = opt.#field_name {
                    self.#field_name = value
                }
            }
        };

        tokens.extend(field_tokens);
    }

    tokens
}
