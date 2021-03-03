use crate::util::types::Types;

#[derive(Clone, PartialEq, Debug)]
pub struct Node<'a> {
    pub r#type: Types<'a>,
    pub children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(r#type: Types<'a>) -> Self {
        Self {
            r#type,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Node<'a>) {
        self.children.push(child);
    }
}

impl<'a> Types<'a> {
    pub fn stringify(&self) -> &'a str {
        match self {
            Self::Scope => "scope",
            Self::Num(_) => "num",
            Self::Dec(_) => "dec",
            Self::Bool(_) => "bool",
            Self::Vec(_) => "Vec",
            Self::String(_) => "String",
            Self::FunctionCall(_) => "function call",
            Self::Operator(_) => "operator",
            Self::CommonSyntax(_) => "common syntax",
            Self::Keyword(keyword) => keyword,
            Self::Identifier(_) => "identifier",
        }
    }
}
