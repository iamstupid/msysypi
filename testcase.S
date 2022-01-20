	var 800 T3
	var 52907904 T2
	var T0
	var T1
	var 80 T5
	T0 = 18
	T1 = -2147483648
f_equal [2]
	if p0 == p1 goto l0
	goto l1
l0:
	return 1
l1:
	return 0
end f_equal
f_dfs [6]
	var T4
	var t0
	var t1
	var t2
	var t3
	var t4
	var t5
	var t6
	var t7
	var t8
	var t9
	var t10
	var t11
	var t12
	var t13
	var t14
	var t15
	var t16
	var t17
	var t18
	var t19
	var t20
	var t21
	var t22
	var t23
	var t24
	var t25
	var t26
	var t27
	var t28
	var t29
	var t30
	var t31
	var t32
	var t33
	var t34
	var t35
	var t36
	var t37
	var t38
	var t39
	var t40
	var t41
	var t42
	var t43
	var t44
	var t45
	var t46
	var t47
	var t48
	var t49
	var t50
	var t51
	var t52
	var t53
	var t54
	var t55
	var t56
	var t57
	var t58
	var t59
	var t60
	var t61
	var t62
	var t63
	var t64
	var t65
	var t66
	var t67
	var t68
	var t69
	var t70
	var t71
	var t72
	var t73
	var t74
	var t75
	var t76
	var t77
	var t78
	var t79
	var t80
	var t81
	var t82
	var t83
	t1 = p5 * 4
	t2 = p4 * 28
	t3 = t2 + t1
	t4 = p3 * 504
	t5 = t4 + t3
	t6 = p2 * 9072
	t7 = t6 + t5
	t8 = p1 * 163296
	t9 = t8 + t7
	t10 = p0 * 2939328
	t11 = t10 + t9
	t0 = T2[t11]
	if t0 != -1 goto l2
	goto l3
l2:
	t13 = p5 * 4
	t14 = p4 * 28
	t15 = t14 + t13
	t16 = p3 * 504
	t17 = t16 + t15
	t18 = p2 * 9072
	t19 = t18 + t17
	t20 = p1 * 163296
	t21 = t20 + t19
	t22 = p0 * 2939328
	t23 = t22 + t21
	t12 = T2[t23]
	return t12
l3:
	t27 = p0 + p1
	t26 = t27 + p2
	t25 = t26 + p3
	t24 = t25 + p4
	if t24 == 0 goto l4
	goto l5
l4:
	return 1
l5:
	T4 = 0
	if 0 != p0 goto l6
	goto l7
l6:
	param p5
	param 2
	t31 = call f_equal
	t30 = p0 - t31
	t33 = p0 - 1
	param t33
	param p1
	param p2
	param p3
	param p4
	param 1
	t32 = call f_dfs
	t29 = t30 * t32
	t28 = T4 + t29
	T4 = t28 % -2147483648
l7:
	if 0 != p1 goto l8
	goto l9
l8:
	param p5
	param 3
	t37 = call f_equal
	t36 = p1 - t37
	t39 = p0 + 1
	param t39
	t40 = p1 - 1
	param t40
	param p2
	param p3
	param p4
	param 2
	t38 = call f_dfs
	t35 = t36 * t38
	t34 = T4 + t35
	T4 = t34 % -2147483648
l9:
	if 0 != p2 goto l10
	goto l11
l10:
	param p5
	param 4
	t44 = call f_equal
	t43 = p2 - t44
	param p0
	t46 = p1 + 1
	param t46
	t47 = p2 - 1
	param t47
	param p3
	param p4
	param 3
	t45 = call f_dfs
	t42 = t43 * t45
	t41 = T4 + t42
	T4 = t41 % -2147483648
l11:
	if 0 != p3 goto l12
	goto l13
l12:
	param p5
	param 5
	t51 = call f_equal
	t50 = p3 - t51
	param p0
	param p1
	t53 = p2 + 1
	param t53
	t54 = p3 - 1
	param t54
	param p4
	param 4
	t52 = call f_dfs
	t49 = t50 * t52
	t48 = T4 + t49
	T4 = t48 % -2147483648
l13:
	if 0 != p4 goto l14
	goto l15
l14:
	param p0
	param p1
	param p2
	t58 = p3 + 1
	param t58
	t59 = p4 - 1
	param t59
	param 5
	t57 = call f_dfs
	t56 = p4 * t57
	t55 = T4 + t56
	T4 = t55 % -2147483648
l15:
	t60 = p5 * 4
	t61 = p4 * 28
	t62 = t61 + t60
	t63 = p3 * 504
	t64 = t63 + t62
	t65 = p2 * 9072
	t66 = t65 + t64
	t67 = p1 * 163296
	t68 = t67 + t66
	t69 = p0 * 2939328
	t70 = t69 + t68
	t71 = T4 % -2147483648
	T2[t70] = t71
	t73 = p5 * 4
	t74 = p4 * 28
	t75 = t74 + t73
	t76 = p3 * 504
	t77 = t76 + t75
	t78 = p2 * 9072
	t79 = t78 + t77
	t80 = p1 * 163296
	t81 = t80 + t79
	t82 = p0 * 2939328
	t83 = t82 + t81
	t72 = T2[t83]
	return t72
end f_dfs
f_main [0]
	var T13
	var T6
	var T7
	var t0
	var t1
	var t2
	var t3
	var t4
	var t5
	var t6
	var t7
	var t8
	var t9
	var t10
	var t11
	var t12
	var t13
	var t14
	var t15
	var t16
	var t17
	var t18
	var t19
	var t20
	var t21
	var t22
	var t23
	var t24
	var t25
	var t26
	var t27
	var t28
	var t29
	var t30
	var t31
	var T8
	var T9
	var T10
	var T11
	var T12
	T6 = call f_getint
	T7 = 0
l16:
	if T7 < 18 goto l32
	goto l33
l32:
	T8 = 0
l17:
	if T8 < 18 goto l30
	goto l31
l30:
	T9 = 0
l18:
	if T9 < 18 goto l28
	goto l29
l28:
	T10 = 0
l19:
	if T10 < 18 goto l26
	goto l27
l26:
	T11 = 0
l20:
	if T11 < 18 goto l24
	goto l25
l24:
	T12 = 0
l21:
	if T12 < 7 goto l22
	goto l23
l22:
	t0 = T12 * 4
	t1 = T11 * 28
	t2 = t1 + t0
	t3 = T10 * 504
	t4 = t3 + t2
	t5 = T9 * 9072
	t6 = t5 + t4
	t7 = T8 * 163296
	t8 = t7 + t6
	t9 = T7 * 2939328
	t10 = t9 + t8
	t11 = -1
	T2[t10] = t11
	T12 = T12 + 1
	goto l21
l23:
	T11 = T11 + 1
	goto l20
l25:
	T10 = T10 + 1
	goto l19
l27:
	T9 = T9 + 1
	goto l18
l29:
	T8 = T8 + 1
	goto l17
l31:
	T7 = T7 + 1
	goto l16
l33:
	T7 = 0
l34:
	if T7 < T6 goto l35
	goto l36
l35:
	t12 = T7 * 4
	t13 = call f_getint
	T3[t12] = t13
	t15 = T7 * 4
	t14 = T3[t15]
	t16 = t14 * 4
	t20 = T7 * 4
	t19 = T3[t20]
	t21 = t19 * 4
	t18 = T5[t21]
	t17 = t18 + 1
	T5[t16] = t17
	T7 = T7 + 1
	goto l34
l36:
	t23 = 1 * 4
	t22 = T5[t23]
	param t22
	t25 = 2 * 4
	t24 = T5[t25]
	param t24
	t27 = 3 * 4
	t26 = T5[t27]
	param t26
	t29 = 4 * 4
	t28 = T5[t29]
	param t28
	t31 = 5 * 4
	t30 = T5[t31]
	param t30
	param 0
	T13 = call f_dfs
	param T13
	call f_putint
	return T13
end f_main
