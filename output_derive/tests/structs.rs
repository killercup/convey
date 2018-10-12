extern crate output;
#[macro_use]
extern crate output_derive;
#[macro_use]
extern crate serde_derive;

#[test]
fn struct_with_named_fields_of_primitive_types() -> Result<(), output::Error> {
    #[derive(Serialize, RenderOutput)]
    struct ErrorMessage {
        code: i32,
        name: String,
        message: String,
    }

    let human = output::human::test();
    let json = output::json::test();
    let mut out = output::new()
        .add_target(human.target())
        .add_target(json.target());

    out.print(&ErrorMessage {
        code: 42,
        name: String::from("info"),
        message: String::from("Derive works"),
    })?;
    out.flush()?;

    assert_eq!(
        human.to_string(),
        "code: 42\n\
         name: info\n\
         message: Derive works\n\n"
    );

    assert_eq!(
        json.to_string(),
        "{\"code\":42,\"name\":\"info\",\"message\":\"Derive works\"}\n\n"
    );

    Ok(())
}

#[test]
fn tuple_struct_of_primitive_types() -> Result<(), output::Error> {
    #[derive(Serialize, RenderOutput)]
    struct ErrorMessage(i32, String);

    let human = output::human::test();
    let json = output::json::test();
    let mut out = output::new()
        .add_target(human.target())
        .add_target(json.target());

    out.print(&ErrorMessage(42, String::from("Derive works")))?;
    out.flush()?;

    assert_eq!(human.to_string(), "(42, Derive works)\n");

    assert_eq!(json.to_string(), "[42,\"Derive works\"]\n\n");

    Ok(())
}
