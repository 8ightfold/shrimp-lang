use self::super::Token_type;

pub struct Location {
    line: u64,
    column: u64,
}

pub struct Line_span {
    start: Location,
    end: Location,
}

pub struct Token {
    type_of: Token_type,
    span: Line_span,
}