use scheme_to_wasm::parse::parse;
use scheme_to_wasm::record_elim::record_elim_exp;
use scheme_to_wasm::type_check::type_check;

#[test]
fn test_record_elim_simple() {
    let exp = parse(&lexpr::from_str("(make-record (bar 3) (foo \"hello\"))").unwrap()).unwrap();
    let typed_exp = type_check(&exp).unwrap();

    let expected_exp =
        type_check(&parse(&lexpr::from_str("(make-tuple 3 \"hello\")").unwrap()).unwrap()).unwrap();
    let rc_exp = record_elim_exp(&typed_exp).unwrap();

    println!("Source: {exp}");
    println!("Record elimination: {rc_exp}");
    assert_eq!(rc_exp, expected_exp);
}

#[test]
fn test_record_elim_order_invariant() {
    let exp = parse(&lexpr::from_str("(make-record (foo \"hello\") (bar 3))").unwrap()).unwrap();
    let typed_exp = type_check(&exp).unwrap();

    let expected_exp =
        type_check(&parse(&lexpr::from_str("(make-tuple 3 \"hello\")").unwrap()).unwrap()).unwrap();
    let rc_exp = record_elim_exp(&typed_exp).unwrap();

    println!("Source: {exp}");
    println!("Record elimination: {rc_exp}");
    assert_eq!(rc_exp, expected_exp);
}

#[test]
fn test_record_elim_record_get() {
    let exp =
        parse(&lexpr::from_str("(record-ref (make-record (foo \"hello\") (bar 3)) foo)").unwrap())
            .unwrap();
    let typed_exp = type_check(&exp).unwrap();

    let expected_exp = type_check(
        &parse(&lexpr::from_str("(tuple-ref (make-tuple 3 \"hello\") 1)").unwrap()).unwrap(),
    )
    .unwrap();
    let rc_exp = record_elim_exp(&typed_exp).unwrap();

    println!("Source: {exp}");
    println!("Record elimination: {rc_exp}");
    assert_eq!(rc_exp, expected_exp);
}

#[test]
fn test_record_elim_complex() {
    let exp = parse(
        &lexpr::from_str(
            r#"(let ((y 3))
        (pack (make-tuple
               (lambda ((env0 : (record (y : int)))
                        (x : int)) : int
                 (+ x (record-ref env0 y)))
               (make-record (y y)))
              (record (y : int))
              (exists T1 (tuple (-> T1 int int) T1))))"#,
        )
        .unwrap(),
    )
    .unwrap();
    let typed_exp = type_check(&exp).unwrap();

    let expected_exp = type_check(
        &parse(
            &lexpr::from_str(
                r#"(let ((y 3))
        (pack (make-tuple
               (lambda ((env0 : (tuple int))
                        (x : int)) : int
                 (+ x (tuple-ref env0 0)))
               (make-tuple y))
              (tuple int)
              (exists T1 (tuple (-> T1 int int) T1))))"#,
            )
            .unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    let rc_exp = record_elim_exp(&typed_exp).unwrap();

    println!("Source: {exp}");
    println!("Record elimination: {rc_exp}");
    assert_eq!(rc_exp, expected_exp);
}
