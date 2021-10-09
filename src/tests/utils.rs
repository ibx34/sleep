use std::{fs::File, io::Read};

pub fn read_python_test(file_name: &str) -> String {
	let mut file = File::open(format!("python/{}.py", file_name)).unwrap();

	let mut buffer = String::new();
	file.read_to_string(&mut buffer).unwrap();
	buffer
}
