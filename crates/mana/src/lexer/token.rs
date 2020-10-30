use logos::Logos;

/// Enum for findable tokens and statements
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    /// [Attribute macros](https://doc.rust-lang.org/stable/reference/procedural-macros.html#attribute-macros) like `derive`<br>
    /// ```ignore
    /// #[derive(Copy)]
    /// struct Something;
    /// ```
    #[regex(r"#\[[^\[\]\n]*\][\t\f\r\n]\n")]
    Attribute,

    /// Rust comments
    /// ```ignore
    /// // Here is a comment!
    /// /// Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do [...]
    /// ```
    #[regex(r"/(//?!*|\*+!*).+\n")]
    Comment,

    /// Rust comments
    /// ```ignore
    /// // Here is a comment!
    /// /// Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do [...]
    /// ```
    #[regex(r"/\*.*\*/\n")]
    MultiLineComment,

    /// Abstract statement
    #[token("abstract")]
    KwAbstract,

    /// Casting statement
    /// ```rust
    /// let x: i32 = 8_u8 as i32;
    /// ```
    #[token("as")]
    KwAs,

    /// Asynchronous statement keyword
    #[token("async")]
    KwAsync,

    /// Await statement keyword
    #[token("await")]
    KwAwait,

    /// Box statement keyword
    #[token("box")]
    KwBox,

    /// Become statement keyword
    #[token("become")]
    KwBecome,

    /// Break statement keyword
    #[token("break")]
    KwBreak,

    /// Constant statement keyword
    #[token("const")]
    KwConst,

    /// Continue statement keyword
    #[token("continue")]
    KwContinue,

    /// Do statement keyword
    #[token("do")]
    KwDo,

    /// Dynamic statement keyword
    #[token("dyn")]
    KwDyn,

    /// Else statement keyword
    #[token("else")]
    KwElse,

    /// Enum statement keyword
    #[token("enum")]
    KwEnum,

    /// Extern statement keyword
    #[token("extern")]
    KwExtern,

    /// False boolean
    #[token("false")]
    KwFalse,

    /// Final statement keyword
    #[token("final")]
    KwFinal,

    /// Function statement keyword
    #[token("fn")]
    KwFn,

    /// For loop statement keyword
    #[token("for")]
    Kwfor,

    /// If statement keyword
    #[token("if")]
    KwIf,

    /// Implementation statement keyword
    #[token("impl")]
    KwImpl,

    /// In statement keyword
    #[token("in")]
    KwIn,

    /// Variable assignation statement keyword
    #[token("let")]
    KwLet,

    /// Loop statement keyword
    #[token("loop")]
    KwLoop,

    /// Macro keyword
    #[token("macro")]
    KwMacro,

    /// Pattern matching statement keyword
    #[token("match")]
    KwMatch,

    /// Module statement keyword
    #[token("mod")]
    KwMod,

    /// Move statement keyword
    #[token("move")]
    KwMove,

    /// Mutable statement keyword
    #[token("mut")]
    KwMut,

    /// Override statement keyword
    #[token("override")]
    KwOverride,

    /// Private statement keyword
    #[token("priv")]
    KwPriv,

    /// Public statement keyword
    #[token("pub")]
    KwPub,

    /// Reference statement keyword
    #[token("ref")]
    KwRef,

    /// Return statement keyword
    #[token("return")]
    KwReturn,

    /// Self type keyword
    #[token("Self")]
    KwSelfType,

    /// Self value keyword
    #[token("self")]
    KwSelfValue,

    /// Static statement keyword
    #[token("static")]
    KwStatic,

    /// Struct value keyword
    #[token("struct")]
    KwStruct,

    /// Super statement keyword
    #[token("super")]
    KwSuper,

    /// Trait statement keyword
    #[token("trait")]
    KwTrait,

    /// True boolean
    #[token("true")]
    KwTrue,

    /// Try-catch statement keyword
    #[token("try")]
    KwTry,

    /// Type statement keyword
    #[token("type")]
    KwType,

    /// Typeof statement keyword
    #[token("typeof")]
    KwTypeOf,

    /// Union statement keyword
    #[token("union")]
    KwUnion,

    /// Unsafe statement keyword
    #[token("unsafe")]
    KwUnsafe,

    /// Unsized statement keyword
    #[token("unsized")]
    KwUnsized,

    /// Module import statement keyword
    #[token("use")]
    KwUse,

    /// Where statement keyword
    #[token("where")]
    KwWhere,

    /// While statement keyword
    #[token("while")]
    KwWhile,

    /// Yield statement keyword
    #[token("yield")]
    KwYield,

    /// Period punctuation
    #[token(".")]
    Period,

    /// Comma punctuation
    #[token(",")]
    Comma,

    /// Semi-colon punctuation
    #[token(";")]
    SemiColon,

    /// Colon punctuation
    #[token(":")]
    Colon,

    /// Exclamation mark
    #[token("!")]
    ExclamationMark,

    /// Question mark
    #[token("?")]
    QuestionMark,

    /// Left parenthesis
    #[token("(")]
    LeftParenthesis,

    /// Right parenthesis
    #[token(")")]
    RightParenthesis,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("<")]
    LeftChevron,

    #[token(">")]
    RightChevron,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token(" +")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("\\")]
    Backslash,

    #[token("`")]
    GraveAccent,

    #[token("%")]
    Modulo,

    #[token("#")]
    Hash,

    #[token("|")]
    Pipe,

    #[token("=")]
    Equal,

    #[token("$")]
    Dollar,

    #[token("&")]
    Ampersand,

    #[token("^")]
    Caret,

    #[token("'")]
    Apostrophe,

    #[token("\"")]
    Prime,

    #[regex(r"\t")]
    HorizontalTab,

    #[regex(r"\v")]
    VerticalTab,

    #[regex(r"\n+")]
    NextLine,

    #[regex(r" +", logos::skip)]
    Space,

    #[regex(r"[0-9]+")]
    Number,

    #[regex(r"[a-zA-Z0-9_]*[a-zA-Z]+[a-zA-Z0-9_]*!")]
    Macro,

    /// Function, variable, macro, enum, struct or trait identifier, must contain at least one letter.
    #[regex(r"[a-zA-Z0-9_]*[a-zA-Z]+[a-zA-Z0-9_]*")]
    Identifier,

    #[token("_")]
    Underscore,

    #[error]
    #[regex(r"[\t\f\r]+", logos::skip)]
    Error,
}