f_a [2]
	var t0
	var t1
	var t2
	var t3
	var t4
	var t5
	if p0 >= 1 goto l0
	goto l1
l0:
	t1 = p0 - 1
	t3 = p0 - 1
	t4 = p1 + 1
	param t3
	param t4
	t2 = call f_a
	param t1
	param t2
	t0 = call f_a
	return t0
l1:
	t5 = p1 + 1
	return t5
end f_a
f_main [0]
	param 5
	param 0
	call f_putint
end f_main
