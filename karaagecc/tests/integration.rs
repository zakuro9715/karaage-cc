use karaagecc_source::Source;
#[test]
fn run() {
    assert_eq!(
        karaagecc::run(Source::inline("10"))
            .unwrap()
            .status
            .code()
            .unwrap(),
        10
    );
    assert_eq!(
        karaagecc::run(Source::inline("42"))
            .unwrap()
            .status
            .code()
            .unwrap(),
        42
    );
}

#[test]
fn compile_error() {
    assert!(format!(
        "{}",
        karaagecc::compile(Source::inline("xx")).expect_err("")
    )
    .contains("error"));
}
