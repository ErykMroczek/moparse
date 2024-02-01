use crate::{SyntaxEvent, SyntaxKind, Token};

pub fn build_tree(tokens: Vec<Token>, comments: Vec<Token>, events: Vec<SyntaxEvent>) -> Tree {
    let mut stack = Vec::new();
    let mut tokens = tokens.into_iter();
    let mut comments = comments.into_iter().peekable();

    for event in events {
        match event {
            SyntaxEvent::Enter(kind) => stack.push(Tree::new(kind)),
            SyntaxEvent::Exit => {
                let tree = stack.pop().unwrap();
                stack.last_mut().unwrap().children.push(Child::Tree(tree));
            }
            SyntaxEvent::Advance => {
                let token = tokens.next().unwrap();
                while let Some(comment) = comments.peek() {
                    if comment.idx < token.idx {
                        stack.last_mut().unwrap().comments.push(comments.next().unwrap());
                    }
                }
                stack.last_mut().unwrap().children.push(Child::Token(token));
            }
        }
    }

    stack.pop().unwrap()
}

pub struct Tree {
    kind: SyntaxKind,
    children: Vec<Child>,
    comments: Vec<Token>,
}

enum Child {
    Token(Token),
    Tree(Tree),
}

impl Tree {
    pub fn new(kind: SyntaxKind) -> Self {
        Tree {
            kind,
            children: Vec::new(),
            comments: Vec::new(),
        }
    }
}
