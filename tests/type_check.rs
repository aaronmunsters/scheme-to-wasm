use im_rc::vector;
use scheme_to_wasm::common::{ExprKind, TypeEnv};
use scheme_to_wasm::parse::{parse, parse_type};
use scheme_to_wasm::type_check::{tc_with_env, type_check};
use scheme_to_wasm::types::Type;

#[test]
fn test_typecheck_prims() {
    let exp = lexpr::from_str("3").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str("-497").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str("#t").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Bool);

    let exp = lexpr::from_str("#f").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Bool);

    let exp = lexpr::from_str("true").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Bool);

    let exp = lexpr::from_str("false").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Bool);

    let exp = lexpr::from_str("\"true\"").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Str);

    let exp = lexpr::from_str("\"foo\"").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Str);

    let exp = lexpr::from_str("\"\"").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Str);
}

#[test]
fn test_typecheck_binops_happy() {
    let exp = lexpr::from_str("(+ 3 5)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str("(* 3 5)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str("(- 3 5)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str("(/ 3 5)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str("(+ (* 4 5) (- 5 2))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str(r#"(concat "hello " "world")"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Str);

    let exp = lexpr::from_str("(and true false)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Bool);

    let exp = lexpr::from_str("(or true false)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Bool);
}

#[test]
fn test_typecheck_all_nodes_annotated() {
    let exp = lexpr::from_str("(> 3 5)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Bool);
    match *typed_exp.kind {
        ExprKind::Binop(_op, arg1, arg2) => {
            assert_eq!(arg1.typ, Type::Int);
            assert_eq!(arg2.typ, Type::Int);
        }
        _ => panic!("Non-binop found!"),
    }
}

#[test]
fn test_typecheck_binops_sad() {
    let exp = lexpr::from_str("(+ 3 true)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    let exp = lexpr::from_str("(* 3 true)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    let exp = lexpr::from_str(r#"(- false "hello")"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    let exp = lexpr::from_str(r#"(/ "foo" 3)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    let exp = lexpr::from_str(r#"(concat 3 "world")"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    let exp = lexpr::from_str(r#"(and 3 6)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    let exp = lexpr::from_str(r#"(or "hello" "world")"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());
}

#[test]
fn test_typecheck_lists_happy() {
    let exp = lexpr::from_str("(null int)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::List(Box::new(Type::Int)));

    let exp = lexpr::from_str("(null bool)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::List(Box::new(Type::Bool)));

    let exp = lexpr::from_str("(cons 3 (null int))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::List(Box::new(Type::Int)));

    let exp = lexpr::from_str("(cons 3 (cons 4 (null int)))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::List(Box::new(Type::Int)));

    let exp = lexpr::from_str(r#"(cons "foo" (cons "bar" (null string)))"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::List(Box::new(Type::Str)));

    // Note we define the types of (car lst) and (cdr lst) uniformly;
    // i.e. the types are the same even if the lists are empty.
    let exp = lexpr::from_str("(car (null int))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str("(cdr (null int))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::List(Box::new(Type::Int)));

    let exp = lexpr::from_str("(car (cons 3 (null int)))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str("(cdr (cons 3 (null int)))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::List(Box::new(Type::Int)));

    let exp = lexpr::from_str("(null? (null int))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Bool);

    // we could change the semantics to be more restrictive, but AFAIK
    // it is okay if we let null take any value in our language as input
    let exp = lexpr::from_str("(null? 3)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Bool);
}

#[test]
fn test_typecheck_lists_sad() {
    // type of car does not match type of cdr
    let exp = lexpr::from_str(r#"(cons "hey" (null int))"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // invalid argument to car
    let exp = lexpr::from_str("(car 3)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // invalid argument to cdr
    let exp = lexpr::from_str("(cdr 3)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());
}

#[test]
fn test_typecheck_tuples_happy() {
    let exp = lexpr::from_str(r#"(make-tuple)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Tuple(vector![]));

    let exp = lexpr::from_str(r#"(make-tuple 3 "hello")"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Tuple(vector![Type::Int, Type::Str]));

    let exp = lexpr::from_str(r#"(tuple-ref (make-tuple 3 "hello") 0)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str(r#"(tuple-ref (make-tuple 3 "hello") 1)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Str);
}

#[test]
fn test_typecheck_tuples_sad() {
    // key too large
    let exp = lexpr::from_str(r#"(tuple-ref (make-tuple 3 "hello") 2)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // key is not a number
    let exp = lexpr::from_str(r#"(tuple-ref (make-tuple 3 "hello") true)"#).unwrap();
    let parsed = parse(&exp);
    assert!(parsed.is_err());

    // first expression is not a tuple
    let exp = lexpr::from_str(r#"(tuple-ref (cons 3 (null int)) 0)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // tuple is empty, so no tuple-ref should be valid
    let exp = lexpr::from_str(r#"(tuple-ref (make-tuple) 0)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());
}

#[test]
fn test_typecheck_records_happy() {
    let exp = lexpr::from_str(r#"(make-record)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Record(vector![]));

    let exp = lexpr::from_str(r#"(make-record (num 3) (name "hello"))"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(
        typed_exp.typ,
        Type::Record(vector![
            (String::from("num"), Type::Int),
            (String::from("name"), Type::Str)
        ])
    );

    let exp = lexpr::from_str(r#"(let ((bar 3)) (make-record (foo bar)))"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(
        typed_exp.typ,
        Type::Record(vector![(String::from("foo"), Type::Int)])
    );

    let exp = lexpr::from_str(r#"(record-ref (make-record (num 3) (name "hello")) num)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str(r#"(record-ref (make-record (num 3) (name "hello")) name)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Str);

    let exp = lexpr::from_str(r#"(let ((a (make-record (b 3)))) (record-ref a b))"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);
}

#[test]
fn test_typecheck_records_sad() {
    // we don't know what type bar is
    let exp = lexpr::from_str(r#"(make-record (foo bar))"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // TODO: In practice, the compiler should have some kind of check to ensure
    // records are not being constructed with multiple fields of the same name,
    // (unless perhaps there were macros or something), but this isn't a
    // high priority feature.

    // // record contains duplicate values
    // let exp = lexpr::from_str(r#"(make-record (num 3) (num 4))"#).unwrap();
    // let typed_exp = type_check(&parse(&exp).unwrap());
    // assert_eq!(typed_exp.is_err(), true);

    // // record contains duplicate values (of different types)
    // let exp = lexpr::from_str(r#"(make-record (num 3) (num "hello"))"#).unwrap();
    // let typed_exp = type_check(&parse(&exp).unwrap());
    // assert_eq!(typed_exp.is_err(), true);

    // invalid key on record-ref
    let exp = lexpr::from_str(r#"(record-ref (make-record (num 3) (name "hello")) foo)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // non-record expression passed to record-ref
    let exp = lexpr::from_str(r#"(record-ref "hello" foo)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());
}

#[test]
fn test_typecheck_let_happy() {
    let exp = lexpr::from_str("(let ((x 23)) (+ x 24))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str("(let ((x 3) (y 5)) (+ x y))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);
}

#[test]
fn test_typecheck_let_sad() {
    // one variable missing
    let exp = lexpr::from_str("(let ((x 23)) (+ x y))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());
}

#[test]
fn test_typecheck_sideeffects_happy() {
    let exp = lexpr::from_str("(begin (+ 3 5))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str("(begin (+ 3 5) (- 4 1))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str("(let ((x 3)) (set! x 7))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);
}

#[test]
fn test_typecheck_sideeffects_sad() {
    // intermediary expression in begin is not valid
    let exp = lexpr::from_str(r#"(begin (+ 3 "hello") (- 4 1))"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // last expression in begin is not valid
    let exp = lexpr::from_str(r#"(begin (+ 3 4) (- 4 "hello"))"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // using set! before variable is defined
    let exp = lexpr::from_str("(set! x 7)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());
}

#[test]
fn test_typecheck_local_scoping() {
    // local variable overrides outer variable
    // TODO: consider making this behavior disallowed
    let exp = lexpr::from_str(
        r#"(let ((x "hello"))
                (let ((x 23))
                    (+ x 24)))"#,
    )
    .unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    // binding from let lasts past its scope
    let exp = lexpr::from_str("(+ (let ((x 5)) (+ x 3)) x)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // binding from lambda lasts past its scope
    let exp = lexpr::from_str("(+ ((lambda ((x : int)) : int x) 3) x)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // let bindings with same names of conflicting types
    let exp = lexpr::from_str("(and (let ((x 5)) (< x 3)) (let ((x false)) (or x true)))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Bool);

    // lambda bindings with same names of conflicting types
    let exp =
        lexpr::from_str("(and ((lambda ((x : int)) : bool (< x 3)) 5) ((lambda ((x : bool)) : bool (and x true)) false))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Bool);

    // using set! after variable goes out of scope
    let exp = lexpr::from_str("(begin (let ((x 3)) (+ x 5)) (set! x 7))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());
}

#[test]
fn test_test_typecheck_if_happy() {
    let exp = lexpr::from_str(r#"(if (< 3 4) 1 -1)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);
}

#[test]
fn test_typecheck_if_sad() {
    // invalid predicate
    let exp = lexpr::from_str(r#"(if 3 4 5)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // consequent and alternate do not match
    let exp = lexpr::from_str(r#"(if (< 3 4) "hello" 5)"#).unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());
}

#[test]
fn test_typecheck_lambda_happy() {
    let exp = lexpr::from_str("(lambda () : int 3)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Func(vector![], Box::new(Type::Int)));

    let exp = lexpr::from_str("(lambda ((x : int)) : bool (< x 5))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(
        typed_exp.typ,
        Type::Func(vector![Type::Int], Box::new(Type::Bool))
    );

    let exp = lexpr::from_str("(lambda ((x : int) (y : int)) : int (* x y))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(
        typed_exp.typ,
        Type::Func(vector![Type::Int, Type::Int], Box::new(Type::Int))
    );

    let exp =
        lexpr::from_str("(lambda ((fn : (-> int int bool)) (x : int) (y : int)) : bool (fn x y))")
            .unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(
        typed_exp.typ,
        Type::Func(
            vector![
                Type::Func(vector![Type::Int, Type::Int], Box::new(Type::Bool)),
                Type::Int,
                Type::Int
            ],
            Box::new(Type::Bool)
        )
    );
}

#[test]
fn test_typecheck_nested_lambdas() {
    let exp = parse(
        &lexpr::from_str(
            "(let ((a 3))
  (lambda ((f : (-> int int))) : (-> int)
    (lambda () : int (f a))))",
        )
        .unwrap(),
    )
    .unwrap();
    let typed_exp = type_check(&exp).unwrap();
    let expected = parse_type(&lexpr::from_str("(-> (-> int int) (-> int))").unwrap()).unwrap();
    assert_eq!(typed_exp.typ, expected);

    let exp = parse(
        &lexpr::from_str(
            r#"(let ((a 3))
  (lambda ((f : (-> int int int int))) : (-> int (-> int int))
     (lambda ((z : int)) : (-> int int)
       (lambda ((x : int)) : int
         (f x z a)))))"#,
        )
        .unwrap(),
    )
    .unwrap();
    let expected =
        parse_type(&lexpr::from_str("(-> (-> int int int int) (-> int (-> int int)))").unwrap())
            .unwrap();
    let typed_exp = type_check(&exp).unwrap();
    assert_eq!(typed_exp.typ, expected);

    let exp = parse(
        &lexpr::from_str(
            "(let ((f (lambda ((x : int)) : (-> int int)
           (lambda ((y : int)) : int (+ x y)))))
  ((f 4) 3))",
        )
        .unwrap(),
    )
    .unwrap();
    let typed_exp = type_check(&exp).unwrap();
    let expected = parse_type(&lexpr::from_str("int").unwrap()).unwrap();
    assert_eq!(typed_exp.typ, expected);
}

#[test]
fn test_typecheck_lambdas_recursive_happy() {
    // Not a very clean way to implement recursive functions.
    // Currently creates a dummy function (so that the function will have a name)
    // and then sets the name to the actual function definition, with the
    // name available for use.
    // Consider adding a named-lambda or fixed point operator.
    let exp = lexpr::from_str(
        r#"(let ((foo (lambda ((x : int)) : int 0)))
    (set! foo (lambda ((x : int)) : int (if (< x 1) 0 (+ 1 (foo (- x 1)))))))"#,
    )
    .unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(
        typed_exp.typ,
        Type::Func(vector![Type::Int], Box::new(Type::Int))
    );
}

#[test]
fn test_typecheck_lambda_sad() {
    // mismatched return type
    let exp = lexpr::from_str("(lambda () : bool 3)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // input types do not work in body
    let exp = lexpr::from_str("(lambda ((x : bool) (y : bool)) : int (+ x y))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());
}

#[test]
fn test_typecheck_apply_happy() {
    let exp = lexpr::from_str("((lambda () : int 3))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str("((lambda ((x : int)) : bool (< x 5)) 3)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Bool);

    let exp = lexpr::from_str("((lambda ((x : int) (y : int)) : int (* x y)) 5 6)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);

    let exp = lexpr::from_str("((lambda ((x : int) (y : int)) : int (* x y)) 5 6)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Int);
}

#[test]
fn test_typecheck_apply_hof_happy() {
    // Note: this is basically (apply (lambda equivalent to <) 3 5)
    let exp = lexpr::from_str(
        "
    ((lambda ((fn : (-> int int bool)) (x : int) (y : int)) : bool
             (fn x y))
     (lambda ((a : int) (b : int)) : bool (< a b))
             3 5)",
    )
    .unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    assert_eq!(typed_exp.typ, Type::Bool);

    // "map" is recursive, so without implementing type-checking for define,
    // (which would create the binding from "map" to its type signature)
    // we must add the name to the environment for the unit test
    let exp = lexpr::from_str(
        "(lambda ((f : (-> int int)) (lst : (list int))) : (list int)
            (if (null? lst)
                (null int)
                (cons (f (car lst)) (map f (cdr lst)))))",
    )
    .unwrap();
    let map_type = Type::Func(
        vector![
            Type::Func(vector![Type::Int], Box::new(Type::Int)),
            Type::List(Box::new(Type::Int)),
        ],
        Box::new(Type::List(Box::new(Type::Int))),
    ); // (-> (-> int int) (list int) (list int))
    let mut env = TypeEnv::new();
    env = env.add_binding((String::from("map"), map_type.clone()));
    let typed_exp = tc_with_env(&parse(&exp).unwrap(), &env).unwrap();
    assert_eq!(typed_exp.typ, map_type);
}

#[test]
fn test_typecheck_apply_sad() {
    // missing parameters
    let exp = lexpr::from_str("((lambda ((x : int)) : bool (< x 5)))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // too many parameters
    let exp = lexpr::from_str("((lambda ((x : int)) : bool (< x 5)) 3 5)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // arg type does not match param type
    let exp = lexpr::from_str("((lambda ((x : int)) : bool (< x 5)) true)").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());
}

#[test]
fn test_typecheck_pack_happy() {
    // show that multiple types can inhabit an existential type
    let exp = &lexpr::from_str("(pack 3 int (exists T0 T0))").unwrap();
    let typed_exp = type_check(&parse(exp).unwrap()).unwrap();
    let typ = parse_type(&lexpr::from_str("(exists T0 T0)").unwrap()).unwrap();
    assert_eq!(typed_exp.typ, typ);

    let exp = &lexpr::from_str("(pack true bool (exists T0 T0))").unwrap();
    let typed_exp = type_check(&parse(exp).unwrap()).unwrap();
    let typ = parse_type(&lexpr::from_str("(exists T0 T0)").unwrap()).unwrap();
    assert_eq!(typed_exp.typ, typ);

    // function
    let exp =
        lexpr::from_str("(pack (lambda ((x : int)) : int (+ x 1)) int (exists T0 (-> T0 T0)))")
            .unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    let typ = parse_type(&lexpr::from_str("(exists T0 (-> T0 T0))").unwrap()).unwrap();
    assert_eq!(typed_exp.typ, typ);

    // slightly more specific type ascription
    let exp =
        lexpr::from_str("(pack (lambda ((x : int)) : int (+ x 1)) int (exists T0 (-> T0 int)))")
            .unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    let typ = parse_type(&lexpr::from_str("(exists T0 (-> T0 int))").unwrap()).unwrap();
    assert_eq!(typed_exp.typ, typ);

    // packing a record expression
    let exp = lexpr::from_str(
        r#"(pack (make-record (a 0)
                            (f (lambda ((x : int)) : int (+ 1 x))))
               int
               (exists T0 (record (a : T0) (f : (-> T0 int)))))"#,
    )
    .unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    let typ =
        parse_type(&lexpr::from_str("(exists T0 (record (a : T0) (f : (-> T0 int))))").unwrap())
            .unwrap();
    assert_eq!(typed_exp.typ, typ);
}

#[test]
fn test_typecheck_pack_sad() {
    let exp = lexpr::from_str("(pack true int (exists T0 T0))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    let exp = lexpr::from_str("(pack 3 int (exists T0 bool))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    let exp =
        lexpr::from_str("(pack (lambda ((x : int)) : bool 3) int (exists T0 (-> T0 T0)))").unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());
}

#[test]
fn test_typecheck_unpack_happy() {
    let exp = lexpr::from_str(
        r#"
            (let ((p (pack (make-record (a 0)
                                (f (lambda ((x : int)) : int
                                     (+ 1 x))))
                   int
                   (exists T0 (record (a : T0)
                                      (f : (-> T0 int)))))))
      (unpack (q p T0)
              ((record-ref q f) (record-ref q a))))
                    "#,
    )
    .unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    let typ = parse_type(&lexpr::from_str("int").unwrap()).unwrap();
    assert_eq!(typed_exp.typ, typ);

    let exp = lexpr::from_str(
        r#"(let ((p (pack (make-record (a 0)
                            (f (lambda ((x : int)) : int
                                 (+ 1 x))))
               int
               (exists T0 (record (a : T0)
                                  (f : (-> T0 int)))))))
  (unpack (q p T2)
          ((lambda ((y : T2)) : int ((record-ref q f) y))
           (record-ref q a))))"#,
    )
    .unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    let typ = parse_type(&lexpr::from_str("int").unwrap()).unwrap();
    assert_eq!(typed_exp.typ, typ);
}

#[test]
fn test_typecheck_unpack_sad() {
    // q.a is not guaranteed to be a number in the existential type,
    // so we cannot use it (and add to it) concretely as such
    let exp = lexpr::from_str(
        r#"
            (let ((p (pack (make-record (a 0)
                                (f (lambda ((x : int)) : int
                                     (+ 1 x))))
                   int
                   (exists T0 (record (a : T0)
                                      (f : (-> T0 int)))))))
      (unpack (q p T0)
              (+ (record-ref q a) 1)))
                    "#,
    )
    .unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());

    // the result type cannot have the type variable free
    let exp = lexpr::from_str(
        r#"
            (let ((p (pack (make-record (a 0)
                                (f (lambda ((x : int)) : int
                                     (+ 1 x))))
                   int
                   (exists T0 (record (a : T0)
                                      (f : (-> T0 int)))))))
      (unpack (q p T0)
              (record-ref q a)))
                    "#,
    )
    .unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap());
    assert!(typed_exp.is_err());
}

#[test]
fn test_typecheck_adt() {
    let exp = lexpr::from_str(
        "(pack (make-record (new 1)
                   (get (lambda ((i : int)) : int i))
                   (inc (lambda ((i : int)) : int (+ i 1))))
      int
      (exists T0 (record (new : T0)
                         (get : (-> T0 int))
                         (inc : (-> T0 T0)))))",
    )
    .unwrap();
    let typed_exp = type_check(&parse(&exp).unwrap()).unwrap();
    let typ = parse_type(
        &lexpr::from_str(
            "(exists T0 (record (new : T0)
                         (get : (-> T0 int))
                         (inc : (-> T0 T0))))",
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(typed_exp.typ, typ);
}
