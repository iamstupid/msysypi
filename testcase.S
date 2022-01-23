
    .text
    .align 2
    .global a
    .type a, @function
a:
    addi sp, sp, -144
    sw ra,140(sp)
    sw a0,0(sp)
    sw a1,4(sp)
    lw s1,0(sp)
    li s2,1
    bge s1,s2,.l0
    j .l1
.l0:
    lw s1,0(sp)
    li s2,1
    sub s1,s1,s2
    sw s1,12(sp)
    lw s1,0(sp)
    li s2,1
    sub s1,s1,s2
    sw s1,20(sp)
    lw s1,4(sp)
    li s2,1
    add s1,s1,s2
    sw s1,24(sp)
    lw a0,20(sp)
    lw a1,24(sp)
    call a
    sw a0,16(sp)
    lw a0,12(sp)
    lw a1,16(sp)
    call a
    sw a0,8(sp)
    lw a0,8(sp)
    lw ra, 140(sp)
    addi sp, sp, 144
    ret
.l1:
    lw s1,4(sp)
    li s2,1
    add s1,s1,s2
    sw s1,28(sp)
    lw a0,28(sp)
    lw ra, 140(sp)
    addi sp, sp, 144
    ret

    .size a,.-a
    .text
    .align 2
    .global main
    .type main, @function
main:
    addi sp, sp, -16
    sw ra,12(sp)
    li a0,5
    li a1,0
    call putint

    .size main,.-main
