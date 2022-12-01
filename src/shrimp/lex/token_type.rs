
pub enum Symbol {
    /* base */
    SCOPE,              // ::
    EQUALS,             // =

    STRAIGHT,           // |
    OPEN_ARROW,         // <
    CLOSE_ARROW,        // >
    COLON,              // :
    AMPERSAND,          // &
    D_AMPERSAND,        // &&
    /* unary */
    INC,                // --
    DEC,                // ++
    /* binary */
    PLUS,               // +
    MINUS,              // -
    MULTIPLY,           // *
    DIVIDE,             // /
    MOD,                // %
    PLUSEQ,             // +=
    MINUSEQ,            // -=
    MULTIPLYEQ,         // *=
    DIVIDEEQ,           // /=
    MODEQ,              // %=

    /* bitwise */
    B_XOR,              // ^
    B_NOT,              // ~
    B_ANDEQ,            // &=
    B_OREQ,             // |=
    B_XOREQ,            // ^=
    B_NOTEQ,            // ~=

    /* logical */
    L_AND,              // &&
    L_OR,               // ||
    L_XOR,              // ^^
    L_NOT,              // !
    L_EQ,               // ==
    L_NOTEQ,            // !=
    L_GREATEREQ,        // >=
    L_LESSEREQ,         // <=

    /* shifts */
    L_SHIFT,            // <<
    R_SHIFT,            // >>
    L_SHIFTEQ,          // <<=
    R_SHIFTEQ,          // >>=

    /* pipes */
    PIPE,               // |>

    /* member access */
    SLICE,              // ..
    MEMBER_REF,         // a.*b
    CLOSURE,            // |a|

    /* misc */
    L_COND,             // !?
    R_COND,             // :
    VARIADIC,           // ...
    OPTIONAL,           // a?
    OPT_CHECK,          // b??
    CONTINUE,           // ?>
    BREAK,              // !>
    MATCH,              // =>


    /* context dependent */
    /* operators */
    REFERENCE,          // &a
    U_PLUS,             // +a
    U_MINUS,            // -a
    PRE_INC,            // ++a
    PRE_DEC,            // --a
    POST_INC,           // a++
    POST_DEC,           // a--

    /* bitwise */
    B_OR,               // |
    B_AND,              // &
    /* logical */
    L_GREATER,          // >
    L_LESSER,           // <

    /* generics */
    L_GENERIC,          // <
    R_GENERIC,          // >

    /* member access */
    ACCESS,             // a.b
    DEREF,              // *a
}

pub enum Keyword {
    /* flow control */
    IF,                 // if
    ELSE,               // else
    WHILE,              // while
    MATCH,              // match
    RETURN,             // return

    /* logical */
    AND,                // and
    OR,                 // or
    NOT,                // not
    XOR,                // xor

    /* exceptions */
    TRY,                // try
    CATCH,              // catch

    /* types */
    VAR,                // var
    STRUCT,             // struct
    ENUM,               // enum
    VARIANT,            // variant
    TUPLE,              // tuple

    /* functional */
    BYTECOUNT,          // bytecount(a)
    BITCOUNT,           // bitcount(a)
    TYPEOF,             // typeof(a)

    /* modules and types */
    USING,              // using
    MODULE,             // module
    PUB,                // pub
    PRIV,               // priv
}

pub enum Metaop {
    IMPORT,             // @import
    EXPORT,             // @export
    EMBED,              // @embed
    META,               // @meta
    ERROR,              // @error
}

pub enum Token_type {
    /* variable */
    TYPE,               // eg. var _: a
    IDENTIFIER,         // eg. fold()

    /* literals */
    FLOAT(String),      // eg. 1.0f
    INTEGRAL(String),   // eg. 741u
    BOOL,               // eg. true
    STRING(String),     // eg. "hello"
    CHAR(String),       // eg. '\n'

    /* control */
    SYMBOL(Symbol),     // eg. %=
    KEYWORD(Keyword),   // eg. if
    metaop(Metaop),     // eg. @import
    macro_name(String), // eg. `op`

    /* internals */
    SEMICOLON,          // ;
    OPEN_PAREN,         // (
    CLOSE_PAREN,        // )
    OPEN_BRACKET,       // {
    CLOSE_BRACKET,      // }
    OPEN_BRACE,         // [
    CLOSE_BRACE,        // ]
    COMMA,              // ,
    DOT,                // .
    INLN_COMM(String),  // //
    BLK_COMM(String),   // /* ... */
    EOF,

    INVALID,
}