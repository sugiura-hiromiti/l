use crate::store;
use syn::TraitBound;
use syn::TypeParam;
use syn::TypeParamBound;

// pub fn extract_field_ident_and_lit(f: &Field,) -> (&Ident, String,) {
// 	let ident = field_ident(f,);
// 	let lit = ident.to_string();
// 	(ident, lit,)
// }

// pub fn field_ident_and_mod<F,>(
// 	f: &Field,
// ) -> Result<(&Ident, impl FnOnce(&mut fmt::Formatter,) -> Result<(), fmt::Error,>,), syn::Error,>
// { 	let attr = f.attrs.iter().map(store::Attr::try_from,).map(|a| Ok(a?.value,),).try_fold(
// 		vec![],
// 		|mut acc, at: Result<String, syn::Error,>| {
// 			acc.push(at?,);
// 			Ok::<Vec<String,>, syn::Error,>(acc,)
// 		},
// 	)?;
// 	Ok((field_ident(f,), move |fmtr: &mut fmt::Formatter| {
// 		fmtr.pad("0",)?;
// 		attr.iter().map(|a| fmtr.write_fmt(format_args!(""),),);
// 		Ok((),)
// 	},),)
// }

// pub fn field_ident(f: &Field,) -> &Ident {
// 	f.ident.as_ref().expect("this derive macro does not support tuple style struct",)
// }

// pub fn field_attr(f: &Field,) -> &Attribute {
// 	todo!()
// }

// pub fn fields_detail(fields: &Vec<&Field,>,) -> Result<store::FieldsDetail, syn::Error,> {
// 	let fields = fields.iter().map(|f| store::FieldDetail::try_from(*f,),).try_fold(
// 		vec![],
// 		|mut acc, fd| -> Result<Vec<store::FieldDetail,>, syn::Error,> {
// 			acc.push(fd?,);
// 			Ok(acc,)
// 		},
// 	)?;
//
// 	Ok(store::FieldsDetail { fields, },)
// }

pub fn join_modifiers(attrs: &Vec<store::Attr,>,) -> String {
	attrs.iter().filter(|a| a.kind == store::AttrKind::Debug,).fold(
		"".to_string(),
		|mut acc, attr| {
			acc.push_str(&attr.value,);
			acc
		},
	)
}

// pub fn literalize(ident: &Ident, ty: &Type,) -> String {
// 	if should_double_quoted(ty,) {
// 		format!("\"{}\"", ident.to_string(),)
// 	} else {
// 		ident.to_string()
// 	}
// }

//  TODO: required to resolve types at runtime due to support generics
// pub fn should_double_quoted(ty: &Type,) -> bool {
// 	match ty {
// 		Type::Path(TypePath { path, .. },) => {
// 			path.segments.last().map(|seg| seg.ident == "String",).unwrap_or(false,)
// 		},
// 		Type::Reference(TypeReference { elem, .. },) => {
// 			if let Type::Path(TypePath { path, .. },) = &**elem {
// 				path.segments.last().map(|seg| seg.ident == "str",).unwrap_or(false,)
// 			} else {
// 				false
// 			}
// 		},
// 		_ => false,
// 	}
// }

pub fn restrict_printable(tp: &mut TypeParam,) -> syn::Result<(),> {
	let printable_bound: TraitBound = syn::parse_str("std::fmt::Display",)?;
	tp.bounds.push(TypeParamBound::Trait(printable_bound,),);
	Ok((),)
}

// pub fn select_modifier(attrs: Vec<&Vec<store::Attr,>,>,) -> Vec<String,> {
// 	attrs.into_iter().map(join_modifiers,).collect()
// }

// fn primitive_numeral_types() -> Vec<String,> {
// 	let posts_abbr = vec!["8", "128", "usize"];
// 	let max_size = posts_abbr[1].parse::<u32>().expect("failed to parse String into i32",);
// 	let min_size = posts_abbr[0].parse::<u32>().expect("failed to parse String into i32",);
// 	let variety = max_size / min_size;
//
// 	let mut posts: Vec<String,> = (1..=variety).map(|u| (u * 8).to_string(),).collect();
// 	posts.push(posts_abbr[2].to_string(),);
// 	let pres = vec!["u", "i"];
//
// 	let dbg_repr = "0";
// 	let type_name = std::any::type_name_of_val(dbg_repr,);
// 	if type_name == std::any::type_name::<&str,>() || type_name == std::any::type_name::<String,>()
// 	{
// 		// no need to convert type
// 	} else {
// 		dbg_repr
// 			.parse::<i32>()
// 			.expect(&format!("failed to parse String into {}", std::any::type_name::<i32,>()),);
// 		u8::from_str_radix(dbg_repr, 2,)
// 			.expect(&format!("failed to parse String into {}", std::any::type_name::<i32,>()),);
// 	}
//
// 	pres.iter().flat_map(|pre| posts.iter().map(|post| pre.to_string() + post,),).collect()
// }
