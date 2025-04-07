use percent_encoding::AsciiSet;
use percent_encoding::PercentDecode;
use percent_encoding::PercentEncode;

pub trait PercentCodec {
    fn encode(&self, encode_set: &'static AsciiSet) -> PercentEncode;
    fn decode(&self) -> PercentDecode;
}

impl PercentCodec for [u8] {
    fn encode(&self, encode_set: &'static AsciiSet) -> PercentEncode {
        ::percent_encoding::percent_encode(self, encode_set)
    }
    fn decode(&self) -> PercentDecode {
        ::percent_encoding::percent_decode(self)
    }
}

impl PercentCodec for str {
    fn encode(&self, encode_set: &'static AsciiSet) -> PercentEncode {
        self.as_bytes().encode(encode_set)
    }
    fn decode(&self) -> PercentDecode {
        self.as_bytes().decode()
    }
}

impl PercentCodec for ::std::borrow::Cow<'_, str> {
    fn encode<'s>(&self, encode_set: &'static AsciiSet) -> PercentEncode {
        self.as_bytes().encode(encode_set)
    }
    fn decode(&self) -> PercentDecode {
        self.as_bytes().decode()
    }
}
