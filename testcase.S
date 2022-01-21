   .comm v0, 8, 4
   .text
    .align 2
    .global func
    .type func, @function
func:
    addi sp, sp, -16
    sw ra,12(sp)
la t1,v0
lw s4,0(t1)
li t0,3
sub s4,t0,s4
li s0,4
mul s4,s4,s0
add s4,a0,s4
lw a0,0(s4)
mv a0,a0
lw ra, 12(sp)
addi sp, sp, 16
ret

    .size func,.-func
   .text
    .align 2
    .global main
    .type main, @function
main:
    addi sp, sp, -32
    sw ra,28(sp)
la t1,v0
li t0,1
sw t0,0(t1)
la t1,v0
li t0,2
sw t0,4(t1)
addi t1,sp,4
li t0,-1
sw t0,0(t1)
addi t1,sp,4
lw s4,0(t1)
addi s4,s4,-2
addi t1,sp,8
sw s4,0(t1)
addi t1,sp,8
lw s4,0(t1)
mv s4,s4
addi t1,sp,12
li t0,16
sw t0,0(t1)
addi t0,sp,4
addi a0,t0,0
sw s4,16(sp)
mv a0,a0
call func
mv a0,a0
lw s4,16(sp)
addi a3,a0,2
add a0,a3,s4
mv a0,a0
lw ra, 28(sp)
addi sp, sp, 32
ret

    .size main,.-main
