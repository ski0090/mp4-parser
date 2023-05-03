use crate::{
    errors::Problem,
    extract::{named_fields, named_struct},
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    Attribute, DeriveInput, Error, FieldsNamed, LitStr, Result,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    Print,
    Rename(Ident),
    Iter,
    Structure,
}

impl Parse for Action {
    fn parse(input: ParseStream) -> Result<Self> {
        syn::custom_keyword!(rename);
        syn::custom_keyword!(iter);
        syn::custom_keyword!(st);
        if input.peek(rename) {
            let _ = input.parse::<rename>()?;
            let _ = input.parse::<syn::Token![=]>()?;
            let name = input.parse::<LitStr>()?;
            if !input.is_empty() {
                Err(Error::new(Span::call_site(), Problem::TokensFollowNewName))
            } else {
                Ok(Action::Rename(Ident::new(
                    name.value().as_str(),
                    Span::call_site(),
                )))
            }
        } else if input.peek(iter) {
            let _ = input.parse::<iter>()?;
            Ok(Action::Iter)
        } else if input.peek(st) {
            let _ = input.parse::<st>()?;
            Ok(Action::Structure)
        } else {
            Ok(Action::Print)
        }
    }
}

fn get_action_from(attributes: &[Attribute]) -> Result<Option<Action>> {
    let mut current: Option<Action> = None;

    for attr in attributes {
        if attr.path().is_ident("print_comp") {
            current = Some(attr.parse_args::<Action>()?);
        }
    }

    Ok(current)
}

pub struct Field {
    name: Ident,
    fn_name: Ident,
    has_iter: bool,
    is_st: bool,
}

impl Field {
    fn from_field(field: &syn::Field) -> Result<Option<Self>> {
        let name: Ident = field
            .ident
            .clone()
            .ok_or(Error::new(Span::call_site(), Problem::UnnamedField))?;

        match get_action_from(field.attrs.as_slice())? {
            Some(Action::Print) => Ok(Some(Field {
                name: name.clone(),
                fn_name: name,
                has_iter: false,
                is_st: false,
            })),
            Some(Action::Rename(ident)) => Ok(Some(Field {
                name,
                fn_name: ident,
                has_iter: false,
                is_st: false,
            })),
            Some(Action::Iter) => Ok(Some(Field {
                name: name.clone(),
                fn_name: name,
                has_iter: true,
                is_st: false,
            })),
            Some(Action::Structure) => Ok(Some(Field {
                name: name.clone(),
                fn_name: name,
                has_iter: false,
                is_st: true,
            })),
            None => Ok(None),
        }
    }

    fn from_fields_named(fields_named: &FieldsNamed) -> Result<Vec<Self>> {
        fields_named
            .named
            .iter()
            .try_fold(Vec::new(), |mut fields, field| {
                if let Some(field) = Field::from_field(field)? {
                    fields.push(field);
                }

                Ok(fields)
            })
    }

    fn emit(&self, struct_name: &Ident) -> TokenStream {
        let field_name = &self.name;
        let fn_print = format_ident!("print_{}", &self.fn_name);
        let comment = format!("Get field {} from instance of {}.", field_name, struct_name,);

        if self.has_iter {
            quote!(
                #[doc=#comment]
                pub fn #fn_print(&self) {
                    self.base.print_depth();
                    println!("<{}>",stringify!(#field_name));
                    self.#field_name.iter().for_each(|b| {
                        self.base.print_depth();
                        print!("\t");
                        println!("{},", b);
                    });
                    self.base.print_depth();
                    println!("</{}>",stringify!(#field_name));
                }
            )
        } else if self.is_st {
            quote!(
                #[doc=#comment]
                pub fn #fn_print(&self) {
                    self.base.print_depth();
                    println!("{}: {:?}", stringify!(#field_name) , self.#field_name);
                }
            )
        } else {
            quote!(
                #[doc=#comment]
                pub fn #fn_print(&self) {
                    self.base.print_depth();
                    println!("{}: {}", stringify!(#field_name) , self.#field_name);
                }
            )
        }
    }
}

pub struct AtiomPrintSt<'a> {
    original: &'a DeriveInput,
    st_name: Ident,
    fields: Vec<Field>,
}

impl<'a> AtiomPrintSt<'a> {
    pub fn emit(&self) -> TokenStream {
        let (impl_generics, struct_generics, where_clause) =
            self.original.generics.split_for_impl();
        let struct_name = &self.st_name;
        let methods: Vec<TokenStream> = self
            .fields
            .iter()
            .map(|field| field.emit(&self.st_name))
            .collect();

        quote!(
            impl #impl_generics #struct_name #struct_generics
                #where_clause
            {
                #(#methods)*
            }
        )
    }
}

impl<'a> TryFrom<&'a DeriveInput> for AtiomPrintSt<'a> {
    type Error = Error;

    fn try_from(node: &'a DeriveInput) -> Result<Self> {
        let struct_data = named_struct(node)?;
        let named_fields = named_fields(struct_data)?;
        let fields = Field::from_fields_named(named_fields)?;

        Ok(AtiomPrintSt {
            original: node,
            st_name: node.ident.clone(),
            fields,
        })
    }
}
