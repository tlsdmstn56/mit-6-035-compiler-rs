# Assembler output syntax

## Start of a file

```gas
.file "decaf_example.decaf"
```
## Declaration of text section

```gas
.text
```


## Global variable

```asm
; int
	.globl	num
	.bss
	.align 4
	.type	num, @object
	.size	num, 4
num:
	.zero	4
```


```asm
; int array
	.globl	arr
	.align 8
	.type	arr, @object
	.size	arr, 12
arr:
	.zero	12
```

## Function

```asm
	.text
	.globl	f
	.type	f, @function
f:
.LFB0:
	.cfi_startproc
	endbr64
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	; statements....
	; statements....
	; statements....
	; statements....
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE0:
	.size	f, .-f
```


## String constant

```asm
	.section	.rodata
.LC0:
	.string	"%d\n"
```
