	var 40 T1
	var T0
f_main [0]
	var T2
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
	T2 = 0
	T3 = T2
l2:
	if T2 < T0 goto l3
	goto l4
l3:
	t0 = T2 * 4
	t1 = call f_getint
	T1[t0] = t1
	t3 = T2 * 4
	t2 = T1[t3]
	T3 = T3 + t2
	T2 = T2 + 1
	goto l2
l4:
	param T3
	t4 = call f_putint
	return 0
end f_main
