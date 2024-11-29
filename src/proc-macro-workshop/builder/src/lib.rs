use proc_macro::TokenStream;

struct Store {
	ident:           syn::Ident,
	builder_ident:   syn::Ident,
	generics:        syn::Generics,
	optional_fields: Vec<syn::Field,>,
	naked_fields:    Vec<syn::Field,>,
}

impl Store {
	fn new(input: TokenStream,) -> Self {
		let input = &syn::parse_macro_input!(input as syn::DeriveInput);

		let ident = original_name(input,);
		let builder_ident = builder_name(&ident,);
		let generics = generics(input,);

		let mut fields = fields(input,).into_iter().collect();
		let optional_fields = optional_fields(&mut fields,);
		let naked_fields = fields;

		Self { ident, builder_ident, generics, optional_fields, naked_fields, }
	}
}

fn original_name(input: &syn::DeriveInput,) -> syn::Ident {
	input.ident.clone()
}

fn builder_name(ident: &syn::Ident,) -> syn::Ident {
	quote::format_ident!("{ident}Builder")
}

fn optional_fields(field_vec: &mut Vec<syn::Field,>,) -> Vec<syn::Field,> {
	let mut optional_fields = vec![];

	let mut i = 0;
	while i < field_vec.len() {
		if type_is_option(&field_vec[i],) {
			// if type of a field is `Option`, add to `optional_fields`
			// And remove that element from `field_vec`
			// for having `field_vec` representing fields which type is naked
			let opt_f = field_vec.remove(i,);
			optional_fields.push(opt_f,);
		} else {
			// if type of a field is not `Option`,
			i += 1;
		}
	}

	optional_fields
}

fn fields(input: &syn::DeriveInput,) -> syn::Fields {
	match &input.data {
		syn::Data::Struct(data_struct,) => data_struct.fields.clone(),
		syn::Data::Enum(_data_enum,) => todo!(),
		syn::Data::Union(_data_union,) => todo!(),
	}
}

fn type_is_option(field: &syn::Field,) -> bool {
	let syn::Type::Path(syn::TypePath { ref path, .. },) = field.ty else {
		return false;
	};
	let type_name = path.segments.last().unwrap();

	type_name.ident == "Option"
}

fn generics(input: &syn::DeriveInput,) -> syn::Generics {
	input.generics.clone()
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream,) -> TokenStream {
	let store = Store::new(input,);

	let decl = quote::quote! {};

	unimplemented!()
}
