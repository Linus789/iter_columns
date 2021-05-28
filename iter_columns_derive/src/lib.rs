extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(IterColumns)]
pub fn iter_columns_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_iter_columns_macro(&ast)
}

fn impl_iter_columns_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_struct = quote::format_ident!("{}Struct", name);
    let name_ext = quote::format_ident!("{}Ext", name);
    let name_options_struct = quote::format_ident!("{}OptionsStruct", name);

    let field_ty = match &ast.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(f) => &f.named[0].ty,
            syn::Fields::Unnamed(f) => &f.unnamed[0].ty,
            syn::Fields::Unit => panic!(),
        },
        syn::Data::Enum(_) => panic!(),
        syn::Data::Union(_) => panic!(),
    };

    let gen = quote! {
        pub struct #name_struct<'a, T> {
            arr: Vec<#field_ty>,
            len: usize,
            index: usize,
        }

        impl<'a, T: 'a> Iterator for #name_struct<'a, T> {
            type Item = Vec<&'a T>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index == self.len {
                    return None;
                }

                let index = self.index;
                let column = self.arr.iter().filter_map(move |row| row.get(index)).collect();

                self.index += 1;
                Some(column)
            }
        }

        pub trait #name_ext<'a, T: 'a>: Iterator<Item = #field_ty> {
            fn columns(self) -> #name_struct<'a, T>
            where
                Self: Sized,
            {
                let arr: Vec<#field_ty> = self.collect();
                let len = arr.iter().map(|row| row.len()).max().unwrap_or(0);

                #name_struct { arr, len, index: 0 }
            }

            fn columns_options(self) -> #name_options_struct<'a, T>
            where
                Self: Sized,
            {
                let arr: Vec<#field_ty> = self.collect();
                let len = arr.iter().map(|row| row.len()).max().unwrap_or(0);

                #name_options_struct { arr, len, index: 0 }
            }
        }

        impl<'a, T: 'a, I: Iterator<Item = #field_ty>> #name_ext<'a, T> for I {}

        pub struct #name_options_struct<'a, T> {
            arr: Vec<#field_ty>,
            len: usize,
            index: usize,
        }

        impl<'a, T: 'a> Iterator for #name_options_struct<'a, T> {
            type Item = Vec<Option<&'a T>>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index == self.len {
                    return None;
                }

                let index = self.index;
                let column = self.arr.iter().map(move |row| row.get(index)).collect();

                self.index += 1;
                Some(column)
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(IterColumnsArray)]
pub fn iter_columns_array_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_iter_columns_array_macro(&ast)
}

fn impl_iter_columns_array_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_struct = quote::format_ident!("{}Struct", name);
    let name_ext = quote::format_ident!("{}Ext", name);

    let field_ty = match &ast.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(f) => &f.named[0].ty,
            syn::Fields::Unnamed(f) => &f.unnamed[0].ty,
            syn::Fields::Unit => panic!(),
        },
        syn::Data::Enum(_) => panic!(),
        syn::Data::Union(_) => panic!(),
    };

    let gen = quote! {
        pub struct #name_struct<'a, T, const N: usize> {
            arr: Vec<#field_ty>,
            len: usize,
            index: usize,
        }

        impl<'a, T: 'a, const N: usize> Iterator for #name_struct<'a, T, N> {
            type Item = Vec<&'a T>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index == self.len {
                    return None;
                }

                let index = self.index;
                let column = self.arr.iter().filter_map(move |row| row.get(index)).collect();

                self.index += 1;
                Some(column)
            }
        }

        pub trait #name_ext<'a, T: 'a, const N: usize>: Iterator<Item = #field_ty> {
            fn columns(self) -> #name_struct<'a, T, N>
            where
                Self: Sized,
            {
                let arr: Vec<#field_ty> = self.collect();
                let len = N;

                #name_struct { arr, len, index: 0 }
            }
        }

        impl<'a, T: 'a, I: Iterator<Item = #field_ty>, const N: usize> #name_ext<'a, T, N> for I {}
    };

    gen.into()
}
