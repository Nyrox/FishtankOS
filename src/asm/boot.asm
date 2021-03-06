extern kmain

global start

section .text
bits 32
start:
	; Move the address of the p3 table into eax and set the 'Present bit' and the 'Writable Bit' and then move that value into the first entry of the p4_table
	mov eax, p3_table
	or eax, 0b11
	mov dword [p4_table + 0], eax

	; Same thing, this time we point the first entry of the p3 table to the first entry of the p2 table
	mov eax, p2_table
	or eax, 0b11
	mov dword [p3_table + 0], eax

	; Now we need to point each p2_table entry to a page
	mov ecx, 0 ; counter
.map_p2_table:
	mov eax, 0x200000; 2MiB
	mul ecx
	or eax, 0b10000011
	mov [p2_table + ecx * 8], eax

	inc ecx
	cmp ecx, 512
	jne .map_p2_table
	
	; Move page table address to cr3
	mov eax, p4_table
	mov cr3, eax

.enable_pae:	
	; Enable PAE
	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax

	; Set the long mode bit
	mov ecx, 0xC0000080
	rdmsr
	or eax, 1 << 8
	wrmsr

.enable_paging:
	; enable paging
	mov eax, cr0
	or eax, 1 << 31
	or eax, 1 << 16
	mov cr0, eax

.init_gdt:
	; Setup gdt
	lgdt [gdt64.pointer]

.init_long_mode:
	; Enable proper long mode
	mov ax, gdt64.data
	mov ss, ax
	mov ds, ax
	mov es, ax

.init_kernel:
	jmp gdt64.code:kmain
	; See you on the other side, jimmy!

; Welcome to bullshit land
section .bss

align 4096

p4_table:
	resb 4096
p3_table:
	resb 4096
p2_table:
	resb 4096



; GDT
section .rodata 
gdt64:
	dq 0

.code: equ $ - gdt64
	dq (1<<44) | (1<<47) | (1<<41) | (1 << 43) | (1<<53)

.data: equ $ - gdt64
	dq (1<<44) | (1<<47) | (1<<41)

.pointer:
	dw .pointer - gdt64 - 1
	dq gdt64



