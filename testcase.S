	var 48 T0
f_main [0]
	var T2
	var T1
	var t0
	var t1
	var t2
	var t3
	var t4
	var t5
	var T3
	T1 = 0
	T2 = 0
l0:
	if T1 <= 5 goto l7
	goto l8
l7:
	T3 = T1
l1:
	if T3 >= 0 goto l5
	goto l6
l5:
	if T3 < 4 goto l2
	goto l4
l2:
	t0 = T1 - T3
	if t0 < 3 goto l3
	goto l4
l3:
	t1 = T3 * 4
	t2 = T1 - T3
	t3 = t2 * 16
	t4 = t3 + t1
	t5 = T2
	T0[t4] = t5
	T2 = 1
l4:
	T3 = T3 - 1
	goto l1
l6:
	T1 = T1 + 1
	goto l0
l8:
	return 0
end f_main
