#[cfg(test)]
mod tests {
	use std::fs::{read_dir, read_to_string, DirEntry};
	use std::process::Command;

	// #[test]
	fn _execute_tests() {
		let cases = read_dir("tests/cases").expect("Could not read tests/cases");
		let mut errors = vec![];
		let mut msgs = vec![];
		for case in cases {
			let case = case.expect("Could not read test case");
			let name = case.path().display().to_string();
			if name.contains('~') {
				continue;
			}
			match run_test(case) {
				Ok(_) => msgs.push(format!("Running {name:.<85}...ok")),
				Err(msg) => {
					errors.push(msg);
					msgs.push(format!("Running {name:.<85}...failed"));
				}
			}
		}
		println!("Ran {} tests", msgs.len());
		msgs.sort();
		for msg in &msgs {
			println!("{}", msg);
		}
		if !errors.is_empty() {
			panic!("Errors:\n\n{}", errors.join("\n\n"));
		}
	}

	fn run_test(file: DirEntry) -> Result<(), String> {
		let contents = read_to_string(file.path()).expect("Could not read test file");
		let lines = contents.split('\n').collect::<Vec<&str>>();
		let mut test_code = vec![];
		let mut idx = None;

		for (i, line) in lines.iter().enumerate() {
			if line.starts_with("// --- Test") {
				continue;
			}
			if line.starts_with("// --- Expected") {
				idx = Some(i);
				break;
			}
			test_code.push(*line);
		}

		let idx = idx.expect(&format!(
			"{:#?}: No expected section in test case definition",
			file.file_name()
		));

		let mut expected_output = vec![];
		for line in &lines[idx + 1..] {
			if !line.is_empty() {
				expected_output.push(line[3..].to_string());
			}
		}

		let input = test_code.join("\n");
		let raw = Command::new("cargo")
			.args(["run", "--", "run"])
			.arg(&input)
			.output()
			.expect("Could not run cargo");

		let stdout = String::from_utf8_lossy(&raw.stdout);
		let output = stdout.split('\n').filter(|l| !l.is_empty()).collect::<Vec<&str>>();

		if output.len() != expected_output.len() {
			return Err(format!(
				"{:#?}: output length does not match expected: {} != {}\nFull output:\n{}",
				file.file_name(),
				output.len(),
				expected_output.len(),
				stdout
			));
		}

		for (i, expected) in expected_output.iter().enumerate() {
			if output[i] != expected.trim() {
				return Err(format!(
					"{:#?}: line {} — got {:?}, expected {:?}\nFull output:\n{}",
					file.file_name(),
					i,
					output[i],
					expected,
					stdout
				));
			}
		}

		Ok(())
	}
}
