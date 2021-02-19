#[derive(Clone, PartialEq, Debug)]
pub enum Types<'a> {
    Scope,
    Num(i32),
    Dec(f32),
    Bool(bool),
    Vec(Vec<Types<'a>>),
    String(String),
    FunctionCall(&'a str),
    Keyword(&'a str),
    Identifier(&'a str),
}

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
            Self::Keyword(keyword) => keyword,
            Self::Identifier(_) => "identifier",
        }
    }
}
