/* Base equality facts, in the egraph */
q = x.
wag = x.
z = y.

/* Base egraph insertions. - (3 separate egraphs?) No implied equalities. For "seeding" the egraph. */
f(x).
bar(boo).
plus(p,r).

/* general looking prolog-like rules that operate over the egraph */
/* bar(X) if f(X) = q f(X) rewrites to q (same e-class) */
bar(X) :- f(X) = q.
biz(X) :- bar(X).
biz(Z) = baz(biz(boz)) :- fizzy(floozy), buppo(Z).

/* rewrite rules. Variables denoted by capitalization */
plus(X,Y) <- plus(Y,X).
/* In principle syntactic shorthand for plus(X,Y) = C :- plus(Y,X) = C. */
/* Rewrites have synergy as well because they are just prolog */
X <- f(f(f(X))).
X <- f(f(X)).
/* implies X <- f(X) */

/* bidirectional rewrite. A useful syntactic shorthand for two rewrite rules. */ 
plus(X,plus(Y,Z)) <-> plus(plus(X,Y),Z).

/* Guarded rewrite. (learn more) */
fiz(baz) <- bar(boo), x = z.

/* Queries
Note that this does NOT insert into the egraph. Should I change that? Or give a new notation for "insert all subterms and then query"?
 */
/* we are querying the egraph with equality statements */
/* equals means equivalent in the egraph */
?- f(x) = X.
?- x = x.
?- y = x.

/* needs to be in the initial egraph for query to work */
?- plus(p,r) = plus(r,p).

?- junk(boo) = otherjunk(baz).
?- bar(boo) = fizz(baz).
?- bar(boo).
?- biz(boo).
?- bar(x).
?- bar(whiff).

/* Query with variables. */
f(f(f(f(x)))).
?-  f(f(f(f(x)))) = X.