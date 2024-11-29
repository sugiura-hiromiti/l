use anyhow::anyhow;
use anyhow::Result as Rslt;
use std::cell::UnsafeCell;
use std::sync::Mutex;
use std::sync::OnceLock;
use tokio::runtime::Runtime as TokioRuntime;
use wasi_common::snapshots::preview_1::wasi_snapshot_preview1;
use wasi_common::sync::WasiCtxBuilder;
use wasi_common::WasiCtx;

static WASI_CTX: OnceLock<Mutex<WasiCtx,>,> = OnceLock::new();

pub fn wasm<'a,>(s: impl AsRef<str,>,) -> Rslt<(),> {
	let mut isolate = v8::Isolate::new(Default::default(),);
	let scope = &mut v8::HandleScope::new(&mut isolate,);
	let cntxt = v8::Context::new(scope, Default::default(),);
	let scope = &mut v8::ContextScope::new(scope, cntxt,);

	let wasm_module = wat::parse_str(s,)?;
	let module =
		v8::WasmModuleObject::compile(scope, &wasm_module,).ok_or(anyhow!("failed to compile"),)?;

	let import_object = import_fd_write(scope, fd_write,);
	let instance = wasm_instance(scope, module, import_object,);

	run(scope, instance,);

	Ok((),)
}

fn wasi_ctx_mut() -> &'static Mutex<WasiCtx,> {
	WASI_CTX.get_or_init(|| {
		let mut builder = WasiCtxBuilder::new();
		let builder = builder.inherit_stdin().inherit_stdout().inherit_stderr();
		Mutex::new(builder.build(),)
	},)
}

fn fd_write(
	scope: &mut v8::HandleScope,
	args: v8::FunctionCallbackArguments,
	mut rv: v8::ReturnValue,
) {
	// get global object
	let context = scope.get_current_context();
	let global = context.global(scope,);

	// access to global memory
	let str_instance = v8::String::new(scope, "gInstance",).unwrap();
	let instance = global.get(scope, str_instance.into(),).unwrap();
	let instance = instance.to_object(scope,).unwrap();

	// access to instance.exports.memory.buffer
	let str_exports = v8::String::new(scope, "exports",).unwrap();
	let exports = instance.get(scope, str_exports.into(),).unwrap();
	let exports = exports.to_object(scope,).unwrap();
	let str_memory = v8::String::new(scope, "buffer",).unwrap();
	let memory = exports.get(scope, str_memory.into(),).unwrap();
	let memory = memory.to_object(scope,).unwrap();

	//cast as ArrayBuffer
	let str_buffer = v8::String::new(scope, "buffer",).unwrap();
	let array_buffer = memory.get(scope, str_buffer.into(),).unwrap();
	let array_buffer = array_buffer.cast::<v8::ArrayBuffer>();
	let backing_store = array_buffer.get_backing_store();
	let memory: &mut [u8] = unsafe {
		std::slice::from_raw_parts_mut(
			backing_store.data().unwrap().as_ptr() as *mut u8,
			backing_store.byte_length(),
		)
	};
	let memory = unsafe { &*(memory as *mut [u8] as *mut [UnsafeCell<u8,>]) };
	let mut memory = wiggle::GuestMemory::Shared(memory,);

	let arg0 = args.get(0,).integer_value(scope,).unwrap_or_default() as i32;
	let arg1 = args.get(1,).integer_value(scope,).unwrap_or_default() as i32;
	let arg2 = args.get(2,).integer_value(scope,).unwrap_or_default() as i32;
	let arg3 = args.get(3,).integer_value(scope,).unwrap_or_default() as i32;

	let mut wasi_ctx = wasi_ctx_mut().lock().unwrap();
	let rslt = TokioRuntime::new()
		.unwrap()
		.block_on(wasi_snapshot_preview1::fd_write(
			&mut *wasi_ctx,
			&mut memory,
			arg0,
			arg1,
			arg2,
			arg3,
		),)
		.unwrap();

	rv.set(v8::Integer::new(scope, rslt,).into(),);
}

fn import_fd_write<'a,>(
	scope: &mut v8::ContextScope<'a, v8::HandleScope,>,
	fd_write: impl Fn(&mut v8::HandleScope, v8::FunctionCallbackArguments, v8::ReturnValue,),
) -> v8::Local<'a, v8::Object,> {
	let import_wasi_p1 = v8::Object::new(scope,);
	let func_template = v8::FunctionTemplate::new(scope, &fd_write,);
	let func = func_template.get_function(scope,).unwrap();
	let str_fd_write = v8::String::new(scope, "fd_write",).unwrap();
	import_wasi_p1.set(scope, str_fd_write.into(), func.into(),);

	let str_wasi_p1 = v8::String::new(scope, "wasi_snapshot_preview1",).unwrap();
	let import_object: v8::Local<'_, v8::Object,> = v8::Object::new(scope,);
	import_object.set(scope, str_wasi_p1.into(), import_wasi_p1.into(),);
	import_object
}

fn wasm_instance<'a,>(
	scope: &mut v8::ContextScope<'a, v8::HandleScope,>,
	module: v8::Local<'_, v8::WasmModuleObject,>,
	import_object: v8::Local<'_, v8::Object,>,
) -> v8::Local<'a, v8::Object,> {
	let str_wasm = v8::String::new(scope, "WebAssembly",).unwrap();
	let context = scope.get_current_context();
	let global = context.global(scope,);
	let global_wasm = global.get(scope, str_wasm.into(),).unwrap().to_object(scope,).unwrap();
	let str2 = v8::String::new(scope, "Instance",).unwrap();
	let instance_ctor = global_wasm.get(scope, str2.into(),).unwrap();
	let instance_ctor = instance_ctor.cast::<v8::Function>();
	let instance =
		instance_ctor.new_instance(scope, &[module.into(), import_object.into(),],).unwrap();

	let str_ginstance = v8::String::new(scope, "gInstance",).unwrap();
	global.set(scope, str_ginstance.into(), instance.into(),);

	instance
}

fn run(scope: &mut v8::ContextScope<'_, v8::HandleScope,>, instance: v8::Local<'_, v8::Object,>,) {
	let str_exports = v8::String::new(scope, "exports",).unwrap();
	let exports = instance.get(scope, str_exports.into(),).unwrap();
	let exports = exports.to_object(scope,).unwrap();
	let str_start = v8::String::new(scope, "_start",).unwrap();
	let start = exports.get(scope, str_start.into(),).unwrap();
	let start = start.cast::<v8::Function>();

	let ret = start.call(scope, exports.into(), &[],).unwrap();
	println!("{ret:?}");
}

#[cfg(test)]
mod tests {
	use super::*;

	const GOOD_NIGHT: &str = r#"
(module
	(type (;0;) (func (param i32 i32 i32 i32) (result i32)))
	(import "wasi_snapshot_preview1" "fd_write" (func $fd_write (type 0)))
	(export "memory" (memory 0))
	(memory $0 1)
	(data (i32.const 16) "Hello, World\n")

	(func $printHello
		(call $fd_write
			(i32.const 1)
			(i32.const 0)
			(i32.const 1)
			(i32.const 128))
		drop)

(func (export "_start")
	(i32.store (i32.const 0) (i32.const 16))
	(i32.store (i32.const 4) (i32.const 13))
	(call $printHello)))
"#;

	#[test]
	fn play() -> Rslt<(),> { wasm(GOOD_NIGHT,) }

	#[test]
	fn one_fn() {
		let mut isolate = v8::Isolate::new(Default::default(),);
		let scope = &mut v8::HandleScope::new(&mut isolate,);
		let context = v8::Context::new(scope, Default::default(),);
		let scope = &mut v8::ContextScope::new(scope, context,);
		let global = context.global(scope,);

		let wasm_module = std::fs::read("hello.wasm",).expect("Failed to read file",);
		let module = v8::WasmModuleObject::compile(scope, &wasm_module,).unwrap();

		let import_wasi_p1 = v8::Object::new(scope,);
		let func_template = v8::FunctionTemplate::new(scope, fd_write,);
		let func = func_template.get_function(scope,).unwrap();
		let str_fd_write = v8::String::new(scope, "fd_write",).unwrap();
		import_wasi_p1.set(scope, str_fd_write.into(), func.into(),);

		let str_wasi_p1 = v8::String::new(scope, "wasi_snapshot_preview1",).unwrap();
		let import_object = v8::Object::new(scope,);
		import_object.set(scope, str_wasi_p1.into(), import_wasi_p1.into(),);

		let str_wasm = v8::String::new(scope, "WebAssembly",).unwrap();
		let global_wasm = global.get(scope, str_wasm.into(),).unwrap().to_object(scope,).unwrap();
		let str2 = v8::String::new(scope, "Instance",).unwrap();
		let instance_ctor = global_wasm.get(scope, str2.into(),).unwrap();
		let instance_ctor = instance_ctor.cast::<v8::Function>();
		let instance =
			instance_ctor.new_instance(scope, &[module.into(), import_object.into(),],).unwrap();
		let str_ginstance = v8::String::new(scope, "gInstance",).unwrap();
		global.set(scope, str_ginstance.into(), instance.into(),);

		let str_exports = v8::String::new(scope, "exports",).unwrap();
		let exports = instance.get(scope, str_exports.into(),).unwrap();
		let exports = exports.to_object(scope,).unwrap();
		let str_start = v8::String::new(scope, "_start",).unwrap();
		let start = exports.get(scope, str_start.into(),).unwrap();
		let start = start.cast::<v8::Function>();

		// call instance.exports._start()
		let ret = start.call(scope, exports.into(), &[],).unwrap();
		println!("{ret:?}");
	}
}
