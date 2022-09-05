use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{Write, Error};

pub fn assemble<P: AsRef<Path>>(asm: &String, path: P) -> Result<(), Error> {
    let mut cmd = Command::new("as")
        .arg("-o")
        .arg(path.as_ref())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process for as");
    {
        
        let child_stdin = cmd.stdin.as_mut().take().unwrap();
        child_stdin.write_all(asm.as_bytes());
    }

    match cmd.wait() {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use super::assemble;
    use tempfile::NamedTempFile;
    const TEST_ASM: &str = r#"
	.file	"main.c"
	.text
	.section	.rodata
.LC0:
	.string	"Hello World"
	.text
	.globl	main
	.type	main, @function
main:
.LFB0:
	.cfi_startproc
	endbr64
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movl	%edi, -4(%rbp)
	movq	%rsi, -16(%rbp)
	leaq	.LC0(%rip), %rax
	movq	%rax, %rdi
	call	puts@PLT
	movl	$0, %eax
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE0:
	.size	main, .-main
	.ident	"GCC: (Ubuntu 11.2.0-19ubuntu1) 11.2.0"
	.section	.note.GNU-stack,"",@progbits
	.section	.note.gnu.property,"a"
	.align 8
	.long	1f - 0f
	.long	4f - 1f
	.long	5
0:
	.string	"GNU"
1:
	.align 8
	.long	0xc0000002
	.long	3f - 2f
2:
	.long	0x3
3:
	.align 8
4:
"#;

    #[test]
    fn test_assemble() {
        let tmp_file = NamedTempFile::new().unwrap();
        let path = tmp_file.into_temp_path();
        let code = String::from(TEST_ASM);
        let res = assemble(&code, path);
        assert!(res.is_ok());
    }
}
