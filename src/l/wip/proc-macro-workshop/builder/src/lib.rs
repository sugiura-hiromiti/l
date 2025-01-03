use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::Span;

enum DeriveTo {
	Struct,
	Enum,
	Union,
}

///  TODO:
///  1. support unnamed member
///  2. support enum/unit
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream,) -> TokenStream {
	let mut target = syn::parse_macro_input!(input as syn::DeriveInput);
	let kind_flag = target_kind(&target.data,);

	let ident = target.ident;
	let (builder, builder_type,) = builder_idents(ident.clone(),);
	let fields = fields(&mut target.data,);

	// builder struct
	let field_type = fields.iter_mut().map(|f| {
		if let Some(ref _f,) = is_option(f,) {
			// do nothing
		} else {
			wrap_with_option(f,);
		}
		field_ty(&*f,)
	},);
	let fields_iterable = fields.iter().clone();
	let builder_def = quote::quote! {
		#[derive(Clone)]
		pub struct #builder_type {
			#(#fields_iterable,)*
		}
	};

	// implement builder() to target struct
	let field_init = fields.iter().map(field_init,);
	let implement = quote::quote! {
		impl #ident {
			pub fn #builder() -> #builder_type {
				#builder_type {
					#(#field_init,)*
				}
			}
		}
	};

	// implement builder struct
	let mut mem_cl = fields.clone();
	let field_ty_unwrap = mem_cl.iter_mut().map(|f| {
		let f = unwrap_option(f,);
		field_ty(&*f,)
	},);
	let fi_cl = fields.iter().map(field_ident,);
	let builder_impl = quote::quote! {
		impl #builder_type {
			#(
				pub fn #fi_cl(&mut self, #fi_cl: #field_ty_unwrap) -> &mut Self {
					self.#fi_cl=Some(#fi_cl);
					self
				}
			)*
		}
	};

	// implement build() to builder struct
	let originally_option: Vec<syn::Ident,> =
		fields.iter().filter_map(is_option,).map(|f| f.ident.unwrap(),).collect();
	let not_option_at_first: Vec<syn::Ident,> = fields
		.iter()
		.filter_map(|f| match is_option(f,) {
			Some(_,) => None,
			None => Some(f.clone(),),
		},)
		.map(|f| f.ident.unwrap(),)
		.collect();
	let noo_str = not_option_at_first.clone().into_iter().map(|ident| ident.to_string(),);
	let build = quote::quote! {
		impl #builder_type {
			pub fn build(&self) -> Result<#ident, Box<dyn std::error::Error>> {
				let me =self.clone();

				#(
					 let #not_option_at_first = me.#not_option_at_first.ok_or(format!("field {} have to be set explicitly",#noo_str))?;
				)*
				#(
					 let #originally_option = me.#originally_option;
				)*

				Ok(
					#ident {
						#(#originally_option,)*
						#(#not_option_at_first,)*
					}
				)
			}
		}
	};

	quote::quote! {
		#implement
		#builder_def
		#builder_impl
		#build
	}
	.into()
}

fn target_kind(data: &syn::Data,) -> DeriveTo {
	match data {
		syn::Data::Struct(_data_struct,) => DeriveTo::Struct,
		syn::Data::Enum(_data_enum,) => DeriveTo::Enum,
		syn::Data::Union(_data_union,) => DeriveTo::Union,
	}
}

fn builder_idents(ident: syn::Ident,) -> (syn::Ident, syn::Ident,) {
	let builder = syn::Ident::new("builder", Span::call_site(),);
	let builder_type = syn::Ident::new(&format!("{}Builder", ident), Span::call_site(),);
	(builder, builder_type,)
}

fn fields(data: &mut syn::Data,) -> &mut syn::Fields {
	match data {
		syn::Data::Struct(data_struct,) => &mut data_struct.fields,
		_ => {
			unreachable!(
				"expected struct, found enum/union. for enum, use extract_variant instead. for \
				 union, use extract_fields instead"
			)
		},
	}
}

fn builder_decl(name: syn::Ident, fields: syn::Fields,) {}

// fn filter_tokens<S: quote::ToTokens, T: quote::ToTokens + IntoIterator,>(
// 	stream: T,
// 	call: impl Fn(T::Item,) -> S,
// ) {
// 	stream.into_iter()
// }

fn wrap_with_option(f: &mut syn::Field,) -> &mut syn::Field {
	let ty = f.ty.clone();
	f.ty = syn::parse_quote! {
		Option<#ty>
	};

	f
}

fn unwrap_option(f: &mut syn::Field,) -> &mut syn::Field {
	match f.ty {
		syn::Type::Path(syn::TypePath { ref path, .. },) => {
			let last = path.segments.last().unwrap();

			if last.ident != "Option" {
				return f;
			}

			match &last.arguments {
				syn::PathArguments::AngleBracketed(args,) => {
					let ty = args.args.first().unwrap();
					match ty {
						syn::GenericArgument::Type(ty,) => {
							f.ty = ty.clone();
						},
						_ => unreachable!("~~~~~~~~~~~~~~~~~~~~"),
					}
				},
				_ => unreachable!("type of field did not wrapped"),
			}
		},
		_ => unreachable!("type of field did not wrapped"),
	}

	f
}

fn field_init(f: &syn::Field,) -> proc_macro2::TokenStream {
	let ident = f.ident.clone().unwrap();
	let default = default(f,);
	quote::quote! { #ident: #default }
}

fn default(f: &syn::Field,) -> syn::Expr {
	let ty = f.ty.clone();
	syn::parse_quote! {
		<#ty>::default()
	}
}

fn field_ty(f: &syn::Field,) -> syn::Type {
	f.ty.clone()
}

fn field_ident(f: &syn::Field,) -> syn::Ident {
	f.ident.clone().unwrap()
}

fn is_option(f: &syn::Field,) -> Option<syn::Field,> {
	match f.ty {
		syn::Type::Path(syn::TypePath { ref path, .. },) => {
			let last = path.segments.last().unwrap();

			if last.ident == "Option" {
				Some(f.clone(),)
			} else {
				None
			}
		},
		_ => None,
	}
}

#[cfg(test)]
mod tests {

	//use super::*;
}
