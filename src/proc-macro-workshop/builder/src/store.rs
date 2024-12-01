use crate::extract;

pub struct Store {
	pub ident:           syn::Ident,
	pub builder_ident:   syn::Ident,
	pub generics:        syn::Generics,
	pub optional_fields: Vec<syn::Field,>,
	pub naked_fields:    Vec<syn::Field,>,
}

impl Store {
	pub fn new(input: &syn::DeriveInput,) -> Self {
		let ident = extract::original_name(input,);
		let builder_ident = extract::builder_name(&ident,);
		let generics = extract::generics(input,);

		let mut fields: Vec<syn::Field,> = extract::fields(input,).into_iter().collect();
		let optional_fields = extract::optional_fields(&mut fields,);
		let naked_fields = fields.into_iter().map(extract::optionalize,).collect();

		Self { ident, builder_ident, generics, optional_fields, naked_fields, }
	}

	pub fn generics_template(&self, ident: &syn::Ident,) -> proc_macro2::TokenStream {
		let (impl_gnrcs, ty_gnrcs, where_clause,) = self.generics.split_for_impl();

		quote::quote! {
			#impl_gnrcs #ident #ty_gnrcs #where_clause
		}
	}

	pub fn fields(&self,) -> impl Iterator<Item = &syn::Field,> + use<'_,> {
		self.optional_fields.iter().chain(&self.naked_fields,).map(|f| f,)
	}

	pub fn members(&self,) -> impl Iterator<Item = Option<&syn::Ident,>,> + use<'_,> {
		let members = self.fields().map(|f| f.ident.as_ref(),);
		members
	}
}

pub struct Setter {
	pub kind:       SetterKind,
	pub ident:      syn::Ident,
	/// currently, only one parameter is required
	pub param_type: syn::Type,
}

impl Setter {
	pub fn new(f: &syn::Field,) -> Result<Self, syn::Error,> {
		let field = &extract::unwrap(f.clone(),);
		match parse_attr(&field.attrs,) {
			SetterKind::Straight => Ok(Self::straight(field,),),
			SetterKind::Each => Self::with_builder_attr(field,),
		}
	}

	fn straight(f: &syn::Field,) -> Self {
		let ident = f.ident.clone().unwrap();
		Self { kind: SetterKind::Straight, ident, param_type: f.ty.clone(), }
	}

	fn with_builder_attr(f: &syn::Field,) -> Result<Self, syn::Error,> {
		let attr = &f.attrs[0];
		let syn::Expr::Assign(syn::ExprAssign { left, right, .. },) =
			attr.parse_args().expect("failed to parse args of builder attribute",)
		else {
			panic!(
				"builder attribute accepts key-value pair in order to specify method type & name",
			)
		};

		let left: syn::Ident = syn::parse_quote! { #left };
		let kind = SetterKind::try_from(left,)?;

		let right: syn::LitStr = syn::parse_quote! { #right };
		let ident = quote::format_ident!("{}", right.value());

		let param_type = extract::unwrap(f.clone(),).ty;
		Ok(Self { kind, ident, param_type, },)
	}
}

#[derive(PartialEq, Eq, Debug,)]
pub enum SetterKind {
	Straight,
	Each,
}

impl TryFrom<syn::Ident,> for SetterKind {
	type Error = syn::Error;

	fn try_from(value: syn::Ident,) -> Result<Self, Self::Error,> {
		match value.to_string().as_str() {
			"each" => Ok(SetterKind::Each,),
			_ => Err(syn::Error::new(value.span(), "expected `builder(each = \"...\")`",),),
		}
	}
}

pub fn parse_attr(attrs: &Vec<syn::Attribute,>,) -> SetterKind {
	if attrs.len() == 0 {
		return SetterKind::Straight;
	}

	attrs
		.iter()
		.find_map(|attr| {
			if let syn::Expr::Assign(_,) =
				attr.parse_args().expect("failed to parse args of builder attribute",)
			{
				Some(SetterKind::Each,)
			} else {
				None
			}
		},)
		.unwrap_or(SetterKind::Straight,)
}
