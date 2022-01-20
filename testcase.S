	var T0
f_counting_sort [3]
	var T3
	var 40 T1
	var T2
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
	T4 = 0
	T2 = 0
	T3 = 0
l0:
	if T4 < 10 goto l1
	goto l2
l1:
	t0 = T4 * 4
	t1 = 0
	T1[t0] = t1
	T4 = T4 + 1
	goto l0
l2:
l3:
	if T2 < p2 goto l4
	goto l5
l4:
	t3 = T2 * 4
	t2 = p0[t3]
	t4 = t2 * 4
	t8 = T2 * 4
	t7 = p0[t8]
	t9 = t7 * 4
	t6 = T1[t9]
	t5 = t6 + 1
	T1[t4] = t5
	T2 = T2 + 1
	goto l3
l5:
	T4 = 1
l6:
	if T4 < 10 goto l7
	goto l8
l7:
	t10 = T4 * 4
	t13 = T4 * 4
	t12 = T1[t13]
	t15 = T4 - 1
	t16 = t15 * 4
	t14 = T1[t16]
	t11 = t12 + t14
	T1[t10] = t11
	T4 = T4 + 1
	goto l6
l8:
	T3 = p2
l9:
	if T3 > 0 goto l10
	goto l11
l10:
	t18 = T3 - 1
	t19 = t18 * 4
	t17 = p0[t19]
	t20 = t17 * 4
	t24 = T3 - 1
	t25 = t24 * 4
	t23 = p0[t25]
	t26 = t23 * 4
	t22 = T1[t26]
	t21 = t22 - 1
	T1[t20] = t21
	t29 = T3 - 1
	t30 = t29 * 4
	t28 = p0[t30]
	t31 = t28 * 4
	t27 = T1[t31]
	t32 = t27 * 4
	t34 = T3 - 1
	t35 = t34 * 4
	t33 = p0[t35]
	p1[t32] = t33
	T3 = T3 - 1
	goto l9
l11:
	return 0
end f_counting_sort
f_main [0]
	var T4
	var 40 T3
	var T2
	var 40 T1
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
	T0 = 10
	t0 = 0 * 4
	t1 = 4
	T1[t0] = t1
	t2 = 1 * 4
	t3 = 3
	T1[t2] = t3
	t4 = 2 * 4
	t5 = 9
	T1[t4] = t5
	t6 = 3 * 4
	t7 = 2
	T1[t6] = t7
	t8 = 4 * 4
	t9 = 0
	T1[t8] = t9
	t10 = 5 * 4
	t11 = 1
	T1[t10] = t11
	t12 = 6 * 4
	t13 = 6
	T1[t12] = t13
	t14 = 7 * 4
	t15 = 5
	T1[t14] = t15
	t16 = 8 * 4
	t17 = 7
	T1[t16] = t17
	t18 = 9 * 4
	t19 = 8
	T1[t18] = t19
	T2 = 0
	param T1
	param T3
	param T0
	T2 = call f_counting_sort
l12:
	if T2 < T0 goto l13
	goto l14
l13:
	t20 = T2 * 4
	T4 = T3[t20]
	param T4
	call f_putint
	T4 = 10
	param T4
	call f_putch
	T2 = T2 + 1
	goto l12
l14:
	return 0
end f_main
