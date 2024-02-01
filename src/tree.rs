use crate::{SyntaxEvent, SyntaxKind, Token};

pub fn build_tree(tokens: Vec<Token>, events: Vec<SyntaxEvent>) -> Tree {
    let mut stack = Vec::new();
    let mut tokens = tokens.into_iter();

    for event in events {
        match event {
            SyntaxEvent::Enter(kind) => stack.push(Tree::new(kind)),
            SyntaxEvent::Exit => {
                let tree = stack.pop().unwrap();
                stack.last_mut().unwrap().push(Child::Tree(tree));
            }
            SyntaxEvent::Advance => {
                let token = tokens.next().unwrap();
                stack.last_mut().unwrap().push(Child::Token(token));
            }
        }
    }

    stack.pop().unwrap()
}

pub struct Tree {
    pub kind: SyntaxKind,
    pub children: Vec<Child>,
}

pub enum Child {
    Token(Token),
    Tree(Tree),
}

impl Tree {
    pub fn new(kind: SyntaxKind) -> Self {
        Tree {
            kind,
            children: Vec::new(),
        }
    }

    pub fn push(&mut self, child: Child) {
        self.children.push(child);
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn start(&self) -> Option<&Token> {
        if let Some(child) = self.children.first() {
            match child {
                Child::Token(token) => Some(token),
                Child::Tree(tree) => tree.start(),
            }
        } else {
            None
        }
    }

    pub fn end(&self) -> Option<&Token> {
        if let Some(child) = self.children.last() {
            match child {
                Child::Token(token) => Some(token),
                Child::Tree(tree) => tree.end(),
            }
        } else {
            None
        }
    }
}
