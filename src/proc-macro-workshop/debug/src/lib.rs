mod converter;
mod store;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS;
use quote::quote;
use store::Attr;
use store::FieldsDetail;
use syn::Ident;

fn impl_derive(stp: &store::SyntaxTreePool,) -> Result<TS, syn::Error,> {
	let ident = &stp.ident;
	let ident_lit = ident.to_string();

	let head_impl = impl_head(stp, ident,)?;
	// let field_chain = chain_field(stp,)?;
	// let attred_fields = chain_field_attred(stp,)?;
	let fields_set = set_fields(stp,)?;

	Ok(quote! {
		#head_impl {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				// let dbg_struct = f.debug_struct(#ident_lit)
				// 	#field_chain;
				// #attred_fields
				//
				// dbg_struct.finish()?;
				f.write_str(&format!("{} {{ ",#ident_lit))?;
				#(#fields_set)*
				f.write_str("}")?;
				Ok(())
			}
		}
	},)
}

fn impl_head(stp: &store::SyntaxTreePool, ident: &Ident,) -> syn::Result<TS,> {
	let gnrcs = stp.adjust_generics()?;
	let (impl_gnrcs, ty_gnrcs, where_clause,) = &gnrcs.split_for_impl();

	Ok(quote! {
		impl #impl_gnrcs std::fmt::Debug for #ident #ty_gnrcs #where_clause
	},)
}

fn set_fields(stp: &store::SyntaxTreePool,) -> Result<Vec<TS,>, syn::Error,> {
	let binding = FieldsDetail::try_from(stp.fields.clone(),)?;
	let (attrs, idents, ident_lits, types,) = binding.fold();
	let len = attrs.len();

	if len == 0 {
		return Ok(vec![],);
	}

	let mut ts_vec: Vec<TS,> = (0..attrs.len() - 1)
		.map(|i| {
			let (attr, ident, _ident_lit, _ty,) = (attrs[i], idents[i], ident_lits[i], types[i],);
			set_field(attr, ident, false,)
		},)
		.collect();

	ts_vec.push(set_field(attrs[len - 1], idents[len - 1], true,),);
	Ok(ts_vec,)
}

fn set_field(attr: &Vec<Attr,>, ident: &Ident, last: bool,) -> TS {
	let mut attr_lit = converter::join_modifiers(attr,);
	//let ident_lit = literalize(ident, ty,);
	let ident_lit = ident.to_string();
	let value = gen_value_ts(&mut attr_lit, ident, last,);

	quote! {
		f.write_fmt(format_args!("{}: {}", #ident_lit, #value))?;
	}
}

fn gen_value_ts(attr_lit: &mut String, ident: &Ident, last: bool,) -> TS {
	if attr_lit.is_empty() {
		attr_lit.push_str("{}",);
	}

	let placeholders = format!("{{}}{attr_lit}{{}}{{}} ");

	quote! {
		{
			let type_name = std::any::type_name_of_val(&self.#ident);
			let (pre, post) = if type_name == "String" || type_name == "&str" {
				("\"", "\"")
			} else {
				("", "")
			};
			let trailling_comma = if !#last {
				","
			} else {
				""
			};

			format!(#placeholders, pre, &self.#ident, post, trailling_comma)
		}
	}
}

// fn set_naked_field(ident_lit: &String, value: TS,) -> TS {
// 	quote! {
// 		f.write_fmt(format_args!("{}: {}", #ident_lit, #value))?;
// 	}
// }
//
// fn set_attred_field(attr_lit: &String, ident_lit: &String, value: TS,) -> TS {
// 	quote! {
// 		let dbg_repr = format!(#attr_lit, #value);
// 		f.write_str(&format!("{}: {} ", #ident_lit, dbg_repr))?;
// 	}
// }

// fn style_fields_attred(
// 	attrs: Vec<&Vec<Attr,>,>,
// 	idents: Vec<&Ident,>,
// 	ident_lits: Vec<&String,>,
// ) -> Vec<TS,> {
// 	let form_mod = converter::select_modifier(attrs,);
// 	idents
// 		.into_iter()
// 		.zip(form_mod.into_iter(),)
// 		.zip(ident_lits.into_iter(),)
// 		.map(|((id, fm,), idl,)| {
// 			quote! {
// 				let dbg_repr = format!(#fm, &self.#id);
// 				f.write_str(&format!("{}: {}", #idl, dbg_repr))?;
// 			}
// 		},)
// 		.collect()
// }

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream,) -> TokenStream {
	let derived_input = syn::parse_macro_input!(input as syn::DeriveInput);
	let store = store::SyntaxTreePool::new(derived_input,);

	let derive_impl = match impl_derive(&store,) {
		Ok(ts,) => ts,
		Err(e,) => return e.to_compile_error().into(),
	};
	quote! {
		#derive_impl
	}
	.into()
}
