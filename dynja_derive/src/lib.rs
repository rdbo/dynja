use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, ExprAssign, LitStr};

fn get_template_path(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ExprAssign);

    let err_path_tokstr: TokenStream = quote! {
        compile_error!("Missing required 'path = \"<PATH>\"'");
    }
    .into();

    if args.left.to_token_stream().to_string() != "path" {
        return err_path_tokstr;
    }

    args.right.to_token_stream().into()
}

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("Failed to parse DeriveInput");

    let struct_ident = ast.ident;
    let struct_data = if let syn::Data::Struct(data) = ast.data {
        data
    } else {
        return quote! { compile_error!("Missing struct for derive") }.into();
    };

    let mut template_attr = None;
    for attr in &ast.attrs {
        if !attr.path().is_ident("template") {
            continue;
        }

        template_attr = Some(
            attr.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
                .expect("Failed to parse arguments from #[template]"),
        )
    }

    let template_attr = template_attr.expect("Missing #[template(path = \"<PATH>\")]");

    let template_path = get_template_path(template_attr.to_token_stream().into());
    let template_path = parse_macro_input!(template_path as LitStr);

    let mut context_str = String::new();
    for field in struct_data.fields {
        let ident = field.ident.expect("Failed to read struct field");
        context_str += format!("{} => self.{}, ", ident, ident).as_str();
    }

    let context: proc_macro2::TokenStream = context_str.parse().unwrap();

    quote! {
        impl dynja::TemplateFile for #struct_ident {
            const PATH: &'static str = #template_path;
        }

        impl #struct_ident {
            fn render(&self) -> String {
                // TODO: Don't use unwraps
                let template = dynja::templates().get_template(<Self as dynja::TemplateFile>::PATH).unwrap();
                template.render(dynja::minijinja::context!(#context)).unwrap()
            }
        }
    }
    .into()
}
