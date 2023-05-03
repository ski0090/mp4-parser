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
    AtomContainer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MacroAction {
    Print,
    Rename,
    Iter,
    Structure,
    AtomContainer,
}

impl From<Action> for MacroAction {
    fn from(value: Action) -> Self {
        match &value {
            Action::Print => MacroAction::Print,
            Action::Rename(_) => MacroAction::Rename,
            Action::Iter => MacroAction::Iter,
            Action::Structure => MacroAction::Structure,
            Action::AtomContainer => MacroAction::AtomContainer,
        }
    }
}

impl Parse for Action {
    fn parse(input: ParseStream) -> Result<Self> {
        syn::custom_keyword!(rename);
        syn::custom_keyword!(iter);
        syn::custom_keyword!(st);
        syn::custom_keyword!(atom_container);
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
        } else if input.peek(atom_container) {
            let _ = input.parse::<atom_container>()?;
            Ok(Action::AtomContainer)
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
    action: MacroAction,
}

impl Field {
    fn from_field(field: &syn::Field) -> Result<Option<Self>> {
        let name: Ident = field
            .ident
            .clone()
            .ok_or(Error::new(Span::call_site(), Problem::UnnamedField))?;
        let action = get_action_from(field.attrs.as_slice())?;
        match &action {
            Some(Action::Rename(ident)) => Ok(Some(Field {
                name,
                fn_name: ident.clone(),
                action: action.unwrap().into(),
            })),
            Some(_) => Ok(Some(Field {
                name: name.clone(),
                fn_name: name,
                action: action.unwrap().into(),
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

    fn create_print_function(&self, struct_name: &Ident) -> TokenStream {
        let field_name = &self.name;
        let fn_print = format_ident!("print_{}", &self.fn_name);
        let comment = format!("Get field {} from instance of {}.", field_name, struct_name,);

        match self.action {
            MacroAction::Rename | MacroAction::Print => quote!(
                #[doc=#comment]
                pub fn #fn_print(&self) {
                    self.base.print_depth();
                    println!("{}: {}", stringify!(#field_name) , self.#field_name);
                }
            ),
            MacroAction::Iter => quote!(
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
            ),
            MacroAction::Structure => quote!(
                #[doc=#comment]
                pub fn #fn_print(&self) {
                    self.base.print_depth();
                    println!("{}: {:?}", stringify!(#field_name) , self.#field_name);
                }
            ),
            MacroAction::AtomContainer => quote!(
                #[doc=#comment]
                pub fn #fn_print(&self) {
                    self.#field_name.iter().for_each(|atom| atom.print_comp());
                }
            ),
        }
    }

    fn create_print_comp_func(&self) -> TokenStream {
        let field_name = &self.name;
        let fn_print = format_ident!("print_{}", field_name);
        if self.action == MacroAction::AtomContainer {}
        quote!(
            self.#fn_print();
        )
    }
}

pub struct PrinterStruct<'a> {
    original: &'a DeriveInput,
    st_name: Ident,
    fields: Vec<Field>,
}

impl<'a> PrinterStruct<'a> {
    pub fn create_print(&self) -> TokenStream {
        let (impl_generics, struct_generics, where_clause) =
            self.original.generics.split_for_impl();
        let struct_name = &self.st_name;
        let methods: Vec<TokenStream> = self
            .fields
            .iter()
            .map(|field| field.create_print_function(&self.st_name))
            .collect();

        quote!(
            impl #impl_generics #struct_name #struct_generics
                #where_clause
            {
                #(#methods)*
            }
        )
    }

    pub fn create_mp4_print(&self) -> TokenStream {
        let (impl_generics, struct_generics, where_clause) =
            self.original.generics.split_for_impl();
        let struct_name = &self.st_name;
        let methods: Vec<TokenStream> = self
            .fields
            .iter()
            .map(|field| field.create_print_function(&self.st_name))
            .collect();

        let methods_call: Vec<TokenStream> = self
            .fields
            .iter()
            .map(|field| field.create_print_comp_func())
            .collect();

        quote!(
            impl #impl_generics #struct_name #struct_generics
                #where_clause
            {
                #(#methods)*
            }

            impl crate::atoms::Mp4AtomPrint for #struct_name {
                fn print_comp(&self) {
                    self.base.print();
                    #(#methods_call)*
                }
            }
        )
    }
}

impl<'a> TryFrom<&'a DeriveInput> for PrinterStruct<'a> {
    type Error = Error;

    fn try_from(node: &'a DeriveInput) -> Result<Self> {
        let struct_data = named_struct(node)?;
        let named_fields = named_fields(struct_data)?;
        let fields = Field::from_fields_named(named_fields)?;

        Ok(PrinterStruct {
            original: node,
            st_name: node.ident.clone(),
            fields,
        })
    }
}
