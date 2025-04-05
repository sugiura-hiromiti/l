// use anyhow::anyhow;
// use anyhow::Error as Eror;
//use anyhow::Result as Rslt;
//use std::collections::HashMap;
use syn::Attribute;
use syn::Expr;
use syn::ExprLit;
use syn::Field;
use syn::Generics;
use syn::Ident;
use syn::Lit;
use syn::Meta;
use syn::MetaList;
use syn::MetaNameValue;
use syn::Path;
use syn::Type;
use syn::spanned::Spanned;

use crate::converter;

pub struct SyntaxTreePool {
	pub fields:   Vec<Field,>,
	pub ident:    Ident,
	pub generics: Generics,
}

impl SyntaxTreePool {
	pub fn new(input: syn::DeriveInput,) -> Self {
		let fields = match input.data {
			syn::Data::Struct(data_struct,) => data_struct.fields,
			syn::Data::Enum(_data_enum,) => todo!(),
			syn::Data::Union(_data_union,) => todo!(),
		};

		let fields: Vec<Field,> = fields.into_iter().collect();
		let mut attributed_fields = vec![];
		let mut raw_fields = vec![];
		fields.iter().for_each(|f| {
			if f.attrs.len() == 0 {
				raw_fields.push(f,);
			} else {
				attributed_fields.push(f,);
			}
		},);

		Self { fields, ident: input.ident, generics: input.generics, }
	}

	pub fn adjust_generics(&self,) -> syn::Result<Generics,> {
		let mut generics = self.generics.clone();
		generics.type_params_mut().try_for_each(converter::restrict_printable,)?;
		Ok(generics,)
	}

	// pub fn naked_fields(&self,) -> Vec<&Field,> {
	// 	self.filter_fields(|f| f.attrs.is_empty(),)
	// }
	//
	// pub fn attred_fields(&self,) -> Vec<&Field,> {
	// 	self.filter_fields(|f| !f.attrs.is_empty(),)
	// }
	//
	// fn filter_fields(&self, pred: impl Fn(&&Field,) -> bool,) -> Vec<&Field,> {
	// 	self.fields.iter().filter(pred,).collect()
	// }
}

pub struct FieldsDetail {
	pub fields: Vec<FieldDetail,>,
}

impl TryFrom<Vec<Field,>,> for FieldsDetail {
	type Error = syn::Error;

	fn try_from(value: Vec<Field,>,) -> Result<FieldsDetail, syn::Error,> {
		let fields =
			value.iter().map(|f| FieldDetail::try_from(f,),).try_fold(vec![], |mut acc, fd| {
				acc.push(fd?,);
				Ok::<_, syn::Error,>(acc,)
			},)?;
		Ok(Self { fields, },)
	}
}

impl FieldsDetail {
	pub fn fold(&self,) -> (Vec<&Vec<Attr,>,>, Vec<&Ident,>, Vec<&String,>, Vec<&Type,>,) {
		self.fields.iter().fold((vec![], vec![], vec![], vec![],), |mut acc, fd| {
			acc.0.push(&fd.attrs,);
			acc.1.push(&fd.ident,);
			acc.2.push(&fd.ident_lit,);
			acc.3.push(&fd.ty,);
			acc
		},)
	}
}

pub struct FieldDetail {
	attrs:     Vec<Attr,>,
	ident:     Ident,
	ident_lit: String,
	ty:        Type,
}

impl TryFrom<&Field,> for FieldDetail {
	type Error = syn::Error;

	fn try_from(value: &Field,) -> Result<Self, Self::Error,> {
		let attrs = value.attrs.iter().filter_map(|a| Attr::try_from(a,).ok(),).collect();
		let ident = value.ident.clone().expect("Non named field would not need method",);
		let ident_lit = ident.to_string();
		let ty = value.ty.clone();
		Ok(Self { attrs, ident, ident_lit, ty, },)
	}
}

// impl FieldDetail {
// 	pub fn has_attr(&self,) -> bool {
// 		!self.attrs.is_empty()
// 	}
// }

pub struct Attr {
	pub kind:  AttrKind,
	pub value: String,
}

impl TryFrom<&Attribute,> for Attr {
	type Error = syn::Error;

	fn try_from(value: &Attribute,) -> Result<Self, Self::Error,> {
		match &value.meta {
			Meta::Path(ref path,) => Err(Self::Error::new(
				path.span(),
				format!(
					"Path style attribute is not supported now: {}",
					path.require_ident()?.to_string()
				),
			),),
			Meta::List(meta_list,) => {
				let kind = match AttrKind::try_from(&meta_list.path,) {
					Ok(ak,) => ak,
					Err(e,) => return Err(Self::Error::new(meta_list.span(), e.to_string(),),),
				};

				let value = parse_arg_value(meta_list,)?;

				Ok(Attr { kind, value, },)
			},
			Meta::NameValue(MetaNameValue { path, value, .. },) => {
				let kind = AttrKind::try_from(path,)?;
				let Expr::Lit(ExprLit { lit, .. },) = value else {
					return Err(syn::Error::new(value.span(), "please give literal expression",),);
				};
				let Lit::Str(ls,) = lit else {
					return Err(syn::Error::new(lit.span(), "expected string literal",),);
				};
				let value = ls.value();

				Ok(Attr { kind, value, },)
			},
		}
	}
}

fn parse_arg_value(ml: &MetaList,) -> Result<String, syn::Error,> {
	let Expr::Lit(ExprLit { lit, .. },) = ml.parse_args()? else {
		return Err(syn::Error::new(ml.span(), "failed to parse value",),);
	};
	let Lit::Str(s,) = lit else {
		return Err(syn::Error::new(lit.span(), "value must be string literal",),);
	};
	Ok(s.value(),)
}

#[derive(PartialEq, Eq,)]
pub enum AttrKind {
	Debug,
}

impl TryFrom<&Path,> for AttrKind {
	type Error = syn::Error;

	fn try_from(value: &Path,) -> Result<Self, Self::Error,> {
		let ident = value.segments.last().unwrap().ident.to_string();
		match ident.as_str() {
			"debug" => Ok(Self::Debug,),
			_ => Err(syn::Error::new(value.span(), format!("no attribute named {ident} found"),),),
		}
	}
}

impl From<AttrKind,> for &str {
	fn from(value: AttrKind,) -> Self {
		match value {
			AttrKind::Debug => "debug",
		}
	}
}
