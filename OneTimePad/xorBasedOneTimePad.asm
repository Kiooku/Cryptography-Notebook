section .data
	file db "/dev/urandom", 0
	userPlaintextCiphertextMsg db "Enter your plaintext/ciphertext (No more than 256 character): "
	lenUserPlaintextCiphertextMsg equ $ - userPlaintextCiphertextMsg
	userKeyMsg db "Enter the key [optional for plaintext] (No more than 256 character): "
	lenUserKeyMsg equ $ - userKeyMsg
	errorMessageEmpty db "Error: Plaintext/Ciphertext empty"
	lenErrorMessageEmpty equ $ - errorMessageEmpty
	randomKeyMsg db "Random Key: "
	lenRandomKeyMsg equ $ - randomKeyMsg
	resultMsg db "Result: "
	lenResultMsg equ $ - resultMsg
	errorBadInput db "Error: The key should be at least as long as the message"
	lenErrorBadInput equ $ - errorBadInput

section .bss
	filedescriptor resd 1
	message resb 256
	key resb 256
	result resb 256

section .text
global _start

_start:
	;Ask user plaintext/ciphertext
	mov eax, 4
	mov ebx, 1
	mov ecx, userPlaintextCiphertextMsg
	mov edx, lenUserPlaintextCiphertextMsg
	int 0x80

	;Read and store the user plaintext/ciphertext
	mov eax, 3
	mov ebx, 2
	mov ecx, message
	mov edx, 256
	int 0x80

	;Check if the user input for the message is empty
	cmp eax, 1 ;Length of input (excluding the newline character)
    je message_empty

	;Ask user key
	mov eax, 4
	mov ebx, 1
	mov ecx, userKeyMsg
	mov edx, lenUserKeyMsg
	int 0x80

	;Read and store the user key (optional input)
	mov eax, 3
	mov ebx, 2
	mov ecx, key
	mov edx, 256
	int 0x80

	;Check if the user input for the key is empty
	cmp eax, 1 ;Length of input (excluding the newline character)
    je key_empty ; If key empty, create a random key and otp (solve a segmentation fault)

	;Check if the key is at least as long as the message
	call check_input_are_legit

    call xor_strings

	jmp exit_program

xor_strings:
	; XOR key and message setup
    mov ecx, message
    mov edx, key
    mov edi, result
	
    xor_loop:
        movzx eax, byte [ecx]  ; Load a byte from message into eax
        xor eax, [edx]         ; XOR with the corresponding byte from key
        mov [edi], al          ; Store the result in the result buffer
		; Move to the next byte in each buffer
		inc ecx
        inc edx
        inc edi
        cmp byte [ecx], 0     ; Check for null terminator (end of message string)
        jnz xor_loop           ; If not reached the end, continue the loop
    
	;Output the result message
	mov eax, 4
	mov ebx, 1
	mov ecx, resultMsg
	mov edx, lenResultMsg
	int 0x80

	;Output the XOR result
    mov eax, 4
    mov ebx, 1
    mov ecx, result
    mov edx, 256
    int 0x80

	ret

message_empty:
	;Output an error message if the message is empty and exit
	mov eax, 4
	mov ebx, 1
	mov ecx, errorMessageEmpty
	mov edx, lenErrorMessageEmpty
	int 0x80

	call exit_program

get_random_bytes:
	;Open the file /dev/urandom
	mov eax, 5
	mov ebx, file
	mov ecx, 0
	int 0x80
	mov [filedescriptor], eax

	;Check if the file opened successfully
	cmp eax, 0
	jl exit_program

	;Get a random 256 bytes
	mov eax, 3
	mov ebx, [filedescriptor]
	mov ecx, key
	mov edx, 256
	int 0x80

	ret

key_empty:
	call get_random_bytes

	;Output the random key message
	mov eax, 4
	mov ebx, 1
	mov ecx, randomKeyMsg
	mov edx, lenRandomKeyMsg
	int 0x80
	
	;Output the random key
	mov eax, 4
	mov ebx, 1
	mov ecx, key
	mov edx, 256
	int 0x80

	call xor_strings

	call exit_program

check_input_are_legit:
	;The key should be at least as long as the message
	lea esi, [message]
	lea edi, [key]

	call compare_strings

compare_strings:
	; Clear the counter register
	xor ecx, ecx
	xor edx, edx

	call calculate_length_message

calculate_length_message:
	cmp byte [esi], 0
	je calculate_length_key ;If null, jump to the calculate_length_key

	inc esi
	inc ecx
	jmp calculate_length_message

calculate_length_key:
	cmp byte [edi], 0
	je compare_strings_result

	inc edi
	inc edx
	jmp calculate_length_key

compare_strings_result:
	cmp edx, ecx ; Compare the lengths of message and key
	jl error_unlegit_input ; Jump if message is shorter than key

	ret

error_unlegit_input:
	mov eax, 4
	mov ebx, 1
	mov ecx, errorBadInput
	mov edx, lenErrorBadInput
	int 0x80

	call exit_program

exit_program:
	;Close the file
	mov eax, 6
	mov ebx, [filedescriptor]
	int 0x80

	;Exit code
	mov eax, 1
	int 0x80
