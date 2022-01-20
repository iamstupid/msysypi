	var T0
	var 40 T1
f_main [0]
	var T4
	var T3
	var t0
	var t1
	var t2
	var t3
	var t4
	T0 = call f_getint
	if T0 > 10 goto l0
	goto l1
l0:
	return 1
l1:
	T3 = 0
	T4 = T3
l2:
	if T3 < T0 goto l3
	goto l4
l3:
	t0 = T3 * 4
	t1 = call f_getint
	T1[t0] = t1
	t3 = T3 * 4
	t2 = T1[t3]
	T4 = T4 + t2
	T3 = T3 + 1
	goto l2
l4:
	param T4
	t4 = call f_putint
	return 0
end f_main