use typertask_engine::parse_script_with_vars;
use std::collections::HashMap;

#[test]
fn test_parse_valid_script_with_speed_and_vars() {
    let script = r#"
        SPEED 50
        DELAY 100
        MOVE 100 200
        CLICK
        TYPE "Hello $VAR{USER}"
        REPEAT 3 {
            DELAY 50
            TYPE "Loop"
        }
    "#;

    let mut vars = HashMap::new();
    vars.insert("USER".to_string(), "AgentAlpha".to_string());

    let commands = parse_script_with_vars(script, &vars).expect("Failed to parse script with vars");
    assert_eq!(commands.len(), 6);
}
