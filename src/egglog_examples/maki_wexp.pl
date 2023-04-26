/* Rewrite rules. */
and(A,B) <- and(B,A).
or(A,B) <- or(B,A).
A <- not(not(A)).
or(or(A,B),C) <- or(A,or(B,C)).
and(and(A,B),C) <- and(A,and(B,C)).

true <- or(A,not(A)).
A <- and(A,true).
A <- or(A,false).
false <- and(A,false).
true <- or(A,true).
A <- and(A,A).
A <- or(A,A).
A <- or(A,and(A,B)).
A <- and(A,or(A,B)).
xor(A,B) <- xor(B,A).
xor(xor(A,B),C) <- xor(A,xor(B,C)).
not(A) <- xor(A,true).
A <- xor(A,false).

/* full adds */
full(A,B,C) <- concat(xor(xor(A,B),C),or(or(and(A,B),and(A,C)), and(B,C))).
full(B,A,C) <- full(A,B,C).
full(C,B,A) <- full(A,B,C).
full(A,C,B) <- full(A,B,C).

/* bidirectional */
xor(A,B) <-> and(or(A,B),not(and(A,B))).
xor(A,B) <-> or(and(not(A),B),and(A,not(B))).
or(and(A,B),and(C,xor(A,B))) <-> or(and(A,C),and(B,xor(A,C))).
not(and(A,B)) <-> or(not(A),not(B)).
not(or(A,B)) <-> and(not(A),not(B)).
and(A,or(B,C)) <-> or(and(A,B),and(A,C)).
or(A,and(B,C)) <-> and(or(A,B),or(A,C)).

/* Initial statements in the e-graph, saturate after running */
or(and(c,d),and(d,e)).
not(not(a)).
or(false,w).
xor(a,xor(b,c)).
onebitadd = concat(xor(xor(a0,cin),b0),or(or(and(a0,cin),and(a0,b0)),and(cin,b0))).

/* Queries (egraph must be populated first before querying, or everything evaluates to unknown) */ 
/* do and(c,d) and and(d,c) both EXIST and belong in the same eclass? */
?- and(c,d) = and(d,c).
/* does and(d,e) exist in the egraph? */
?- and(d,e).
/* extending egg with prolog: what will X be to make the following equivalent? */
?- not(not(X)) = a.
/* what would X be such that not(not(X)) is present in the egraph? */
?- not(not(X)).
/* extract the shortest syntax tree from the eclass representing not(not(a)) */
?- not(not(a)) = X.
/* not(not(not(a))) -> not(a), which is present in the egraph */
?- not(not(not(a))).
?- or(w,false) = w.

/* test boolean logic */
?- xor(b,xor(a,c)).

/* test full adders */
?- full(b0,a0,cin) = full(a0,b0,cin).
/* query: give all occurrences of full adds in the egraph */
?- full(X,Y,Z).

/* 2-bit add */
concat(select(onebitadd,0),concat(xor(xor(a1,b1),select(onebitadd,1)), or(or(and(a1,b1),and(a1,select(onebitadd,1))),and(b1,select(onebitadd,1))))).
?- concat(select(onebitadd,0),concat(xor(xor(a1,b1),select(onebitadd,1)), or(or(and(a1,b1),and(a1,select(onebitadd,1))),and(b1,select(onebitadd,1))))) = X.
