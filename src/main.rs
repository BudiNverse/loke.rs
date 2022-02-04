use argh::FromArgs;
use serde::Deserialize;
use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

/// Run yo damn hackerearth test cases
#[derive(FromArgs)]
pub struct Args {
    /// path to your executable
    #[argh(option)]
    executable: String,

    /// path to your test file
    #[argh(option)]
    test_file: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct TestFormat<'a> {
    pub input: &'a str,
    pub expected: &'a str,
}

fn main() -> Result<(), io::Error> {
    let args = argh::from_env::<Args>();

    // read input file
    let test_p = Path::new(&args.test_file);

    let data = fs::read_to_string(test_p)?;
    let test_cases = serde_json::from_str::<'_, Vec<TestFormat>>(&data)
        .expect("Eh bro your file path sure anot?");

    for (idx, test) in test_cases.into_iter().enumerate() {
        // spawn subprocess
        let mut cmd = Command::new(&args.executable)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;

        let child_stdin = cmd.stdin.as_mut().unwrap();
        let _ = child_stdin.write_all(test.input.as_bytes());

        match cmd.wait_with_output() {
            Ok(output) => {
                if let Ok(s) = String::from_utf8(output.stdout) {
                    if s.trim().eq(test.expected.trim()) {
                        println!("✔️\tTest no: {} passed!", idx);
                    } else {
                        println!(
                            "❌\tTest no: {} failed!\n\t\tExpect:{}\n\t\tActual:\x1b[93m{}\x1b[0m",
                            idx, test.expected.trim(), s.trim()
                        );
                    }
                } else {
                    println!("❌\tTest no: {} failed! Runtime Error!", idx)
                }
            }
            Err(_) => println!("❌\tTest no: {} failed! Runtime Error!", idx),
        }
    }

    Ok(())
}
