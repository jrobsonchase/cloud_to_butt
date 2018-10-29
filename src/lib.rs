extern crate proc_macro;

#[proc_macro_attribute]
pub fn cloud_to_butt(
    attr: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    assert!(
        attr.is_empty(),
        "cloud_to_butt attribute takes no arguments"
    );
    impl_cloud_to_butt(body)
}

fn impl_cloud_to_butt(body: proc_macro::TokenStream) -> proc_macro::TokenStream {
    body.into_iter().map(replace_cloud).collect()
}

fn replace_cloud(tree: proc_macro::TokenTree) -> proc_macro::TokenTree {
    match tree {
        proc_macro::TokenTree::Ident(id) => {
            let span = id.span();
            let text = id.to_string();
            let text = text.replace("cloud", "butt");
            proc_macro::Ident::new(&text, span).into()
        }
        proc_macro::TokenTree::Literal(_) => {
            if let Ok(str_lit) = syn::parse::<syn::LitStr>(tree.clone().into()) {
                let text = str_lit.value().replace("cloud", "butt");
                proc_macro::Literal::string(&text).into()
            } else {
                tree
            }
        }
        proc_macro::TokenTree::Group(grp) => {
            let delim = grp.delimiter();
            let stream = grp.stream();
            proc_macro::Group::new(delim, stream.into_iter().map(replace_cloud).collect()).into()
        }
        t => t,
    }
}
