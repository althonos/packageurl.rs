use ::percent_encoding::EncodeSet;
use ::percent_encoding::PercentEncode;
use ::percent_encoding::PercentDecode;

pub trait PercentCodec {
    fn encode<E: EncodeSet>(&self, encode_set: E) -> PercentEncode<E>;
    fn decode(&self) -> PercentDecode;
}

impl PercentCodec for [u8] {
    fn encode<E: EncodeSet>(&self, encode_set: E) -> PercentEncode<E> {
        ::percent_encoding::percent_encode(self, encode_set)
    }
    fn decode(&self) -> PercentDecode {
        ::percent_encoding::percent_decode(self)
    }
}

impl PercentCodec for str {
    fn encode<E: EncodeSet>(&self, encode_set: E) -> PercentEncode<E> {
        self.as_bytes().encode(encode_set)
    }
    fn decode(&self) -> PercentDecode {
        self.as_bytes().decode()
    }
}

impl<'a> PercentCodec for ::std::borrow::Cow<'a, str> {
    fn encode<E: EncodeSet>(&self, encode_set: E) -> PercentEncode<E> {
        self.as_bytes().encode(encode_set)
    }
    fn decode(&self) -> PercentDecode {
        self.as_bytes().decode()
    }
}
