use std::rc::Rc;

use crate::error::LexicalError;

#[derive(Debug)]
pub(crate) struct Tokens(Vec<LexResult>);

impl Tokens {
    pub fn new(tokens: Vec<LexResult>) -> Self {
        Self(tokens)
    }
}

impl IntoIterator for Tokens {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = LexResult;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub(crate) type LexResult = Result<Token, LexicalError>;

#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    kind: TokenKind,
    line: usize,
}

impl Token {
    pub(crate) fn new(
        kind: TokenKind,
        line: usize,
    ) -> Self {
        Self { kind, line }
    }

    pub(crate) fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub(crate) fn line(&self) -> usize {
        self.line
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenKind {
    // Single-character tokens
    LeftParen(LeftParenToken),
    RightParen(RightParenToken),
    LeftBrace(LeftBraceToken),
    RightBrace(RightBraceToken),
    Comma(CommaToken),
    Dot(DotToken),
    Minus(MinusToken),
    Plus(PlusToken),
    Semicolon(SemicolonToken),
    Slash(SlashToken),
    Star(StarToken),

    // One or two character tokens
    Bang(BangToken),
    BangEqual(BangEqualToken),
    Equal(EqualToken),
    EqualEqual(EqualEqualToken),
    Greater(GreaterToken),
    GreaterEqual(GreaterEqualToken),
    Less(LessToken),
    LessEqual(LessEqualToken),

    // Literals
    Identifier(IdentifierToken),
    String(StringToken),
    Number(NumberToken),

    // Keywords
    And(AndToken),
    Class(ClassToken),
    Else(ElseToken),
    False(FalseToken),
    Fun(FunToken),
    For(ForToken),
    If(IfToken),
    Nil(NilToken),
    Or(OrToken),
    Print(PrintToken),
    Return(ReturnToken),
    Super(SuperToken),
    This(ThisToken),
    True(TrueToken),
    Var(VarToken),
    While(WhileToken),

    Eof,
}

// TODO replace with macro

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LeftParenToken {
    lexeme: Rc<str>,
}
impl LeftParenToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<LeftParenToken> for TokenKind {
    fn from(t: LeftParenToken) -> Self {
        Self::LeftParen(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RightParenToken {
    lexeme: Rc<str>,
}
impl RightParenToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<RightParenToken> for TokenKind {
    fn from(t: RightParenToken) -> Self {
        Self::RightParen(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LeftBraceToken {
    lexeme: Rc<str>,
}
impl LeftBraceToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<LeftBraceToken> for TokenKind {
    fn from(t: LeftBraceToken) -> Self {
        Self::LeftBrace(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RightBraceToken {
    lexeme: Rc<str>,
}
impl RightBraceToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<RightBraceToken> for TokenKind {
    fn from(t: RightBraceToken) -> Self {
        Self::RightBrace(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CommaToken {
    lexeme: Rc<str>,
}
impl CommaToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<CommaToken> for TokenKind {
    fn from(t: CommaToken) -> Self {
        Self::Comma(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DotToken {
    lexeme: Rc<str>,
}
impl DotToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<DotToken> for TokenKind {
    fn from(t: DotToken) -> Self {
        Self::Dot(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MinusToken {
    lexeme: Rc<str>,
}
impl MinusToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<MinusToken> for TokenKind {
    fn from(t: MinusToken) -> Self {
        Self::Minus(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PlusToken {
    lexeme: Rc<str>,
}
impl PlusToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<PlusToken> for TokenKind {
    fn from(t: PlusToken) -> Self {
        Self::Plus(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SemicolonToken {
    lexeme: Rc<str>,
}
impl SemicolonToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<SemicolonToken> for TokenKind {
    fn from(t: SemicolonToken) -> Self {
        Self::Semicolon(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SlashToken {
    lexeme: Rc<str>,
}
impl SlashToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<SlashToken> for TokenKind {
    fn from(t: SlashToken) -> Self {
        Self::Slash(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StarToken {
    lexeme: Rc<str>,
}
impl StarToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<StarToken> for TokenKind {
    fn from(t: StarToken) -> Self {
        Self::Star(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BangToken {
    lexeme: Rc<str>,
}
impl BangToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<BangToken> for TokenKind {
    fn from(t: BangToken) -> Self {
        Self::Bang(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BangEqualToken {
    lexeme: Rc<str>,
}
impl BangEqualToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<BangEqualToken> for TokenKind {
    fn from(t: BangEqualToken) -> Self {
        Self::BangEqual(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EqualToken {
    lexeme: Rc<str>,
}
impl EqualToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<EqualToken> for TokenKind {
    fn from(t: EqualToken) -> Self {
        Self::Equal(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EqualEqualToken {
    lexeme: Rc<str>,
}
impl EqualEqualToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<EqualEqualToken> for TokenKind {
    fn from(t: EqualEqualToken) -> Self {
        Self::EqualEqual(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GreaterToken {
    lexeme: Rc<str>,
}
impl GreaterToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<GreaterToken> for TokenKind {
    fn from(t: GreaterToken) -> Self {
        Self::Greater(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GreaterEqualToken {
    lexeme: Rc<str>,
}
impl GreaterEqualToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<GreaterEqualToken> for TokenKind {
    fn from(t: GreaterEqualToken) -> Self {
        Self::GreaterEqual(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LessToken {
    lexeme: Rc<str>,
}
impl LessToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<LessToken> for TokenKind {
    fn from(t: LessToken) -> Self {
        Self::Less(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LessEqualToken {
    lexeme: Rc<str>,
}
impl LessEqualToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<LessEqualToken> for TokenKind {
    fn from(t: LessEqualToken) -> Self {
        Self::LessEqual(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct IdentifierToken {
    lexeme: Rc<str>,
}
impl IdentifierToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }

    pub(crate) fn lexeme(&self) -> Rc<str> {
        Rc::clone(&self.lexeme)
    }
}

impl From<IdentifierToken> for TokenKind {
    fn from(t: IdentifierToken) -> Self {
        Self::Identifier(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StringToken {
    lexeme: Rc<str>,
}
impl StringToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }

    pub(crate) fn lexeme(&self) -> Rc<str> {
        Rc::clone(&self.lexeme)
    }
}

impl From<StringToken> for TokenKind {
    fn from(t: StringToken) -> Self {
        Self::String(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NumberToken {
    lexeme: f64,
}
impl NumberToken {
    pub(crate) fn new(lexeme: f64) -> Self {
        Self { lexeme }
    }

    pub(crate) fn lexeme(&self) -> f64 {
        self.lexeme
    }
}

impl From<NumberToken> for TokenKind {
    fn from(t: NumberToken) -> Self {
        Self::Number(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AndToken {
    lexeme: Rc<str>,
}
impl AndToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<AndToken> for TokenKind {
    fn from(t: AndToken) -> Self {
        Self::And(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ClassToken {
    lexeme: Rc<str>,
}
impl ClassToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<ClassToken> for TokenKind {
    fn from(t: ClassToken) -> Self {
        Self::Class(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ElseToken {
    lexeme: Rc<str>,
}
impl ElseToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<ElseToken> for TokenKind {
    fn from(t: ElseToken) -> Self {
        Self::Else(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FalseToken {
    lexeme: Rc<str>,
}
impl FalseToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<FalseToken> for TokenKind {
    fn from(t: FalseToken) -> Self {
        Self::False(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FunToken {
    lexeme: Rc<str>,
}
impl FunToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<FunToken> for TokenKind {
    fn from(t: FunToken) -> Self {
        Self::Fun(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ForToken {
    lexeme: Rc<str>,
}
impl ForToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<ForToken> for TokenKind {
    fn from(t: ForToken) -> Self {
        Self::For(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct IfToken {
    lexeme: Rc<str>,
}
impl IfToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<IfToken> for TokenKind {
    fn from(t: IfToken) -> Self {
        Self::If(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NilToken {
    lexeme: Rc<str>,
}
impl NilToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<NilToken> for TokenKind {
    fn from(t: NilToken) -> Self {
        Self::Nil(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct OrToken {
    lexeme: Rc<str>,
}
impl OrToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<OrToken> for TokenKind {
    fn from(t: OrToken) -> Self {
        Self::Or(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PrintToken {
    lexeme: Rc<str>,
}
impl PrintToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<PrintToken> for TokenKind {
    fn from(t: PrintToken) -> Self {
        Self::Print(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ReturnToken {
    lexeme: Rc<str>,
}
impl ReturnToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<ReturnToken> for TokenKind {
    fn from(t: ReturnToken) -> Self {
        Self::Return(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SuperToken {
    lexeme: Rc<str>,
}
impl SuperToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<SuperToken> for TokenKind {
    fn from(t: SuperToken) -> Self {
        Self::Super(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ThisToken {
    lexeme: Rc<str>,
}
impl ThisToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<ThisToken> for TokenKind {
    fn from(t: ThisToken) -> Self {
        Self::This(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TrueToken {
    lexeme: Rc<str>,
}
impl TrueToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<TrueToken> for TokenKind {
    fn from(t: TrueToken) -> Self {
        Self::True(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VarToken {
    lexeme: Rc<str>,
}
impl VarToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<VarToken> for TokenKind {
    fn from(t: VarToken) -> Self {
        Self::Var(t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct WhileToken {
    lexeme: Rc<str>,
}
impl WhileToken {
    pub(crate) fn new(lexeme: impl Into<Rc<str>>) -> Self {
        Self {
            lexeme: lexeme.into(),
        }
    }
}

impl From<WhileToken> for TokenKind {
    fn from(t: WhileToken) -> Self {
        Self::While(t)
    }
}
