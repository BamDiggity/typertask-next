use typertask_engine::parse_script;

#[test]
fn test_parse_valid_script() {
    let script = r#"
        DELAY 100
        MOVE 100 200
        CLICK
        TYPE "Hello"
        REPEAT 3 {
            DELAY 50
            TYPE "Loop"
        }
    "#;

    let commands = parse_script(script).expect("Failed to parse valid script");
    assert_eq!(commands.len(), 5);
}
