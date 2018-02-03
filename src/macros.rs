macro_rules! try_parse (
    ($remainder:ident, $parsefn:path) => {
        match $parsefn($remainder) {
            nom::IResult::Done(i, o) => (i, o),
            _ => return Err(()),
            // nom::IResult::Error(_) => return Err(()),
            // nom::IResult::Incomplete(_) => return Err(()),
        };
    }
);
