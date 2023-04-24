use egg::{*};
use termion::{color};

// Represents all bit operations in Maki, 1:1 translation from synthesized netlist into a chain of SEQ and DEF statements
define_language! {
    enum Maki {
        "SEQ" = Sequence([Id; 2]),
        "DEF" = Define([Id; 2]),
        "LHS" = LDefine(Box<[Id]>),

        "HALF_ADD" = HalfAdder([Id; 2]),
        "FULL_ADD" = FullAdder([Id; 3]),
        "AND" = And([Id; 2]),
        "OR" = Or([Id; 2]),
        "NOT" = Not(Id),
        "XOR" = Xor([Id; 2]),
        "CONCAT" = Concat(Box<[Id]>),
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "MUX" = Mux([Id; 3]),
        "SELECT" = Select([Id; 2]),
        "OUT" = Out(Id),
        Num(i32),
        Symbol(Symbol),
    }
}

// Subset of Maki that represent wexps in the netlist
define_language! {
    enum MakiWexp {
        "HALF_ADD" = HalfAdder([Id; 2]),
        "FULL_ADD" = FullAdder([Id; 3]),
        "N_BIT_ADD" = NAdder(Box<[Id]>),
        "AND" = And([Id; 2]),
        "OR" = Or([Id; 2]),
        "NOT" = Not(Id),
        "XOR" = Xor([Id; 2]),
        "CONCAT" = Concat(Box<[Id]>), // Concat(Box<[Id]>),
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "MUX" = Mux([Id; 3]),
        "SELECT" = Select([Id; 2]),
        "OUT" = Out(Id),
        Num(i32),
        Symbol(Symbol),
    }
}

// examples:
// (SEQ (DEF (LHS tmp23) (AND a carry_in))
// (SEQ (DEF tmp18 (AND a b))
// (SEQ (DEF tmp17 (XOR a b))
// (SEQ (DEF tmp22 (AND b carry_in))
// (SEQ (DEF tmp19 (XOR tmp17 carry_in))
// (SEQ (DEF tmp20 (OR tmp18 tmp23))
// (SEQ (DEF sum tmp19))
// (SEQ (DEF tmp21 (OR tmp20 tmp22)))))))))))

// Rewrite rules
fn make_rules_maki() -> Vec<Rewrite<Maki, ()>> {
    let v = vec![
        rewrite!("comm_add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rewrite!("comm_and"; "(AND ?a ?b)" => "(AND ?b ?a)"),
        rewrite!("double_neg"; "(NOT (NOT ?a))" => "(?a)"),
        rewrite!("assoc_or"; "(OR ?a (OR ?b ?c))" => "(OR (OR ?a ?b) ?c)"),
        rewrite!("assoc_and"; "(AND ?a (AND ?b ?c))" => "(AND (AND ?a ?b) ?c)"),
        rewrite!("dist_and_or"; "(AND ?a (OR ?b ?c))" => "(OR (AND ?a ?b) (AND ?a ?c))"),
        rewrite!("dist_or_and"; "(OR ?a (AND ?b ?c))" => "(AND (OR ?a ?b) (OR ?a ?c))"),
        rewrite!("comm_or"; "(OR ?a ?b)" => "(OR ?b ?a)"),
        rewrite!("lem"; "(OR ?a (NOT ?a))" => "true"),
        rewrite!("and_identity"; "(AND ?a true)" => "?a"),
        rewrite!("or_identity"; "(OR ?a false)" => "?a"),
        rewrite!("and_kill"; "(AND ?a false)" => "false"),
        rewrite!("or_kill"; "(OR ?a true)" => "true"),
        rewrite!("and_idem"; "(AND ?a ?a)" => "?a"),
        rewrite!("or_idem"; "(OR ?a ?a)" => "?a"),
        rewrite!("absorb1"; "(OR ?a (AND ?a ?b))" => "?a"),
        rewrite!("absorb2"; "(AND ?a (OR ?a ?b))" => "?a"),

        rewrite!("and_define"; "(SEQ (DEF (LHS ?var) ?definition) (SEQ (DEF (LHS ?a) (AND ?var ?c)) ?b))" => "(SEQ (DEF (LHS ?var) ?definition) (SEQ (DEF (LHS ?a) (AND ?definition ?c)) ?b))"),
        // rewrite!("seq_sing_commut"; "(SEQ ?a ?b)" => "(SEQ ?b ?a)"), // look into conditional rewrites, storing global state, and associating values with e-classes, such as wire metadata
        rewrite!("seq_commut"; "(SEQ ?a (SEQ ?b ?c))" => "(SEQ ?b (SEQ ?a ?c))"),
        rewrite!("half_add"; "(SEQ (DEF (LHS ?s) (XOR ?a ?b)) (SEQ (DEF (LHS ?c) (AND ?a ?b)) ?d))" => "(SEQ (DEF (LHS ?s ?c) (HALF_ADD ?a ?b)) ?d)"),
        rewrite!("1_bit_add"; "(SEQ (DEF (LHS ?s1 ?c1) (HALF_ADD ?a ?b)) (SEQ (DEF (LHS ?s2 ?c2) (HALF_ADD ?s1 ?d)) (SEQ (DEF (LHS ?cout) (OR ?c1 ?c2)) ?e)))" => "(SEQ (DEF (LHS ?s2 ?cout) (FULL_ADD ?a ?b ?d)) ?e)"),
    ];

    v
}

fn make_rules_makiwexp() -> Vec<Rewrite<MakiWexp, ()>> {
    let mut v = vec![
        // rewrite!("comm_add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rewrite!("comm_and"; "(AND ?a ?b)" => "(AND ?b ?a)"),
        rewrite!("double_neg"; "(NOT (NOT ?a))" => "(?a)"),
        rewrite!("assoc_or"; "(OR ?a (OR ?b ?c))" => "(OR (OR ?a ?b) ?c)"),
        rewrite!("assoc_and"; "(AND ?a (AND ?b ?c))" => "(AND (AND ?a ?b) ?c)"),

        rewrite!("comm_or"; "(OR ?a ?b)" => "(OR ?b ?a)"),
        rewrite!("lem"; "(OR ?a (NOT ?a))" => "true"),
        rewrite!("and_identity"; "(AND ?a true)" => "?a"),
        rewrite!("or_identity"; "(OR ?a false)" => "?a"),
        rewrite!("and_kill"; "(AND ?a false)" => "false"),
        rewrite!("or_kill"; "(OR ?a true)" => "true"),
        rewrite!("and_idem"; "(AND ?a ?a)" => "?a"),
        rewrite!("or_idem"; "(OR ?a ?a)" => "?a"),
        rewrite!("absorb1"; "(OR ?a (AND ?a ?b))" => "?a"),
        rewrite!("absorb2"; "(AND ?a (OR ?a ?b))" => "?a"),
        rewrite!("comm_xor"; "(XOR ?a ?b)" => "(XOR ?b ?a)"),
        rewrite!("assoc_xor"; "(XOR ?a (XOR ?b ?c))" => "(XOR (XOR ?a ?b) ?c)"),
        rewrite!("xor_neg_identity"; "(XOR ?a true)" => "(NOT ?a)"),
        rewrite!("xor_identity"; "(XOR ?a false)" => "?a"),

        rewrite!("half_add"; "(CONCAT (AND ?a ?b) (XOR ?a ?b))" => "(HALF_ADD ?a ?b)"), // half add represents 2 vectors expressions
        // rewrite!("full_add_from_half"; "(CONCAT (AND ?a ?b) (XOR ?a ?b))" => "(FULL_ADD ?a ?b)"),
        rewrite!("full_add"; "(CONCAT (XOR (XOR ?a ?b) ?cin) (OR (OR (AND ?a ?b) (AND ?a ?cin)) (AND ?b ?cin)))" => "(FULL_ADD ?a ?b ?cin)"),
        rewrite!("comm_full_add"; "(FULL_ADD ?a ?b ?c)" => "(FULL_ADD ?b ?a ?c)"),
        rewrite!("comm_full_add2"; "(FULL_ADD ?a ?b ?c)" => "(FULL_ADD ?c ?b ?a)"),
        rewrite!("comm_full_add3"; "(FULL_ADD ?a ?b ?c)" => "(FULL_ADD ?a ?c ?b)"),
    ];

    v.extend(vec![
        rewrite!("xor_conv"; "(XOR ?a ?b)" <=> "(AND (OR ?a ?b) (NOT (AND ?a ?b)))"),
        rewrite!("xor_conv2"; "(XOR ?a ?b)" <=> "(OR (AND (NOT ?a) ?b) (AND ?a (NOT ?b)))"),
        rewrite!("xor_and_comm"; "(OR (AND ?a ?b) (AND ?c (XOR ?a ?b)))" <=> "(OR (AND ?a ?c) (AND ?b (XOR ?a ?c)))"),
        rewrite!("de_morgan"; "(NOT (AND ?a ?b))" <=> "(OR (NOT ?a) (NOT ?b))"),
        rewrite!("de_morgan2"; "(NOT (OR ?a ?b))" <=> "(AND (NOT ?a) (NOT ?b))"),
        rewrite!("dist_and_or"; "(AND ?a (OR ?b ?c))" <=> "(OR (AND ?a ?b) (AND ?a ?c))"),
        rewrite!("dist_or_and"; "(OR ?a (AND ?b ?c))" <=> "(AND (OR ?a ?b) (OR ?a ?c))"),
    ].concat());

    v
}

fn simplify_maki(s: &str) -> String {
    let expr: RecExpr<Maki> = s.parse().unwrap();

    // simplify the expression using a Runner, which creates an e-graph with
    // the given expression and runs the given rules over it
    let runner = Runner::default().with_expr(&expr).run(&make_rules_maki());

    // The Runner knows which e-class the expression given with `with_expr` is in
    let root = runner.roots[0]; // roots is the eclasses in the graph added with the initial with_expr method, (0) is the base

    // TODO: explore custom Extractors
    // use an Extractor to pick the best element of the root eclass
    // Extractor is a struct with the method new, creates it "constructor"
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best) = extractor.find_best(root);
    println!("Simplified {} with cost {}", expr, best_cost);
    best.to_string()
}

fn simplify_makiwexp(s: &str) -> String {
    let expr: RecExpr<MakiWexp> = s.parse().unwrap();
    let runner = Runner::default().with_expr(&expr).run(&make_rules_makiwexp());
    let root = runner.roots[0];

    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (_, best) = extractor.find_best(root);

    best.to_string()
}

fn compare_makiwexp(s1: &str, s2: &str, equal: bool) {
    let expr1: RecExpr<MakiWexp> = s1.parse().unwrap();
    let expr2: RecExpr<MakiWexp> = s2.parse().unwrap();

    let mut runner = Runner::default().with_expr(&expr1).run(&make_rules_makiwexp());
    let root = runner.roots[0];
    
    // instead of providing an extractor, see if the equivalent wexp belongs in the same root eclass
    let e_id = runner.egraph.add_expr(&expr2); // Adds an entire expression, not just an enode

    // Canonicalizes eclass Id - AKA convert data that has more than one representation into a "standard" format
    // runner.roots are the Ids of the initial expr in the runner
    if equal {
        assert_eq!(runner.egraph.find(runner.roots[0]), runner.egraph.find(e_id)); // Check if enodes are in same eclass
    } else {
        assert_ne!(runner.egraph.find(runner.roots[0]), runner.egraph.find(e_id)); // Check if enodes are in same eclass
    }
    
    println!("ID, ca: {} {}", runner.roots[0], runner.egraph.find(runner.roots[0])); // runner.roots[0], runner.egraph.find(runner.roots[0])
}

// ================= tests for rewriting Maki with DEF and SEQ ===================
#[test]
fn half_add() {
    println!("{}", simplify_maki("(SEQ (DEF (LHS s) (AND a b)) (SEQ (DEF (LHS c) (XOR a b)) (OUT s)))"));
    println!("{}", simplify_maki("(SEQ (DEF (LHS s) (XOR a b)) (SEQ (DEF (LHS c) (AND a b)) (OUT s)))"));
}

#[test]
fn full_add_basic() {
    println!("{}", simplify_maki("(SEQ (DEF (LHS s cout) (HALF_ADD a b)) (SEQ (DEF (LHS s1 cout1) (HALF_ADD s cin)) (SEQ (DEF (LHS cout2) (OR cout cout1)) (AND d e))))"));
    println!("{}", simplify_maki("(SEQ (DEF (LHS s cout) (HALF_ADD a b)) (SEQ (DEF (LHS s1 cout1) (HALF_ADD s cin)) (SEQ (AND d e) (DEF (LHS cout2) (OR cout cout1)))))"));
}

#[test]
fn basic_to_full_add() {
    // Concern: where would the half-add fit into the tree? wire-expressions involve wire-expression operands, but the result of half-add is 2 separate wire-expressions
    // Example: (AND (OR a b) c) makes sense, but not (AND (HALF_ADD a b) c), since (HALF_ADD a b) generates two wire vectors
    // Concern: each circuit might not be properly represented as a single tree: (XOR a b) (AND a b)
    println!("{}", simplify_maki("(SEQ (DEF (LHS cout) (AND a b)) (SEQ (DEF (LHS sum) (XOR a b)) (SEQ (DEF (LHS cout1) (AND sum cin)) (SEQ (DEF (LHS sum1) (XOR sum cin)) (SEQ (DEF (LHS cout2) (OR cout1 cout)) (SEQ (OUT sum1) (OUT cout2)))))))"));
}

#[test]
fn ripple_carry() {
    println!("{}", simplify_maki("(SEQ (DEF (LHS tmp17) (SELECT a 0)) (SEQ (DEF (LHS tmp18) (SELECT a 1)) (SEQ (DEF (LHS tmp15) (SELECT b 0)) (SEQ (DEF (LHS tmp16) (SELECT b 1)) (SEQ (DEF (LHS tmp32) (AND tmp17 carry_in)) (SEQ (DEF (LHS tmp24) (AND tmp15 carry_in)) (SEQ (DEF (LHS tmp26) (AND tmp17 tmp15)) (SEQ (DEF (LHS tmp20) (XOR tmp17 tmp15)) (SEQ (DEF (LHS tmp21) (XOR tmp18 tmp16)) (SEQ (DEF (LHS tmp30) (XOR tmp20 carry_in)) (SEQ (DEF (LHS tmp28) (OR tmp26 tmp32)) (SEQ (DEF (LHS tmp22) (OR tmp28 tmp24)) (SEQ (DEF (LHS carry_out) tmp22) (SEQ (DEF (LHS tmp19) (CONCAT tmp21 tmp30)) (DEF (LHS sum) tmp19)))))))))))))))"));
}
// =================================================================================

// ======================== tests for rewriting wexps ==============================
#[test]
fn compare_bitlogic() {
    compare_makiwexp("(NOT (NOT true))", "true", true);
    compare_makiwexp("(AND x y)", "(AND y x)", true);
    compare_makiwexp("(NOT (AND y x))", "(OR (NOT x) (NOT y))", true);
    compare_makiwexp("(NOT (NOT (AND x y)))", "(AND y x)", true);
    // compare_makiwexp("(AND y x)", "(NOT (NOT (AND x y)))");
    compare_makiwexp("(NOT (NOT (AND x y)))", "(NOT (OR (NOT y) (NOT x)))", true);
    compare_makiwexp("(NOT (NOT (AND x y)))", "(AND (NOT (NOT y)) (NOT (NOT x)))", true);
    compare_makiwexp("(NOT (AND x y))", "(OR (NOT y) (NOT x))", true);
    compare_makiwexp("(AND (NOT x) (NOT y))", "(NOT (OR y x))", true);

    compare_makiwexp("(XOR (XOR a b) c)", "(XOR (XOR c a) b)", true);
}

fn main() {
    // sanity check
    println!("{}", simplify_makiwexp("(NOT (NOT true))"));
    // test_makiwexp();
}

#[test]
fn test_full_add() {
    // basic tests, single full adders
    let full_add1 = "(CONCAT (XOR (XOR a b) cin) (OR (OR (AND a b) (AND a cin)) (AND b cin)))";
    let full_add2 = "(CONCAT (XOR (XOR a b) cin) (OR (AND a b) (AND cin (XOR a b))))";
    let full_add3 = "(CONCAT (XOR (XOR b a) cin) (OR (AND b a) (AND cin (XOR b a))))";
    let full_add4 = "(CONCAT (XOR (XOR cin b) a) (OR (OR (AND cin b) (AND a cin)) (AND b a))))";
    let full_add5 = "(CONCAT (XOR cin (XOR b a)) (OR (AND b a) (AND cin (XOR a b))))";
    let full_add6 = "(CONCAT (XOR (XOR cin a) b) (OR (AND a cin) (AND b (XOR a cin))))";

    compare_makiwexp("(OR (AND a cin) (AND b (XOR a cin)))", "(AND (OR b (AND a cin)) (OR (AND a cin) (XOR a cin)))", true);
    compare_makiwexp("(AND (OR b (AND a cin)) (OR (AND a cin) (XOR a cin)))", "(OR (AND (OR (AND a cin) b) (AND a cin)) (AND (OR (AND a cin) b) (XOR a cin)))", true);

    compare_makiwexp(full_add1, full_add4, true);
    compare_makiwexp(full_add6, full_add1, true);
    compare_makiwexp(full_add1, full_add2, false); // false because rewrites are unidirectional - when equivalence classes for full_add1 gets saturated, they don't encompass full_add2
    compare_makiwexp(full_add2, full_add1, true); // when equivalence classes for full_add2 gets saturated, they encompass full_add1
    compare_makiwexp(full_add2, full_add3, true);
    compare_makiwexp(full_add5, full_add3, true);
    compare_makiwexp(full_add5, full_add6, true);
    compare_makiwexp(full_add6, full_add5, true);

    // TODO: figure out why the following fails
    compare_makiwexp("(OR (AND b a) (AND cin (XOR a b)))", "(OR (AND a cin) (AND b (XOR a cin)))", true);
    // compare_makiwexp(full_add6, full_add5, true);
    
    println!("{}", simplify_makiwexp(full_add1));
    println!("{}", simplify_makiwexp(full_add2));
    println!("{}", simplify_makiwexp(full_add3));
    println!("{}", simplify_makiwexp(full_add4));
    println!("{}", simplify_makiwexp(full_add5));
    println!("{}", simplify_makiwexp(full_add6));

    // tentative: the following won't simplify to a full-adder because the order of the sum and carry out bits are switched
    // reasoning: CONCAT isn't commutative
    println!("{}", simplify_makiwexp("(CONCAT (OR (AND a b) (AND cin (XOR a b))) (XOR (XOR a b) cin))"));
    
    // multiple full adders
    println!("{}", simplify_makiwexp("(CONCAT (CONCAT (XOR (XOR a b) cin) (OR (AND a b) (AND cin (XOR a b)))) (CONCAT (XOR (XOR a b) cin) (OR (OR (AND a b) (AND a cin)) (AND b cin))))"));
    println!("{}", simplify_makiwexp("(OR (CONCAT (XOR (XOR c d) cin) (OR (AND c d) (AND cin (XOR c d)))) (CONCAT (XOR (XOR a b) cin) (OR (OR (AND a b) (AND a cin)) (AND b cin))))"));
}

#[test]
fn test_full_add_adv() {
    compare_makiwexp("(FULL_ADD a b c)", "(FULL_ADD b a c)", true);
    compare_makiwexp("(FULL_ADD a b c)", "(FULL_ADD a c b)", true);
}

