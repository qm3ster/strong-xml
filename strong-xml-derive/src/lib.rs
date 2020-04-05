#![recursion_limit = "256"]

extern crate proc_macro;

mod read;
mod types;
mod write;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam};
use types::Element;

#[proc_macro_derive(XmlRead, attributes(xml))]
pub fn derive_xml_read(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;

    let lifetime = match &generics.params.last() {
        Some(GenericParam::Lifetime(lt)) => Some(lt),
        _ => None,
    };

    let impl_read = read::impl_read(Element::parse(input.clone()));

    let gen = quote! {
        impl #generics #name #generics {
            fn from_str(
                text: & #lifetime str
            ) -> strong_xml::XmlResult<#name #generics> {
                let mut reader = strong_xml::XmlReader::new(text);
                Self::from_reader(&mut reader)
            }

            fn from_reader(
                mut reader: &mut strong_xml::XmlReader #generics
            ) -> strong_xml::XmlResult<#name #generics> {
                use strong_xml::xmlparser::{ElementEnd, Token, Tokenizer};
                use strong_xml::XmlError;
                #impl_read
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(XmlWrite, attributes(xml))]
pub fn derive_xml_write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;

    let impl_write = write::impl_write(Element::parse(input.clone()));

    let gen = quote! {
        impl #generics #name #generics {
            pub(crate) fn to_string(&self) -> strong_xml::XmlResult<String> {
                let mut writer = strong_xml::XmlWriter::new(Vec::new());

                self.to_writer(&mut writer)?;

                Ok(String::from_utf8(writer.inner)?)
            }

            pub(crate) fn to_writer<W: std::io::Write>(
                &self,
                mut writer: &mut strong_xml::XmlWriter<W>
            ) -> strong_xml::XmlResult<()> {
                #impl_write

                Ok(())
            }
        }
    };

    gen.into()
}
