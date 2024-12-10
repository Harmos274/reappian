pub const Token = struct {
    pos: u64,
    kind: Kind,
};

pub const Kind = enum {
    OpenParen,
    CloseParen,
};
