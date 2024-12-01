#![feature(const_type_name)]
mod extract;
mod store;

use extract::*;
use proc_macro::TokenStream;
use store::*;

fn target_impl(store: &Store,) -> proc_macro2::TokenStream {
	let Store { ident, builder_ident, .. } = &store;
	let head = store.generics_template(ident,);
	let members = store.members();

	quote::quote! {
		impl #head {
			pub fn builder() -> #builder_ident {
				#builder_ident {
					#(#members: std::option::Option::None,)*
				}
			}
		}
	}
}

fn builder_decl(store: &Store,) -> proc_macro2::TokenStream {
	let Store { builder_ident, .. } = &store;
	let (ident, ty,): (Vec<&syn::Ident,>, Vec<&syn::Type,>,) =
		store.fields().map(ident_and_ty,).unzip();
	let head = store.generics_template(builder_ident,);

	quote::quote! {
		pub struct #head {
			#(#ident: #ty,)*
		}
	}
}

fn setter_impl(store: &Store,) -> Result<proc_macro2::TokenStream, syn::Error,> {
	let head = store.generics_template(&store.builder_ident,);

	let methods = store.fields().map(setter_for_a_field,).try_fold(
		quote::quote! {},
		|accum, ts| match ts {
			Ok(ts,) => Ok(quote::quote! {
				#accum
				#ts
			},),
			Err(e,) => Err(e,),
		},
	)?;
	// let types = store.fields().map(|f| {
	// 	let f = unwrap(f.clone(),);
	// 	f.ty
	// },);

	Ok(quote::quote! {
		impl #head {
			#methods
		}
	},)
}

fn setter_for_a_field(f: &syn::Field,) -> Result<proc_macro2::TokenStream, syn::Error,> {
	let Setter { ident, param_type, kind, } = Setter::new(f,)?;

	let signature = quote::quote! {
		pub fn #ident(&mut self, #ident: #param_type) -> &mut Self
	};

	let block = match kind {
		SetterKind::Each => {
			let field_ident = f.ident.as_ref().unwrap();
			quote::quote! {
					if let std::option::Option::Some(ref mut field) = self.#field_ident {
						field.push(#ident);
					} else {
						self.#field_ident = std::option::Option::Some(vec![#ident]);
					}

					self
			}
		},
		SetterKind::Straight => quote::quote! {
			self.#ident=std::option::Option::Some(#ident);
			self
		},
	};

	Ok(quote::quote! {
		#signature {
			#block
		}
	},)
}

fn build_impl(store: &Store,) -> proc_macro2::TokenStream {
	let Store { ident, builder_ident, optional_fields, naked_fields, .. } = &store;
	let head = store.generics_template(builder_ident,);
	let optional_fields_ident: Vec<Option<&syn::Ident,>,> =
		optional_fields.iter().map(crate::ident,).collect();
	let naked_fields_ident: Vec<Option<&syn::Ident,>,> =
		naked_fields.iter().map(crate::ident,).collect();

	quote::quote! {
		impl #head {
			pub fn build(&mut self) -> std::result::Result<#ident, std::boxed::Box<dyn std::error::Error>> {
				#(
					let #optional_fields_ident = self.#optional_fields_ident.clone();
				)*
				#(
					let #naked_fields_ident = self.#naked_fields_ident.clone().unwrap_or_default();
				)*

				std::result::Result::Ok(
					#ident {
						#(
							#optional_fields_ident,
						)*
						#(
							#naked_fields_ident,
						)*
					}
				)
			}
		}
	}
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream,) -> TokenStream {
	let input = &syn::parse_macro_input!(input as syn::DeriveInput);
	let store = Store::new(input,);

	let impl_target = target_impl(&store,);
	let decl_builder = builder_decl(&store,);
	let impl_setter = match setter_impl(&store,) {
		Ok(ts,) => ts,
		Err(e,) => return e.to_compile_error().into(),
	};
	let impl_build = build_impl(&store,);

	quote::quote! {
		#impl_target
		#decl_builder
		#impl_setter
		#impl_build
	}
	.into()
}
