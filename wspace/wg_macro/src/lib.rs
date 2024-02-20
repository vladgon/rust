use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{
    Data::Struct,
    DataStruct,
    Expr,
    Expr::Path,
    ExprAssign,
    ExprLit,
    ExprPath,
    Fields::Named,
    FieldsNamed,
};

#[proc_macro_attribute]
pub fn sql(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let annotated_item: syn::DeriveInput = syn::parse(annotated_item).unwrap();
    let input: Expr = syn::parse(input).unwrap();

    let left_right = match input {
        Expr::Assign(ExprAssign { left, right, .. }) => (left, right),
        _ => unimplemented!("Unsupported expression")
    };
    let left = match left_right.0.as_ref() {
        Path(ExprPath { path, .. }) => path.get_ident().unwrap(),
        _ => unimplemented!("Unsupported expression")
    };
    let right = match left_right.1.as_ref() {
        Expr::Lit(ExprLit { lit, .. }) => lit,
        _ => unimplemented!("Unsupported expression")
    };

    let fields = match annotated_item.data {
        Struct(DataStruct { fields: Named(FieldsNamed { ref named, .. }), .. }) => named,
        _ => unimplemented!("Only works for structs"),
    };
    let (impl_generics, ty_generics, where_clause) = &annotated_item.generics.split_for_impl();
    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { pub #name: #ty }
    });

    let name = &annotated_item.ident;
    let ident_lower_case =  &annotated_item.ident.to_string().to_lowercase();
    let ident_lower_case = format_ident!("{}", ident_lower_case);
    let attributes = &annotated_item.attrs;

    let gen = quote! {
        const #ident_lower_case :&str = "Sample Constant";
        #(#attributes)*
        pub struct  #name #ty_generics #where_clause {
            #(#builder_fields,)*
        }
       impl #impl_generics #name #ty_generics #where_clause{
            ///Gets the url from macro
            pub fn get_url(&self) -> String{
                let res = format!("{} => {}", stringify!(#left), #right);
                res.into()
            }
        }
    };
    gen.into()
}
