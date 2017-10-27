global start

section .text
bits 32
start:
	mov word [0xb8000], 0x0248 ; H	
	mov word [0xb8002], 0x0265 ; E
	mov word [0xb8004], 0x026c ; L
	mov word [0xb8006], 0x026c ; L
	mov word [0xb8008], 0x026f ; O
	hlt

