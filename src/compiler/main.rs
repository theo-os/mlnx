use anyhow::{bail, Context as _, Result};
use melior_next::{dialect, ir::*, pass, utility::*, Context, ExecutionEngine};

fn main() -> Result<()> {
	let file_path = std::env::args().nth(1).context("No file path provided")?;
	let file =
		std::fs::read_to_string(&file_path).context("Failed to read file")?;
	let parsed = rnix::Root::parse(&file);

	if !parsed.errors().is_empty() {
		println!("Errors:");
		for error in parsed.errors() {
			println!("{}", error);
		}

		bail!("Failed to parse file");
	}

	let ast = parsed.tree();

	dbg!(&ast);

	let registry = dialect::Registry::new();
	register_all_dialects(&registry);

	let context = Context::new();
	context.append_dialect_registry(&registry);
	context.get_or_load_dialect("func");
	register_all_llvm_translations(&context);

	let location = Location::unknown(&context);
	let mut module = Module::new(location);

	let integer_type = Type::integer(&context, 64);

	let function = {
		let region = Region::new();
		let block = Block::new(&[]);

		let constant = operation::Builder::new(
			"arrith.constant",
			Location::unknown(&context),
		)
		.add_attributes(
			&NamedAttribute::new_parsed_vec(&context, &[("value", "0")])
				.context("Failed to parse attributes")?,
		);
		block.append_operation(
			operation::Builder::new("func.return", Location::unknown(&context))
				.add_operands(vec![constant])
				.build(),
		);

		region.append_block(block);

		operation::Builder::new("func.func", Location::unknown(&context))
			.add_attributes(
				&NamedAttribute::new_parsed_vec(
					&context,
					&[
						("function_type", "() -> i64"),
						("sym_name", "\"main\""),
						("llvm.emit_c_interface", "unit"),
					],
				)
				.unwrap(),
			)
			.add_regions(vec![region])
			.build()
	};

	module.body().append_operation(function);

	assert!(module.as_operation().verify());

	let pass_manager = pass::Manager::new(&context);
	register_all_passes();
	pass_manager.add_pass(pass::conversion::convert_scf_to_cf());
	pass_manager.add_pass(pass::conversion::convert_cf_to_llvm());
	pass_manager.add_pass(pass::conversion::convert_func_to_llvm());
	pass_manager.add_pass(pass::conversion::convert_arithmetic_to_llvm());
	pass_manager.enable_verifier(true);
	pass_manager
		.run(&mut module)
		.context("Failed to run pass manager")?;

	let engine = ExecutionEngine::new(&module, 0, &[], false);
	let mut result = 0;

	unsafe {
		engine
			.invoke_packed("add", &mut [&mut result as *mut i64 as *mut ()])
			.context("Failed to invoke function")?;
	}

	Ok(())
}
