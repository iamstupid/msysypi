   .text
    .align 2
    .global if_if_Else
    .type if_if_Else, @function
if_if_Else:
    addi sp, sp, -16
    sw ra,12(sp)
li a0,5
li s4,10
li t1,5
bne a0,t1,.l0
li t1,10
bne s4,t1,.l1
li a0,25
.l1:
j .l2
.l0:
addi a0,a0,15
.l2:
mv a0,a0
lw ra, 12(sp)
addi sp, sp, 16
ret

    .size if_if_Else,.-if_if_Else
   .text
    .align 2
    .global main
    .type main, @function
main:
    addi sp, sp, -16
    sw ra,12(sp)
call if_if_Else
mv a0,a0
mv a0,a0
lw ra, 12(sp)
addi sp, sp, 16
ret

    .size main,.-main
