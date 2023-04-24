/* Base equality facts (no variables), in all the egraphs */
wag(wag(wag)) = x.
z = x.
z = p.

/* Base egraph insertions. - (idk if 3 unrelated eclasses or 3 separate egraphs?) No implied equalities. For "seeding" the egraph. */
plus(p,x).
bar(boo).
fizzy(floozy).
buppo(dog).
biz(x).

/* general looking prolog-like rules; inserts things into the egraph */
/* bar(X) is added to egraph if f(X) rewrites to w (same e-class) */
bar(X) :- f(X) = w.
/* if bar(X) is true, biz(X) is true, but unknown if bar(X) != biz(X) */
biz(X) :- bar(X).
/* biz(Z) and baz(biz(boz)) are added to egraph in the same e-class if fizzy(floozy) and buppo(Z) are in the e-graph */
biz(Z) = baz(biz(boz)) :- fizzy(floozy), buppo(Z).

/* Rewrite rules. Variables denoted by capitalization */
plus(X,Y) <- plus(Y,X).

/* In principle syntactic shorthand for plus(X,Y) = C :- plus(Y,X) = C. */
/* Rewrites have synergy */
X <- f(f(f(X))).
X <- f(f(X)).
/* implies X <- f(X) */

/* bidirectional rewrite. A useful syntactic shorthand for two rewrite rules. */ 
plus(X,plus(Y,Z)) <-> plus(plus(X,Y),Z).

/* Guarded rewrite. */
/* bar(boo), x rewrites to z implies fiz(baz) */
fiz(baz) <- bar(boo), x = z.

/* Queries
Note that this does NOT insert into the egraph. Should I change that? Or give a new notation for "insert all subterms and then query"?
 */
/* We are querying the egraph */
/* equals means in equivalent eclasses in the egraph */
/* X. can be many things, it looks like egglog extracts the smallest syntax tree (fewest enodes) from the e-class*/
?- f(x) = X.
?- x = x.
?- w = x.

/* needs to be in the initial egraph for query to work */
/* translation: plus p r is in the same eclass as plus r p in the egraph */
?- plus(p,r) = plus(r,p).
/* translation: plus p x is in the egraph */
?- plus(p,x).

?- junk(boo) = otherjunk(baz).
?- bar(boo) = fiz(baz).

/* important: bar(X) :- biz(X) doesn't imply bar(boo) = biz(boo) */
?- bar(boo) = biz(boo).
?- biz(dog).
?- baz(biz(boz)).

/* bar(x) evaluates to unknown because bar(x) isn't inserted in the egraph at the beginning or can be deduced from prolog rules */
?- bar(x).

?- biz(x).
?- bar(w).

/* Query with variables. */
f(f(f(f(x)))).
?-  f(f(f(f(x)))) = X.