use dbus_message_parser::{Message, MessageType, Value};

fn create_method_call() -> Message {
    Message::method_call(
        "destination.address",
        "/object/path",
        "interface.name",
        "MethodName",
    )
}

fn check_error_msg(msg: Message, error_name: &str, error_message: &str) {
    let (header, body) = msg.split();
    assert_eq!(header.get_error_name().unwrap(), error_name);
    assert_eq!(body.len(), 1);
    assert_eq!(body[0], Value::String(error_message.to_string()));
}

#[test]
fn get_serial() {
    let mut msg = create_method_call();
    msg.set_serial(443);
    assert_eq!(msg.get_serial(), 443);
}

#[test]
fn get_reply_serial() {
    let mut msg = create_method_call();
    msg.set_serial(443);
    let msg = msg
        .method_return()
        .expect("Could not create method return message");
    assert_eq!(msg.get_reply_serial(), Some(443));
}

#[test]
fn get_reply_serial_none() {
    let msg = create_method_call();
    assert_eq!(msg.get_reply_serial(), None);
}

#[test]
fn get_path() {
    let msg = create_method_call();
    assert_eq!(msg.get_path(), Some("/object/path"));
}

#[test]
fn get_path_none() {
    let msg = create_method_call();
    let msg = msg
        .method_return()
        .expect("Could not create method return message");
    assert_eq!(msg.get_path(), None);
}

#[test]
fn has_interface() {
    let msg = create_method_call();
    assert!(msg.has_interface());
}

#[test]
fn get_interface() {
    let msg = create_method_call();
    assert_eq!(msg.get_interface(), Some("interface.name"));
}

#[test]
fn get_interface_none() {
    let msg = create_method_call();
    let msg = msg
        .method_return()
        .expect("Could not create method return message");
    assert_eq!(msg.get_interface(), None);
}

#[test]
fn has_member() {
    let msg = create_method_call();
    assert!(msg.has_member());
}

#[test]
fn get_member() {
    let msg = create_method_call();
    assert_eq!(msg.get_member(), Some("MethodName"));
}

#[test]
fn get_member_none() {
    let msg = create_method_call();
    let msg = msg
        .method_return()
        .expect("Could not create method return message");
    assert_eq!(msg.get_member(), None);
}

#[test]
fn has_error_name() {
    let msg = create_method_call();
    assert!(!msg.has_error_name());
}

#[test]
fn get_error_name() {
    let msg = create_method_call();
    let msg = msg.error("name".to_string(), "message".to_string());
    assert_eq!(msg.get_error_name(), Some("name"));
}

#[test]
fn get_sender() {
    let msg = create_method_call();
    let msg = msg
        .method_return()
        .expect("Could not create method return message");
    assert_eq!(msg.get_sender(), Some("destination.address"));
}

#[test]
fn get_sender_none() {
    let msg = create_method_call();
    assert_eq!(msg.get_sender(), None);
}

#[test]
fn get_destination() {
    let msg = create_method_call();
    assert_eq!(msg.get_destination(), Some("destination.address"));
}

#[test]
fn get_destination_none() {
    let msg = create_method_call();
    let msg = msg
        .method_return()
        .expect("Could not create method return message");
    assert_eq!(msg.get_destination(), None);
}

#[test]
fn get_signature() {
    let mut msg = create_method_call();
    msg.add_value(Value::Uint32(0));
    assert_eq!(msg.get_signature(), "u");
}

#[test]
fn get_signature_empty() {
    let msg = create_method_call();
    assert_eq!(msg.get_signature(), "");
}

#[test]
fn get_body() {
    let mut msg = create_method_call();
    msg.add_value(Value::Uint32(0));
    assert_eq!(msg.get_body(), &[Value::Uint32(0)][..]);
}

#[test]
fn get_type() {
    let msg = create_method_call();
    assert_eq!(msg.get_type(), MessageType::MethodCall);
}

#[test]
fn split() {
    let mut msg = create_method_call();
    msg.add_value(Value::Uint32(0));
    let (header, body) = msg.split();
    assert_eq!(header.get_signature(), Some("u"));
    assert_eq!(body, &[Value::Uint32(0)][..]);
}

#[test]
fn has_signature() {
    let msg = create_method_call();
    let (header, _) = msg.split();
    assert!(!header.has_signature());
}

#[test]
fn unknown_path() {
    let msg = create_method_call();
    let msg = msg.unknown_path().unwrap();
    check_error_msg(
        msg,
        "org.freedesktop.DBus.Error.UnknownPath",
        "does not have a path /object/path",
    );
}

#[test]
fn unknown_path_none() {
    let msg = create_method_call();
    let msg = msg.error(
        "org.example.error".to_string(),
        "Example error message".to_string(),
    );
    assert!(msg.unknown_path().is_none());
}

#[test]
fn unknown_interface() {
    let msg = create_method_call();
    let msg = msg.unknown_interface().unwrap();
    check_error_msg(
        msg,
        "org.freedesktop.DBus.Error.UnknownInterface",
        "does not have an interface interface.name",
    );
}

#[test]
fn unknown_interface_none() {
    let msg = create_method_call();
    let msg = msg.error(
        "org.example.error".to_string(),
        "Example error message".to_string(),
    );
    assert!(msg.unknown_interface().is_none());
}

#[test]
fn unknown_member() {
    let msg = create_method_call();
    let msg = msg.unknown_member().unwrap();
    check_error_msg(
        msg,
        "org.freedesktop.DBus.Error.UnknownMember",
        "does not have a member MethodName",
    );
}

#[test]
fn unknown_member_none() {
    let msg = create_method_call();
    let msg = msg.error(
        "org.example.error".to_string(),
        "Example error message".to_string(),
    );
    assert!(msg.unknown_member().is_none());
}
