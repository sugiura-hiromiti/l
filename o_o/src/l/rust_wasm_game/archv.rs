fn drw_tri(
	context: &CanvasRenderingContext2d,
	color: (u8, u8, u8,),
	a: (f64, f64,),
	b: (f64, f64,),
	c: (f64, f64,),
) {
	context.set_fill_style(&wasm_bindgen::JsValue::from_str(&format!(
		"rgb({},{},{})",
		color.0, color.1, color.2
	),),);
	context.move_to(a.0, a.1,);
	context.begin_path();
	context.line_to(b.0, b.1,);
	context.line_to(c.0, c.1,);
	context.line_to(a.0, a.1,);
	context.close_path();
	context.stroke();
	context.fill();
}

fn spsk(
	context: CanvasRenderingContext2d,
	color: (u8, u8, u8,),
	a: (f64, f64,),
	b: (f64, f64,),
	c: (f64, f64,),
	mut depth: usize,
) {
	if depth != 0 {
		depth -= 1;
		drw_tri(&context, color, a, b, c,);

		let rng = 0..255;
		let generate = |range: Range<i32,>| -> (u8, u8, u8,) {
			let mut rng = thread_rng();
			(
				rng.gen_range(range.clone(),).try_into().unwrap(),
				rng.gen_range(range.clone(),).try_into().unwrap(),
				rng.gen_range(range,).try_into().unwrap(),
			)
		};
		let n_color = generate(rng,);
		let n_a = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0,);
		let n_b = ((c.0 + b.0) / 2.0, (c.1 + b.1) / 2.0,);
		let n_c = ((c.0 + a.0) / 2.0, (c.1 + a.1) / 2.0,);

		spsk(context.clone(), n_color, a, n_a, n_c, depth,);
		spsk(context.clone(), n_color, n_a, b, n_b, depth,);
		spsk(context, n_color, n_b, c, n_c, depth,);
	}
}
