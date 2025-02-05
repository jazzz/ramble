mod generated;

fn main() {}

#[cfg(test)]
mod tests {

    use std::u8;

    use crate::generated::*;

    #[test]
    fn round_trip() {
        let msg: Message = Hello { seq: 1, flags: 2 }.into();
        let bytes = msg.to_bytes().expect("to_bytes");

        let deserialized_msg = Message::from_bytes(bytes.as_slice()).expect("to message");
        let reserialized_bytes = deserialized_msg.to_bytes().expect("to_bytes_again");

        assert!(msg == deserialized_msg, "message mismatch");
        assert!(bytes == reserialized_bytes, "bytes mismatch");
    }

    #[test]
    fn negative_serialize() {
        let msg: Message = Hello { seq: 1, flags: 2 }.into();
        let bytes = msg.to_bytes().expect("to_bytes");

        let fake_msg: Message = Hello { seq: 0, flags: 2 }.into();
        let fake_bytes = fake_msg.to_bytes().expect("to_bytes");

        let deserialized_msg = Message::from_bytes(fake_bytes.as_slice()).expect("to message");
        let reserialized_bytes = deserialized_msg.to_bytes().expect("to_bytes_again");

        assert!(msg != deserialized_msg, "message mismatch");
        assert!(bytes != reserialized_bytes, "bytes mismatch");
    }

    #[test]
    fn msg_equals() {
        let m0: Message = Hello { seq: 1, flags: 2 }.into();
        let bytes0 = m0.to_bytes().expect("to_bytes");

        let m1: Message = Hello { seq: 1, flags: 2 }.into();
        let bytes1 = m1.to_bytes().expect("to_bytes");

        assert!(m0 == m1);
        assert!(bytes0 == bytes1);
    }

    #[test]
    fn msg_differ_by_contents() {
        let m0: Message = Hello { seq: 1, flags: 2 }.into();
        let bytes0 = m0.to_bytes().expect("to_bytes");

        let m1: Message = Hello { seq: 1, flags: 99 }.into();
        let bytes1 = m1.to_bytes().expect("to_bytes");

        assert!(m0 != m1);
        assert!(bytes0 != bytes1);
    }

    #[test]
    fn type_distinction() {
        let m0: Message = Hello { seq: 1, flags: 2 }.into();
        let bytes0 = m0.to_bytes().expect("to_bytes");

        let m1: Message = Bye { seq: 1, flags: 2 }.into();
        let bytes1 = m1.to_bytes().expect("to_bytes");

        assert!(m0 != m1, "types should be different");

        assert!(bytes0 != bytes1, "bytes differ by tag");
        assert!(&bytes0[1..] == &bytes1[1..], "contents should be the same");
    }

    #[test]
    fn check_for_padding() {
        let msg: Message = Primitives {
            u8: u8::MAX,
            u16: u16::MAX,
            u32: u32::MAX,
            u64: u64::MAX,
            i8: i8::MAX,
            i16: i16::MAX,
            i32: i32::MAX,
            i64: i64::MAX,
        }
        .into();

        let bytes = msg.to_bytes().expect("to_bytes");
        let payload = &bytes.as_slice()[1..]; // Skip message tag;

        #[rustfmt::skip]
        let expected: &[u8] = &[
            0xFF,
            0xFF,0xFF,
            0xFF,0xFF,0xFF,0xFF,
            0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
            0x7F,
            0xFF,0x7F,
            0xFF,0xFF,0xFF,0x7F,
            0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x7F,
        ];

        assert!(payload == expected); //Max values must result in max bytes

        let deserialized_msg = Message::from_bytes(bytes.as_slice()).expect("to message");
        assert!(msg == deserialized_msg, "message mismatch");
    }
}
