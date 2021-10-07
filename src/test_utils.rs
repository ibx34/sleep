#[macro_export]
macro_rules! read_python_file {
  ($file_name: literal) => {{
    use std::{fs::File, io::Read};

    let mut file = File::open(format!("python/{}.py", $file_name)).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    buffer
  }};
}
