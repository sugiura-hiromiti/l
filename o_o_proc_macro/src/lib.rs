use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use syn::parse::Parse;
use syn::parse_macro_input;

extern crate proc_macro;

#[proc_macro_attribute]
pub fn empty_trait_impl_block(attr: TokenStream, item: TokenStream,) -> TokenStream {
	let types = parse_macro_input!(attr as Types);
	let trait_def = parse_macro_input!(item as syn::ItemTrait);

	todo!()
}

struct Types {
	type_list: Vec<syn::Type,>,
}

impl Types {
	pub fn iter(&self,) -> std::slice::Iter<'_, syn::Type,> {
		self.type_list.iter()
	}
}

impl Parse for Types {
	fn parse(input: syn::parse::ParseStream,) -> syn::Result<Self,> {
		let parsed = input.step(|c| {
			let mut rest = *c;
			let mut type_list = vec![];

			while let Some((tt, next,),) = rest.token_tree() {
				match tt {
					TokenTree::Ident(idnt,) => {
						let ty: syn::Type = syn::parse_quote! { #idnt };
						type_list.push(ty,);
						rest = next;
					},
					TokenTree::Punct(_,) => rest = next,
					_ => {
						return Err(syn::Error::new(
							tt.span(),
							format!("parse failed\ntoken tree is: {tt:#?}"),
						),);
					},
				};
			}
			Ok((Types { type_list, }, rest,),)
		},)?;
		Ok(parsed,)
	}
}
