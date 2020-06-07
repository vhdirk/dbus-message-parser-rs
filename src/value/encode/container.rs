use crate::{EncodeError, EncodeResult, Encoder, Value};
use bytes::BytesMut;

impl<'a> Encoder<'a> {
    /// Encode a `&Vec<Value>` as an array into the buffer.
    pub fn array(&mut self, vec: &Vec<Value>, sig: &str, is_le: bool) -> EncodeResult {
        let mut array_buf = BytesMut::with_capacity(128);
        let mut encoder = Encoder::new(&mut array_buf, self.fds);
        let mut sig_cmp = String::new();
        for v in vec {
            v.get_signature(&mut sig_cmp);
            if sig == sig_cmp {
                sig_cmp.clear();
            } else {
                return Err(EncodeError::ArraySignatureMismatch(
                    sig.to_string(),
                    sig_cmp,
                ));
            }
            encoder.value(v, is_le)?;
        }

        let array_len = array_buf.len();
        self.uint_32(array_len as u32, is_le);

        match sig.get(0..1) {
            Some(s) => match s {
                "v" | "y" | "g" => {}
                "n" | "q" => self.algin(2),
                "b" | "i" | "u" | "a" | "s" | "o" => self.algin(4),
                "x" | "t" | "d" | "(" | "{" => self.algin(8),
                signature => return Err(EncodeError::UnknownSignature(signature.to_string())),
            },
            None => return Err(EncodeError::NullSignature),
        }

        self.buf.reserve(array_len);
        self.buf.extend(array_buf);

        Ok(())
    }

    /// Encode a `&[Value]` as a struct into the buffer.
    pub fn encode_struct(&mut self, values: &[Value], is_le: bool) -> EncodeResult {
        for v in values {
            self.value(v, is_le)?;
        }

        Ok(())
    }

    /// Encode a `&(Value, Value)` as a dict entry into the buffer.
    pub fn dict_entry(&mut self, b: &(Value, Value), is_le: bool) -> EncodeResult {
        let (key, value) = &*b;
        self.value(key, is_le)?;
        self.value(value, is_le)
    }

    /// Encode a `&[Value]` as a variant into the buffer.
    pub fn variant(&mut self, values: &[Value], is_le: bool) -> EncodeResult {
        let mut sig = String::new();

        for v in values {
            v.get_signature(&mut sig);
        }

        self.signature(&sig)?;

        for v in values {
            self.value(v, is_le)?;
        }

        Ok(())
    }
}
