use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{Write};
use std::error::Error;
use std::boxed::Box;
use std::fmt;
use std::fmt::{Display, Debug};

pub struct AssemblerError(String);
impl Error for AssemblerError {}

impl Display for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Debug for AssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

pub fn assemble<P: AsRef<Path>>(asm: &String, path: P) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("as")
        .arg("-o")
        .arg(path.as_ref())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process for as");
    {
        
        let child_stdin = cmd.stdin.as_mut().take().unwrap();
        if let Err(e) = child_stdin.write_all(asm.as_bytes()) {
            return Err(Box::new(e));
        }
    }

    match cmd.wait_with_output() {
        Ok(output) => if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
            Err(Box::new(AssemblerError(stderr)))
        },
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::assemble;
    use tempfile::NamedTempFile;
    use crate::test_util::TEST_ASM;

    #[test]
    fn test_assemble() {
        let tmp_file = NamedTempFile::new().unwrap();
        let path = tmp_file.into_temp_path();
        let code = String::from(TEST_ASM);
        let res = assemble(&code, path);
        assert!(res.is_ok());
    }
}
