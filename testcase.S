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
    addi sp, sp, -96
    sw ra,92(sp)
    call getint
    la s4,v0
    sw a0,0(s4)
    lui s1,%hi(v0)
    lw s1,%lo(v0)(s1)
    li s2,10
    ble s1,s2,.l0
    li a0,1
    lw ra, 92(sp)
    addi sp, sp, 96
    ret
.l0:
    li s1,0
    sw s1,0(sp)
    lw s1,0(sp)
    sw s1,16(sp)
.l1:
    lw s1,0(sp)
    lui s2,%hi(v0)
    lw s2,%lo(v0)(s2)
    bge s1,s2,.l2
    call getint
    sw a0,32(sp)
    lw s1,0(sp)
    li s2,4
    mul s1,s1,s2
    sw s1,48(sp)
    lw s1,48(sp)
    la s2,v1
    add s1,s1,s2
    lw s1,0(s1)
    sw s1,64(sp)
    lw s1,16(sp)
    lw s2,64(sp)
    add s1,s1,s2
    sw s1,16(sp)
    lw s1,0(sp)
    li s2,1
    add s1,s1,s2
    sw s1,0(sp)
    j .l1
.l2:
    lw a0,16(sp)
    call putint
    li a0,0
    lw ra, 92(sp)
    addi sp, sp, 96
    ret

    .size main,.-main
