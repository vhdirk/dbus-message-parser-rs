use dbus_message_parser::Message;
use bytes::BytesMut;


fn decode_encode(msg: &[u8]) {
    let mut offset: usize = 0;
    // Decode BytesMut to Message
    let bytes_mut = BytesMut::from(&msg[..]);
    // Decode BytesMut to Message
    let msg = Message::decode(&bytes_mut, &mut offset).unwrap();
    // Create a empty BytesMut
    let mut bytes_mut = BytesMut::new();
    // Encode Message to BytesMut
    msg.encode(&mut bytes_mut).unwrap();
}

#[test]
fn msg_1() {
    let msg = b"l\x02\x01\x01\n\0\0\0\x01\0\0\0=\0\0\0\x06\x01s\0\x05\0\0\0\
    :1.98\0\0\0\x05\x01u\0\x01\0\0\0\x08\x01g\0\x01s\0\0\x07\x01s\0\x14\0\0\0\
    org.freedesktop.DBus\0\0\0\0\x05\0\0\0:1.98\0";

    decode_encode(&msg[..]);
}

#[test]
fn msg_2() {
    let msg = b"l\x02\x01\x01\xec\x00\x00\x00`\x00\x00\x006\x00\x00\x00\x06\
    \x01s\x00\x06\x00\x00\x00:1.105\x00\x00\x08\x01g\x00\na{s(bgav)}\x00\x05\
    \x01u\x009\x01\x00\x00\x07\x01s\x00\x05\x00\x00\x00:1.99\x00\x00\x00\xe4\
    \x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00quit\x00\x00\x00\x00\x00\x00\
    \x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
    \x0c\x00\x00\x00new-document\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0b\x00\x00\x00\
    preferences\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
    \x00\x00\t\x00\x00\x00shortcuts\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00help\x00\x00\x00\x00\
    \x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
    \x00\x00\x05\x00\x00\x00about\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\n\x00\x00\x00new-window\
    \x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    decode_encode(&msg[..]);
}

#[test]
fn msg_3() {
    let msg = b"l\x04\x01\x01t\x00\x00\x00\xb8\x00\x00\x00v\x00\x00\x00\x01\
    \x01o\x00\x10\x00\x00\x00/org/gnome/dfeet\x00\x00\x00\x00\x00\x00\x00\x00\
    \x02\x01s\x00\x0f\x00\x00\x00org.gtk.Actions\x00\x08\x01g\x00\x16\
    asa{sb}a{sv}a{s(bgav)}\x00\x00\x00\x00\x00\x03\x01s\x00\x07\x00\x00\x00\
    Changed\x00\x07\x01s\x00\x05\x00\x00\x00:1.89\x00\x00\x00\x00\x00\x00\x00\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\\\x00\x00\x00\x00\x00\x00\
    \x00\x04\x00\x00\x00help\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x05\x00\x00\x00about\x00\
    \x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
    \x00\x00\x00\x00\x04\x00\x00\x00quit\x00\x00\x00\x00\x00\x00\x00\x00\x01\
    \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    decode_encode(&msg[..]);
}

#[test]
fn msg_4() {
    let msg = b"l\x01\x00\x01\x00\x00\x00\x00\xbd\x00\x00\x00\x8e\x00\x00\x00\
    \x01\x01o\x00\x04\x00\x00\x00/org\x00\x00\x00\x00\x02\x01s\x00#\x00\x00\
    \x00org.freedesktop.DBus.Introspectable\x00\x00\x00\x00\x00\x06\x01s\x00\
    \x1c\x00\x00\x00org.freedesktop.FileManager1\x00\x00\x00\x00\x03\x01s\x00\
    \n\x00\x00\x00Introspect\x00\x00\x00\x00\x00\x00\x07\x01s\x00\x05\x00\x00\
    \x00:1.89\x00\x00\x00";

    decode_encode(&msg[..]);
}
