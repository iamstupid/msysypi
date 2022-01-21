   .global v0
    .section .sdata
    .align 2
    .type v0,@object
    .size v0,4
v0:
    .word 0
   .text
    .align 2
    .global bubblesort
    .type bubblesort, @function
bubblesort:
    addi sp, sp, -16
    sw ra,12(sp)
li s4,0
.l0:
lui t0,%hi(v0)
lw t0,%lo(v0)(t0)
addi a3,t0,-1
bge s4,a3,.l1
li a2,0
.l2:
lui t0,%hi(v0)
lw t0,%lo(v0)(t0)
sub a3,t0,s4
addi a3,a3,-1
bge a2,a3,.l3
li s0,4
mul a3,a2,s0
add a3,a0,a3
lw a3,0(a3)
addi a1,a2,1
li s0,4
mul a1,a1,s0
add a1,a0,a1
lw a1,0(a1)
ble a3,a1,.l4
addi a3,a2,1
li s0,4
mul a3,a3,s0
add a3,a0,a3
lw a3,0(a3)
mv t6,a3
li s0,4
mul a3,a2,s0
add a3,a0,a3
lw a1,0(a3)
addi a3,a2,1
li s0,4
mul a3,a3,s0
add a3,a0,a3
sw a1,0(a3)
li s0,4
mul a3,a2,s0
add a3,a0,a3
sw t6,0(a3)
.l4:
addi a2,a2,1
j .l2
.l3:
addi s4,s4,1
j .l0
.l1:
li a0,0
lw ra, 12(sp)
addi sp, sp, 16
ret

    .size bubblesort,.-bubblesort
   .text
    .align 2
    .global insertsort
    .type insertsort, @function
insertsort:
    addi sp, sp, -16
    sw ra,12(sp)
li a3,1
.l5:
lui t1,%hi(v0)
lw t1,%lo(v0)(t1)
bge a3,t1,.l6
li s0,4
mul a2,a3,s0
add s4,a0,a2
lw a2,0(s4)
mv a1,a2
addi t6,a3,-1
.l7:
li a2,0
li t1,-1
ble t6,t1,.l9
li s0,4
mul s4,t6,s0
add t5,a0,s4
lw t5,0(t5)
bge a1,t5,.l9
li a2,1
.l9:
beq a2,x0,.l8
li s0,4
mul a2,t6,s0
add a2,a0,a2
lw a2,0(a2)
addi t5,t6,1
li s0,4
mul t5,t5,s0
add t5,a0,t5
sw a2,0(t5)
addi t6,t6,-1
j .l7
.l8:
addi a2,t6,1
li s0,4
mul a2,a2,s0
add a2,a0,a2
sw a1,0(a2)
addi a3,a3,1
j .l5
.l6:
li a0,0
lw ra, 12(sp)
addi sp, sp, 16
ret

    .size insertsort,.-insertsort
   .text
    .align 2
    .global QuickSort
    .type QuickSort, @function
QuickSort:
    addi sp, sp, -32
    sw ra,28(sp)
bge a1,a2,.l10
mv t4,a1
mv t2,a2
li s0,4
mul a3,a1,s0
add a3,a0,a3
lw a3,0(a3)
mv t5,a3
.l11:
bge t4,t2,.l12
.l13:
li t6,0
bge t4,t2,.l15
li s0,4
mul a3,t2,s0
add a3,a0,a3
lw t3,0(a3)
addi a3,t5,-1
ble t3,a3,.l15
li t6,1
.l15:
beq t6,x0,.l14
addi t2,t2,-1
j .l13
.l14:
bge t4,t2,.l16
li s0,4
mul a3,t2,s0
add a3,a0,a3
lw t6,0(a3)
li s0,4
mul a3,t4,s0
add a3,a0,a3
sw t6,0(a3)
addi t4,t4,1
.l16:
.l17:
li a3,0
bge t4,t2,.l19
li s0,4
mul t6,t4,s0
add t6,a0,t6
lw t6,0(t6)
bge t6,t5,.l19
li a3,1
.l19:
beq a3,x0,.l18
addi t4,t4,1
j .l17
.l18:
bge t4,t2,.l20
li s0,4
mul a3,t4,s0
add a3,a0,a3
lw t6,0(a3)
li s0,4
mul a3,t2,s0
add a3,a0,a3
sw t6,0(a3)
addi t2,t2,-1
.l20:
j .l11
.l12:
li s0,4
mul a3,t4,s0
add a3,a0,a3
sw t5,0(a3)
addi t5,t4,-1
addi a3,a0,0
sw a0,0(sp)
sw a2,4(sp)
sw t4,8(sp)
mv a0,a3
mv a1,a1
mv a2,t5
call QuickSort
lw a0,0(sp)
lw a2,4(sp)
lw t4,8(sp)
addi t5,t4,1
mv a0,a0
mv a1,t5
mv a2,a2
call QuickSort
.l10:
li a0,0
lw ra, 28(sp)
addi sp, sp, 32
ret

    .size QuickSort,.-QuickSort
   .text
    .align 2
    .global getMid
    .type getMid, @function
getMid:
    addi sp, sp, -16
    sw ra,12(sp)
lui t0,%hi(v0)
lw t0,%lo(v0)(t0)
li t1,2
rem s4,t0,t1
bne s4,x0,.l21
lui t0,%hi(v0)
lw t0,%lo(v0)(t0)
li s0,2
div a3,t0,s0
li s0,4
mul s4,a3,s0
add a2,a0,s4
lw a2,0(a2)
addi a3,a3,-1
li s0,4
mul a3,a3,s0
add s4,a0,a3
lw s4,0(s4)
add s4,a2,s4
li s0,2
div a0,s4,s0
mv a0,a0
lw ra, 12(sp)
addi sp, sp, 16
ret
.l21:
lui t0,%hi(v0)
lw t0,%lo(v0)(t0)
li s0,2
div a3,t0,s0
li s0,4
mul s4,a3,s0
add s4,a0,s4
lw a0,0(s4)
mv a0,a0
lw ra, 12(sp)
addi sp, sp, 16
ret

    .size getMid,.-getMid
   .text
.align 2
.global getMost
.type getMost, @function
getMost:
sw ra,-4(sp)
li s0, -4016
add sp, sp, s0
li a2,0
.l23:
li t1,1000
bge a2,t1,.l24
li s0,4
mul a3,a2,s0
addi t0,sp,0
add a3,t0,a3
sw x0,0(a3)
addi a2,a2,1
j .l23
.l24:
li a2,0
li t6,0
.l25:
lui t1,%hi(v0)
lw t1,%lo(v0)(t1)
bge a2,t1,.l26
li s0,4
mul a3,a2,s0
add a3,a0,a3
lw a3,0(a3)
mv a1,a3
li s0,4
mul a3,a1,s0
addi t0,sp,0
add a3,t0,a3
lw a3,0(a3)
addi a3,a3,1
li s0,4
mul t5,a1,s0
addi t0,sp,0
add t5,t0,t5
sw a3,0(t5)
li s0,4
mul a3,a1,s0
addi t0,sp,0
add a3,t0,a3
lw a3,0(a3)
ble a3,t6,.l27
li s0,4
mul a3,a1,s0
addi t0,sp,0
add a3,t0,a3
lw a3,0(a3)
mv t6,a3
mv s4,a1
.l27:
addi a2,a2,1
j .l25
.l26:
mv a0,s4

li s0,4016
add sp,sp,s0
lw ra, -4(sp)
ret

.size getMost,.-getMost
   .text
    .align 2
    .global revert
    .type revert, @function
revert:
    addi sp, sp, -16
    sw ra,12(sp)
li a3,0
li a1,0
.l28:
bge a3,a1,.l29
li s0,4
mul a2,a3,s0
add a2,a0,a2
lw a2,0(a2)
mv a2,a2
li s0,4
mul t6,a1,s0
add s4,a0,t6
lw t6,0(s4)
li s0,4
mul t5,a3,s0
add t5,a0,t5
sw t6,0(t5)
li s0,4
mul t6,a1,s0
add t6,a0,t6
sw a2,0(t6)
addi a3,a3,1
addi a1,a1,-1
j .l28
.l29:
li a0,0
lw ra, 12(sp)
addi sp, sp, 16
ret

    .size revert,.-revert
   .text
    .align 2
    .global arrCopy
    .type arrCopy, @function
arrCopy:
    addi sp, sp, -16
    sw ra,12(sp)
li a3,0
.l30:
lui t1,%hi(v0)
lw t1,%lo(v0)(t1)
bge a3,t1,.l31
li s0,4
mul s4,a3,s0
add a2,a0,s4
lw a2,0(a2)
li s0,4
mul s4,a3,s0
add t6,a1,s4
sw a2,0(t6)
addi a3,a3,1
j .l30
.l31:
li a0,0
lw ra, 12(sp)
addi sp, sp, 16
ret

    .size arrCopy,.-arrCopy
   .text
    .align 2
    .global calSum
    .type calSum, @function
calSum:
    addi sp, sp, -16
    sw ra,12(sp)
li a3,0
li t6,0
.l32:
lui t1,%hi(v0)
lw t1,%lo(v0)(t1)
bge t6,t1,.l33
li s0,4
mul a2,t6,s0
add a2,a0,a2
lw a2,0(a2)
add a3,a3,a2
rem a2,t6,a1
addi s4,a1,-1
beq a2,s4,.l34
li s0,4
mul a2,t6,s0
add a2,a0,a2
sw x0,0(a2)
j .l35
.l34:
li s0,4
mul a2,t6,s0
add a2,a0,a2
sw a3,0(a2)
li a3,0
.l35:
addi t6,t6,1
j .l32
.l33:
li a0,0
lw ra, 12(sp)
addi sp, sp, 16
ret

    .size calSum,.-calSum
   .text
    .align 2
    .global avgPooling
    .type avgPooling, @function
avgPooling:
    addi sp, sp, -16
    sw ra,12(sp)
li s4,0
li a3,0
.l36:
lui t1,%hi(v0)
lw t1,%lo(v0)(t1)
bge s4,t1,.l37
addi a2,a1,-1
bge s4,a2,.l38
li s0,4
mul a2,s4,s0
add a2,a0,a2
lw a2,0(a2)
add a3,a3,a2
j .l39
.l38:
addi a2,a1,-1
bne s4,a2,.l40
lw a2,0(a0)
mv t6,a2
div a2,a3,a1
sw a2,0(a0)
j .l41
.l40:
li s0,4
mul a2,s4,s0
add a2,a0,a2
lw a2,0(a2)
add a3,a3,a2
sub a3,a3,t6
sub a2,s4,a1
addi a2,a2,1
li s0,4
mul a2,a2,s0
add a2,a0,a2
lw a2,0(a2)
mv t6,a2
div t5,a3,a1
sub a2,s4,a1
addi a2,a2,1
li s0,4
mul a2,a2,s0
add a2,a0,a2
sw t5,0(a2)
.l41:
.l39:
addi s4,s4,1
j .l36
.l37:
lui t0,%hi(v0)
lw t0,%lo(v0)(t0)
sub s4,t0,a1
addi s4,s4,1
.l42:
lui t1,%hi(v0)
lw t1,%lo(v0)(t1)
bge s4,t1,.l43
li s0,4
mul a3,s4,s0
add a3,a0,a3
sw x0,0(a3)
addi s4,s4,1
j .l42
.l43:
li a0,0
lw ra, 12(sp)
addi sp, sp, 16
ret

    .size avgPooling,.-avgPooling
   .text
    .align 2
    .global main
    .type main, @function
main:
    addi sp, sp, -272
    sw ra,268(sp)
li t0,32
la t1,v0
sw t0,0(t1)
addi t1,sp,0
li t0,7
sw t0,0(t1)
addi t1,sp,4
li t0,23
sw t0,0(t1)
addi t1,sp,8
li t0,89
sw t0,0(t1)
addi t1,sp,12
li t0,26
sw t0,0(t1)
addi t1,sp,16
li t0,282
sw t0,0(t1)
addi t1,sp,20
li t0,254
sw t0,0(t1)
addi t1,sp,24
li t0,27
sw t0,0(t1)
addi t1,sp,28
li t0,5
sw t0,0(t1)
addi t1,sp,32
li t0,83
sw t0,0(t1)
addi t1,sp,36
li t0,273
sw t0,0(t1)
addi t1,sp,40
li t0,574
sw t0,0(t1)
addi t1,sp,44
li t0,905
sw t0,0(t1)
addi t1,sp,48
li t0,354
sw t0,0(t1)
addi t1,sp,52
li t0,657
sw t0,0(t1)
addi t1,sp,56
li t0,935
sw t0,0(t1)
addi t1,sp,60
li t0,264
sw t0,0(t1)
addi t1,sp,64
li t0,639
sw t0,0(t1)
addi t1,sp,68
li t0,459
sw t0,0(t1)
addi t1,sp,72
li t0,29
sw t0,0(t1)
addi t1,sp,76
li t0,68
sw t0,0(t1)
addi t1,sp,80
li t0,929
sw t0,0(t1)
addi t1,sp,84
li t0,756
sw t0,0(t1)
addi t1,sp,88
li t0,452
sw t0,0(t1)
addi t1,sp,92
li t0,279
sw t0,0(t1)
addi t1,sp,96
li t0,58
sw t0,0(t1)
addi t1,sp,100
li t0,87
sw t0,0(t1)
addi t1,sp,104
li t0,96
sw t0,0(t1)
addi t1,sp,108
li t0,36
sw t0,0(t1)
addi t1,sp,112
li t0,39
sw t0,0(t1)
addi t1,sp,116
li t0,28
sw t0,0(t1)
addi t1,sp,120
li t0,1
sw t0,0(t1)
addi t1,sp,124
li t0,290
sw t0,0(t1)
addi t0,sp,0
addi a0,t0,0
addi t0,sp,128
addi a1,t0,0
mv a0,a0
mv a1,a1
call arrCopy
addi t0,sp,128
addi a0,t0,0
mv a0,a0
call revert
li a1,0
.l44:
li t1,32
bge a1,t1,.l45
li s0,4
mul s4,a1,s0
addi t0,sp,128
add s4,t0,s4
lw a0,0(s4)
mv a0,a0
sw a1,256(sp)
mv a0,a0
call putint
lw a1,256(sp)
addi a1,a1,1
j .l44
.l45:
addi t0,sp,128
addi a0,t0,0
mv a0,a0
call bubblesort
li a1,0
.l46:
li t1,32
bge a1,t1,.l47
li s0,4
mul s4,a1,s0
addi t0,sp,128
add s4,t0,s4
lw a0,0(s4)
mv a0,a0
sw a1,256(sp)
mv a0,a0
call putint
lw a1,256(sp)
addi a1,a1,1
j .l46
.l47:
addi t0,sp,128
addi a0,t0,0
mv a0,a0
call getMid
mv a0,a0
mv a0,a0
call putint
addi t0,sp,128
addi a0,t0,0
mv a0,a0
call getMost
mv a0,a0
mv a0,a0
call putint
addi t0,sp,0
addi a0,t0,0
addi t0,sp,128
addi a1,t0,0
mv a0,a0
mv a1,a1
call arrCopy
addi t0,sp,128
addi a0,t0,0
mv a0,a0
call bubblesort
li a1,0
.l48:
li t1,32
bge a1,t1,.l49
li s0,4
mul s4,a1,s0
addi t0,sp,128
add s4,t0,s4
lw a0,0(s4)
mv a0,a0
sw a1,256(sp)
mv a0,a0
call putint
lw a1,256(sp)
addi a1,a1,1
j .l48
.l49:
addi t0,sp,0
addi a0,t0,0
addi t0,sp,128
addi a1,t0,0
mv a0,a0
mv a1,a1
call arrCopy
addi t0,sp,128
addi a0,t0,0
mv a0,a0
call insertsort
li a1,0
.l50:
li t1,32
bge a1,t1,.l51
li s0,4
mul s4,a1,s0
addi t0,sp,128
add s4,t0,s4
lw a0,0(s4)
mv a0,a0
sw a1,256(sp)
mv a0,a0
call putint
lw a1,256(sp)
addi a1,a1,1
j .l50
.l51:
addi t0,sp,0
addi a0,t0,0
addi t0,sp,128
addi a1,t0,0
mv a0,a0
mv a1,a1
call arrCopy
li a1,0
li a0,31
addi t0,sp,128
addi a3,t0,0
sw a1,256(sp)
sw a0,260(sp)
mv a0,a3
mv a1,a1
lw a2,260(sp)
call QuickSort
lw a1,256(sp)
.l52:
li t1,32
bge a1,t1,.l53
li s0,4
mul s4,a1,s0
addi t0,sp,128
add s4,t0,s4
lw a0,0(s4)
mv a0,a0
sw a1,256(sp)
mv a0,a0
call putint
lw a1,256(sp)
addi a1,a1,1
j .l52
.l53:
addi t0,sp,0
addi a0,t0,0
addi t0,sp,128
addi a1,t0,0
mv a0,a0
mv a1,a1
call arrCopy
addi t0,sp,128
addi a0,t0,0
mv a0,a0
li t1,4
mv a1,t1
call calSum
li a1,0
.l54:
li t1,32
bge a1,t1,.l55
li s0,4
mul s4,a1,s0
addi t0,sp,128
add s4,t0,s4
lw a0,0(s4)
mv a0,a0
sw a1,256(sp)
mv a0,a0
call putint
lw a1,256(sp)
addi a1,a1,1
j .l54
.l55:
addi t0,sp,0
addi a0,t0,0
addi t0,sp,128
addi a1,t0,0
mv a0,a0
mv a1,a1
call arrCopy
addi t0,sp,128
addi a0,t0,0
mv a0,a0
li t1,3
mv a1,t1
call avgPooling
li a1,0
.l56:
li t1,32
bge a1,t1,.l57
li s0,4
mul s4,a1,s0
addi t0,sp,128
add s4,t0,s4
lw a0,0(s4)
mv a0,a0
sw a1,256(sp)
mv a0,a0
call putint
lw a1,256(sp)
addi a1,a1,1
j .l56
.l57:
li a0,0
lw ra, 268(sp)
addi sp, sp, 272
ret

    .size main,.-main
