mod generated;

fn main() {}

#[cfg(test)]
mod tests {

    use std::{io::Cursor, u8};

    use binread::BinRead;
    use binwrite::BinWrite;

    use crate::generated::*;

    #[test]
    fn pstruct_nominal() {
        let input: String = "abcd".into();
        let mut ps = PString::new(input.clone());

        let inner = ps.get_str().expect("get_str");
        assert!(input.as_str() == inner, "strings don't match");

        let new_input: String = "efg".into();
        let size = ps.set_str(new_input.clone());

        assert!(size == new_input.len() as Strlen);
    }

    /// Tests that PString serializes and deserializes properly
    #[test]
    fn pstring_round_trip() {
        let raw_bytes = vec![0x61, 0x62, 0x63, 0x64]; // "abcd"
        let input = String::from_utf8(raw_bytes.clone()).expect("bad test setup");

        let obj = PString::new(input.clone());

        // Serialize PString
        let mut buf: Vec<u8> = vec![];
        obj.write(&mut buf).expect("bad writer");

        // Deserialize back to object
        let derserialzied_obj =
            PString::read(&mut Cursor::new(buf.as_slice())).expect("deserializing");

        // ReReserialize back to bytes
        let mut other_buf: Vec<u8> = vec![];
        obj.write(&mut other_buf).expect("bad writer");

        // compare values
        assert!(obj == derserialzied_obj, "obj mismatch");
        assert!(buf == other_buf, "bytes mismatch");
        assert!(buf == [0x04, 0x00, 0x61, 0x62, 0x63, 0x64]);
    }

    /// Tests that PString serializes and deserializes properly
    #[test]
    fn pstring_characterset() {
        let ascii_set = String::from(
            "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&\'()*+,-./:;<=>?@[]~",
        );

        let obj = PString::new(ascii_set.clone());

        // Serialize PString
        let mut buf: Vec<u8> = vec![];
        obj.write(&mut buf).expect("bad writer");

        // Deserialize back to object
        let deserialized_obj =
            PString::read(&mut Cursor::new(buf.as_slice())).expect("deserializing");

        // compare values
        assert!(obj == deserialized_obj, "obj mismatch");
        assert!(
            ascii_set.as_str() == obj.get_str().unwrap(),
            "string mismatch"
        );

        let some_index = 17;
        buf[some_index] = 255; // Add Non UTFChars
        let s = PString::read(&mut Cursor::new(buf.as_slice())).expect("message didn't parsing");
        assert!(s.get_str().is_err(), "no error thrown on access");
    }

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

    #[test]
    fn variable_len_message_serialization() {
        let input_data = vec![0x61 as u8, 0x62, 0x63, 0x64];
        let input_string = String::from_utf8(input_data.clone()).unwrap();

        let msg: Message = VariableLen {
            prefixed_str: PString::new(input_string),
        }
        .into();

        let bytes = msg.to_bytes().expect("to_bytes");
        let payload = &bytes.as_slice()[1..]; // Skip message tag;

        // Expected bytes are the [Length, *input_str]
        let mut expected = vec![input_data.len() as u8, 0];
        expected.extend_from_slice(input_data.as_slice());

        assert!(payload == expected);
    }
}
