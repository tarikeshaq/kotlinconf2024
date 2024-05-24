	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0
	.globl	__ZN3add3add17h04a3194515e4b5bdE
	.p2align	2
__ZN3add3add17h04a3194515e4b5bdE:
Lfunc_begin0:
	.file	1 "/Users/teshaq/code/kotlin-conf24/add" "src/lib.rs"
	.loc	1 1 0
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #16]
	stur	x1, [x29, #-8]
Ltmp0:
	.loc	1 2 5 prologue_end
	adds	x8, x0, x1
	str	x8, [sp, #8]
	cset	w8, hs
	tbnz	w8, #0, LBB0_2
	b	LBB0_1
LBB0_1:
	.loc	1 0 5 is_stmt 0
	ldr	x0, [sp, #8]
	.cfi_def_cfa wsp, 48
	.loc	1 3 2 epilogue_begin is_stmt 1
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB0_2:
	.cfi_restore_state
	.loc	1 2 5
	adrp	x0, _str.0@PAGE
	add	x0, x0, _str.0@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_1@PAGE
	add	x2, x2, l___unnamed_1@PAGEOFF
	bl	__ZN4core9panicking5panic17h3131e0868b9f8622E
Ltmp1:
Lfunc_end0:
	.cfi_endproc

	.section	__TEXT,__const
l___unnamed_2:
	.ascii	"src/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_1:
	.quad	l___unnamed_2
	.asciz	"\n\000\000\000\000\000\000\000\002\000\000\000\005\000\000"

	.section	__TEXT,__const
	.p2align	4, 0x0
_str.0:
	.ascii	"attempt to add with overflow"

	.section	__DWARF,__debug_abbrev,regular,debug
Lsection_abbrev:
	.byte	1
	.byte	17
	.byte	1
	.byte	37
	.byte	14
	.byte	19
	.byte	5
	.byte	3
	.byte	14
	.byte	16
	.byte	23
	.byte	27
	.byte	14
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	0
	.byte	0
	.byte	2
	.byte	57
	.byte	1
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	3
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	63
	.byte	25
	.byte	0
	.byte	0
	.byte	4
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	5
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	62
	.byte	11
	.byte	11
	.byte	11
	.byte	0
	.byte	0
	.byte	0
	.section	__DWARF,__debug_info,regular,debug
Lsection_info:
Lcu_begin0:
.set Lset0, Ldebug_info_end0-Ldebug_info_start0
	.long	Lset0
Ldebug_info_start0:
	.short	4
.set Lset1, Lsection_abbrev-Lsection_abbrev
	.long	Lset1
	.byte	8
	.byte	1
	.long	0
	.short	28
	.long	57
.set Lset2, Lline_table_start0-Lsection_line
	.long	Lset2
	.long	87
	.quad	Lfunc_begin0
.set Lset3, Lfunc_end0-Lfunc_begin0
	.long	Lset3
	.byte	2
	.long	124
	.byte	3
	.quad	Lfunc_begin0
.set Lset4, Lfunc_end0-Lfunc_begin0
	.long	Lset4
	.byte	1
	.byte	109
	.long	128
	.long	124
	.byte	1
	.byte	1
	.long	106

	.byte	4
	.byte	2
	.byte	143
	.byte	16
	.long	166
	.byte	1
	.byte	1
	.long	106
	.byte	4
	.byte	2
	.byte	145
	.byte	120
	.long	171
	.byte	1
	.byte	1
	.long	106
	.byte	0
	.byte	0
	.byte	5
	.long	160
	.byte	7
	.byte	8
	.byte	0
Ldebug_info_end0:
	.section	__TEXT,__text,regular,pure_instructions
Lsec_end0:
	.section	__DWARF,__debug_aranges,regular,debug
	.long	44
	.short	2
.set Lset5, Lcu_begin0-Lsection_info
	.long	Lset5
	.byte	8
	.byte	0
	.space	4,255
	.quad	Lfunc_begin0
.set Lset6, Lsec_end0-Lfunc_begin0
	.quad	Lset6
	.quad	0
	.quad	0
	.section	__DWARF,__debug_str,regular,debug
Linfo_string:
	.asciz	"clang LLVM (rustc version 1.77.2 (25ef9e3d8 2024-04-09))"
	.asciz	"src/lib.rs/@/5d2t7bf2jrihoo3c"
	.asciz	"/Users/teshaq/code/kotlin-conf24/add"
	.asciz	"add"
	.asciz	"_ZN3add3add17h04a3194515e4b5bdE"
	.asciz	"usize"
	.asciz	"left"
	.asciz	"right"
	.section	__DWARF,__apple_names,regular,debug
Lnames_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	2
	.long	2
	.long	12
	.long	0
	.long	1
	.short	1
	.short	6
	.long	0
	.long	-1
	.long	193486030
	.long	549566432
.set Lset7, LNames0-Lnames_begin
	.long	Lset7
.set Lset8, LNames1-Lnames_begin
	.long	Lset8
LNames0:
	.long	124
	.long	1
	.long	47
	.long	0
LNames1:
	.long	128
	.long	1
	.long	47
	.long	0
	.section	__DWARF,__apple_objc,regular,debug
Lobjc_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	1
	.long	0
	.long	12
	.long	0
	.long	1
	.short	1
	.short	6
	.long	-1
	.section	__DWARF,__apple_namespac,regular,debug
Lnamespac_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	1
	.long	1
	.long	12
	.long	0
	.long	1
	.short	1
	.short	6
	.long	0
	.long	193486030
.set Lset9, Lnamespac0-Lnamespac_begin
	.long	Lset9
Lnamespac0:
	.long	124
	.long	1
	.long	42
	.long	0
	.section	__DWARF,__apple_types,regular,debug
Ltypes_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	1
	.long	1
	.long	20
	.long	0
	.long	3
	.short	1
	.short	6
	.short	3
	.short	5
	.short	4
	.short	11
	.long	0
	.long	277156213
.set Lset10, Ltypes0-Ltypes_begin
	.long	Lset10
Ltypes0:
	.long	160
	.long	1
	.long	106
	.short	36
	.byte	0
	.long	0
.subsections_via_symbols
	.section	__DWARF,__debug_line,regular,debug
Lsection_line:
Lline_table_start0:
