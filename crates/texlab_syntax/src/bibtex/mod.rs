mod ast;
mod finder;
mod formatting;
mod lexer;
mod parser;

pub use self::ast::*;
pub use self::finder::*;
pub use self::formatting::*;

use self::lexer::BibtexLexer;
use self::parser::BibtexParser;
use texlab_protocol::Position;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BibtexSyntaxTree {
    pub root: BibtexRoot,
}

impl BibtexSyntaxTree {
    pub fn preambles(&self) -> Vec<&BibtexPreamble> {
        let mut preambles: Vec<&BibtexPreamble> = Vec::new();
        for declaration in &self.root.children {
            if let BibtexDeclaration::Preamble(preamble) = declaration {
                preambles.push(&preamble);
            }
        }
        preambles
    }

    pub fn strings(&self) -> Vec<&BibtexString> {
        let mut strings: Vec<&BibtexString> = Vec::new();
        for declaration in &self.root.children {
            if let BibtexDeclaration::String(string) = declaration {
                strings.push(&string);
            }
        }
        strings
    }

    pub fn entries(&self) -> Vec<&BibtexEntry> {
        let mut entries: Vec<&BibtexEntry> = Vec::new();
        for declaration in &self.root.children {
            if let BibtexDeclaration::Entry(entry) = declaration {
                entries.push(&entry);
            }
        }
        entries
    }

    pub fn entry(&self, key: &str) -> Option<&BibtexEntry> {
        self.entries()
            .into_iter()
            .find(|entry| entry.key.as_ref().map(BibtexToken::text) == Some(key))
    }

    pub fn find(&self, position: Position) -> Vec<BibtexNode> {
        let mut finder = BibtexFinder::new(position);
        finder.visit_root(&self.root);
        finder.results
    }

    pub fn crossref(&self, entry: &BibtexEntry) -> Option<&BibtexEntry> {
        let field = entry.field("crossref")?;
        if let Some(BibtexContent::BracedContent(content)) = &field.content {
            if let Some(BibtexContent::Word(name)) = content.children.get(0) {
                return self.entry(name.token.text());
            }
        }
        None
    }
}

impl From<BibtexRoot> for BibtexSyntaxTree {
    fn from(root: BibtexRoot) -> Self {
        BibtexSyntaxTree { root }
    }
}

impl From<&str> for BibtexSyntaxTree {
    fn from(text: &str) -> Self {
        let lexer = BibtexLexer::new(text);
        let mut parser = BibtexParser::new(lexer);
        parser.root().into()
    }
}
