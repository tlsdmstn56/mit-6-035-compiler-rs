use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use std::boxed::Box;
use std::fmt;
use std::fmt::{Display, Debug};
use std::error::Error;

pub struct LinkerError(String);
impl Error for LinkerError {}

impl Display for LinkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Debug for LinkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

// ld /usr/lib/x86_64-linux-gnu/crti.o /usr/lib/x86_64-linux-gnu/crtn.o /usr/lib/x86_64-linux-gnu/crt1.o -lc main.o -dynamic-linker /lib64/ld-linux-x86-64.so.2 -o main_ELF_executable
pub fn link<P1, P2, T, I>(obj_path: P1, out_path: P2, libs: I) -> Result<(), Box<dyn std::error::Error>>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    T: AsRef<OsStr>,
    I: IntoIterator<Item = T>,
{
    let cmd = Command::new("ld")
        .arg("/usr/lib/x86_64-linux-gnu/crti.o")
        .arg("/usr/lib/x86_64-linux-gnu/crtn.o")
        .arg("/usr/lib/x86_64-linux-gnu/crt1.o")
        .arg("-lc") // libc.so
        .arg(obj_path.as_ref())
        .arg("-dynamic-linker") // libc.so
        .arg("/lib64/ld-linux-x86-64.so.2")
        .arg("-o")
        .arg(out_path.as_ref())
        .output();
    
    match cmd {
        Ok(output) => if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
            Err(Box::new(LinkerError(stderr)))
        },
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::link;
    use crate::assembler::assemble;
    use crate::test_util::TEST_ASM;
    use tempfile::NamedTempFile;
    use std::process::Command;

    #[test]
    fn test_link() {
        let asm_file = NamedTempFile::new().unwrap();
        let asm_path = asm_file.into_temp_path();
        let code = String::from(TEST_ASM);
        let res = assemble(&code, &asm_path);
        assert!(res.is_ok());
        let out_file = NamedTempFile::new().unwrap();
        let out_path = out_file.into_temp_path();
        let libs = Vec::<String>::new();
        let res = link(&asm_path, &out_path, libs);
        assert!(res.is_ok());
        let res = Command::new(&out_path).output().unwrap();
        let stdout = String::from_utf8_lossy(&res.stdout).into_owned();
        assert_eq!(stdout, "Hello World\n");
    }
}
