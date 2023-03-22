use egg::{*};
use termion::{color};

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
        "CONST" = Const([Id; 2]),
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

// (DEF A B (HALF_ADD C D))
// ("half_add"; "(SEQ (DEF (LHS ?s) (XOR ?a ?b)) (SEQ (DEF (LHS ?c) (AND ?a ?b)) ?d))" => "(SEQ (DEF (LHS ?s ?c) (HALF_ADD ?a ?b)))")


fn make_rules() -> Vec<Rewrite<Maki, ()>> {
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
        rewrite!("seq_sing_commut"; "(SEQ ?a ?b)" => "(SEQ ?b ?a)"), // look into conditional rewrites
        rewrite!("seq_commut"; "(SEQ ?a (SEQ ?b ?c))" => "(SEQ ?b (SEQ ?a ?c))"),
        rewrite!("half_add"; "(SEQ (DEF (LHS ?s) (XOR ?a ?b)) (SEQ (DEF (LHS ?c) (AND ?a ?b)) ?d))" => "(SEQ (DEF (LHS ?s ?c) (HALF_ADD ?a ?b)) ?d)"),
        rewrite!("1_bit_add"; "(SEQ (DEF (LHS ?s1 ?c1) (HALF_ADD ?a ?b)) (SEQ (DEF (LHS ?s2 ?c2) (HALF_ADD ?s1 ?d)) (SEQ (DEF (LHS ?cout) (OR ?c1 ?c2)) ?e)))" => "(SEQ (DEF (LHS ?s2 ?cout) (FULL_ADD ?a ?b ?d)) ?e)"),
    ];

    v
}

fn simplify(s: &str) -> String {
    // parse the expression, the type annotation tells it which Language to use
    // recursive expression from a user defined language
    let expr: RecExpr<Maki> = s.parse().unwrap();

    // simplify the expression using a Runner, which creates an e-graph with
    // the given expression and runs the given rules over it
    let runner = Runner::default().with_expr(&expr).run(&make_rules());

    // the Runner knows which e-class the expression given with `with_expr` is in
    let root = runner.roots[0]; // roots is the eclass in the egraph (0) is the original

    // use an Extractor to pick the best element of the root eclass
    // extractor is a struct with the method new, creates it "constructor"
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best) = extractor.find_best(root);
    println!("Simplified {} to {} with cost {}", expr, best, best_cost);
    best.to_string()
}

#[test]
fn half_add() {
    // println!("{}", simplify("(SEQ (DEF (LHS s) (AND a b)) (SEQ (DEF (LHS c) (XOR a b)) (OUT s)))"));
    // println!("{}", simplify("(SEQ (DEF (LHS s) (XOR a b)) (SEQ (DEF (LHS c) (AND a b)) (OUT s)))"));
    // println!("{}", simplify("(SEQ (DEF (LHS s cout) (HALF_ADD a b)) (SEQ (DEF (LHS s1 cout1) (HALF_ADD s cin)) (SEQ (DEF (LHS cout2) (OR cout cout1)) (AND d e))))"));
    // println!("{}", simplify("(SEQ a b)"))
}

fn main() {
    println!("{}", simplify("(NOT (NOT true))"));
}
