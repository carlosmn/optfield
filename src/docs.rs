use syn::{Attribute, ItemStruct, LitStr};

use crate::args::{Args, Doc};

const DOC: &str = "doc";

/// Changes itemin place.
/// If args.doc is None, removes doc attrs from item.
/// If args.doc is Some(Same), does nothing.
/// If args.doc is Some(Custom(docs)), sets doc attrs to docs.
pub fn generate(item: &mut ItemStruct, args: &Args) {
    use Doc::*;

    match &args.doc {
        Some(Same) => return,
        None => remove_doc_attrs(item),
        Some(Custom(docs)) => replace_doc_attrs(item, docs),
    }
}

fn remove_doc_attrs(item: &mut ItemStruct) {
    let mut new_attrs = Vec::with_capacity(item.attrs.len());

    for attr in &item.attrs {
        if !is_doc_attr(attr) {
            new_attrs.push(attr.clone());
        }
    }

    item.attrs = new_attrs;
}

fn replace_doc_attrs(item: &mut ItemStruct, docs: &LitStr) {
    remove_doc_attrs(item);

    unimplemented!();
}

fn is_doc_attr(attr: &Attribute) -> bool {
    attr.path.is_ident(DOC)
}