use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenTree;
use syn::parse::Parse;
use syn::parse_macro_input;

extern crate proc_macro;

#[proc_macro_attribute]
pub fn empty_trait_impl_block(attr: TokenStream, item: TokenStream,) -> TokenStream {
	let types = parse_macro_input!(attr as Types);
	let trait_def = parse_macro_input!(item as syn::ItemTrait);
	let trait_ident = &trait_def.ident;
	let trait_generics = &trait_def.generics;
	let trait_unsafety = trait_def.unsafety;

	let impls = types.iter().map(|ty| {
		quote::quote! {
			#trait_unsafety impl #trait_generics #trait_ident #trait_generics for #ty {}
		}
	},);

	quote::quote! {
		#trait_def

		#(#impls)*
	}
	.into()
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

#[proc_macro]
pub fn bench_for_all_integers(item: TokenStream,) -> TokenStream {
	let types = parse_macro_input!(item as Types);
	let fn_defs = types.iter().map(|ty| {
		let syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. },) = ty else {
			unreachable!()
		};

		let ident = segments.first().unwrap().ident.clone();
		let ref_fn_ident =
			syn::Ident::new(&format!("pass_ref_{}", ident.to_string()), Span::call_site(),);
		let fn_ident =
			syn::Ident::new(&format!("pass_copy_{}", ident.to_string()), Span::call_site(),);
		quote::quote! {
			#[bench]
			fn #ref_fn_ident(b: &mut Bencher,) {
				b.iter(|| {
					for i in 0..(N as #ty) {
						take_ref(black_box(&i,),);
					}
				},);
			}

			#[bench]
			fn #fn_ident(b: &mut Bencher,) {
				b.iter(|| {
					for i in 0..(N as #ty) {
						take_copy(black_box(i,),);
					}
				},);
			}
		}
	},);

	quote::quote! {
		#(#fn_defs)*
	}
	.into()
}
