pub fn original_name(input: &syn::DeriveInput,) -> syn::Ident {
	input.ident.clone()
}

pub fn builder_name(ident: &syn::Ident,) -> syn::Ident {
	quote::format_ident!("{ident}Builder")
}

pub fn generics(input: &syn::DeriveInput,) -> syn::Generics {
	input.generics.clone()
}

pub fn fields(input: &syn::DeriveInput,) -> syn::Fields {
	match &input.data {
		syn::Data::Struct(data_struct,) => data_struct.fields.clone(),
		syn::Data::Enum(_data_enum,) => todo!(),
		syn::Data::Union(_data_union,) => todo!(),
	}
}

pub fn optional_fields(field_vec: &mut Vec<syn::Field,>,) -> Vec<syn::Field,> {
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

fn type_is_option(field: &syn::Field,) -> bool {
	let type_name = explain_type_path(&field.ty,);
	type_name.ident == "Option"
}

pub fn optionalize(mut field: syn::Field,) -> syn::Field {
	let ty = field.ty;
	field.ty = syn::parse_quote! {
		std::option::Option<#ty>
	};

	field
}

pub fn unwrap(field: syn::Field,) -> syn::Field {
	let type_name = explain_type_path(&field.ty,);
	let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
		ref args, ..
	},) = type_name.arguments
	else {
		panic!("Given field is not wrapped");
	};

	let syn::GenericArgument::Type(ty,) = args.last().unwrap() else {
		panic!("failed to find type in generic arguments");
	};
	//let ty: syn::Type = syn::parse_quote! {#args};
	let mut field = field.clone();
	field.ty = ty.clone();
	field
}

fn explain_type_path(ty: &syn::Type,) -> &syn::PathSegment {
	let syn::Type::Path(syn::TypePath { ref path, .. },) = ty else {
		panic!("type kind must be path representation");
	};

	path.segments.last().unwrap()
}

pub fn ident(field: &syn::Field,) -> Option<&syn::Ident,> {
	field.ident.as_ref()
}

fn ty(field: &syn::Field,) -> &syn::Type {
	&field.ty
}

pub fn ident_and_ty(field: &syn::Field,) -> (&syn::Ident, &syn::Type,) {
	(ident(field,).unwrap(), ty(field,),)
}

// pub fn attr_valued_by(v: impl AsRef<str,>,) -> impl Fn(&syn::Field,) -> Option<&syn::Expr,> {
// 	move |syn::Field { attrs, .. }| {}
// }
