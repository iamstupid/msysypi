    .comm v1,40,4
    .global v0
    .section .sdata
    .align  2
    .type   v0, @object
    .size   v0, 4
v0:
    .word   0


    .text
    .align 2
    .global main
    .type main, @function
main:
    addi sp, sp, -240
    sw ra,236(sp)
    call getint
    la s4,v0
    sw a0,0(s4)
    lui s1,%hi(v0)
    lw s1,%lo(v0)(s1)
    li s2,10
    ble s1,s2,.l0
    li a0,1
    lw ra, 236(sp)
    addi sp, sp, 240
    ret
.l0:
    li s1,0
    sw s1,12(sp)
    lw s1,12(sp)
    sw s1,8(sp)
.l1:
    lw s1,12(sp)
    lui s2,%hi(v0)
    lw s2,%lo(v0)(s2)
    bge s1,s2,.l2
    call getint
    sw a0,20(sp)
    lw a0,20(sp)
    call putint
    li s1,4
    lw s2,12(sp)
    mul s1,s1,s2
    sw s1,24(sp)
    la s1,v1
    lw s2,24(sp)
    add s1,s1,s2
    sw s1,28(sp)
    lw s1,20(sp)
    lw s4,28(sp)
    sw s1,0(s4)
    li s1,4
    lw s2,12(sp)
    mul s1,s1,s2
    sw s1,40(sp)
    la s1,v1
    lw s2,40(sp)
    add s1,s1,s2
    sw s1,44(sp)
    li s1,0
    lw s2,44(sp)
    add s1,s1,s2
    lw s1,0(s1)
    sw s1,36(sp)
    lw s1,8(sp)
    lw s2,36(sp)
    add s1,s1,s2
    sw s1,8(sp)
    lw s1,12(sp)
    li s2,1
    add s1,s1,s2
    sw s1,12(sp)
    j .l1
.l2:
    lw a0,8(sp)
    call putint
    li s1,10
    sw s1,52(sp)
    lw a0,52(sp)
    call putch
    lw a0,8(sp)
    lw ra, 236(sp)
    addi sp, sp, 240
    ret
    li a0,0
    lw ra, 236(sp)
    addi sp, sp, 240
    ret

    .size main,.-main
