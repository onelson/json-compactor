use super::compactor;

#[test]
fn test_compactor() {
    assert_eq!(
        compactor(r#" { "msg": "HelloWorld" } "#),
        Ok(String::from(r#"{"msg":"HelloWorld"}"#))
    );
}

#[test]
fn test_nested_strings() {
    assert_eq!(
        compactor(r#"{"msg": "Hello \"inner\" World" }"#),
        Ok(String::from(r#"{"msg":"Hello \"inner\" World"}"#))
    );
}

#[test]
fn test_kitchen_sink() {
    assert_eq!(
        compactor(
            r#"{"arr": [ 
        {"k": "v"}, 123, 
        null, 0.1234, true, false   ],
        "something": true, "nothing":null }"#
        ),
        Ok(String::from(
            r#"{"arr":[{"k":"v"},123,null,0.1234,true,false],"something":true,"nothing":null}"#
        ))
    );
}
