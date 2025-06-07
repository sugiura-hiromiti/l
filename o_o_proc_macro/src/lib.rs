extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenTree;
use syn::parse::Parse;
use syn::parse_macro_input;

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
		let idnt_builder = |attr: &str| {
			syn::Ident::new(&format!("pass_{}_{}", attr, ident.to_string()), Span::call_site(),)
		};

		let fn_ident = idnt_builder("copy",);
		let ref_fn_ident = idnt_builder("ref",);
		let mut_ident = idnt_builder("mut",);
		let mut_ref_ident = idnt_builder("mut_ref",);
		quote::quote! {
			#[bench]
			fn #mut_ref_ident(b: &mut Bencher,) {
				b.iter(|| {
					for mut i in 0..(N as #ty) {
						take_mut_ref(black_box(&mut i,),);
					}
				},);
			}

			#[bench]
			fn #mut_ident(b: &mut Bencher,) {
				b.iter(|| {
					for i in 0..(N as #ty) {
						take_mut_copy(black_box(i,),);
					}
				},);
			}

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

const ARGS_COUNT: usize = 10;

#[proc_macro]
pub fn bench_fn_call_with_more_than_8_args(item: TokenStream,) -> TokenStream {
	let item2 = item.clone();
	let arg_ty = parse_macro_input!(item as syn::Type);
	let arg_ident = parse_macro_input!(item2 as syn::Ident);
	let implements = args_benchers(arg_ty, arg_ident,);
	quote::quote! {
		#(#implements)*
	}
	.into()
}

fn args_benchers(arg_ty: syn::Type, arg_ident: syn::Ident,) -> Vec<proc_macro2::TokenStream,> {
	(1..=ARGS_COUNT)
		.map(|i| {
			let params: Vec<_,> = (1..=i)
				.map(|j| syn::Ident::new(&format!("param{j}"), Span::call_site(),),)
				.collect();
			let fn_ident_base = format!("take_{i}_{}_args", arg_ident);
			let fn_ident = syn::Ident::new(&fn_ident_base, Span::call_site(),);
			let bencher_ident =
				syn::Ident::new(&format!("bench_{fn_ident_base}"), Span::call_site(),);

			let helper = quote::quote! {
				fn #fn_ident(#(#params: #arg_ty,)*) {}
			};

			let args = vec![syn::Ident::new("i", Span::call_site(),); i];
			let bencher = quote::quote! {
				#[bench]
				fn #bencher_ident(b: &mut Bencher){
					b.iter(|| {
						for i in 0..(N as #arg_ty) {
							#fn_ident(#(black_box(#args),)*);
						}
					})
				}
			};

			quote::quote! {
				#helper
				#bencher
			}
		},)
		.collect()
}
