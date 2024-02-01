use crate::{SyntaxEvent, SyntaxKind, Token};

struct Tree {
    kind: SyntaxKind,
    parent: Option<usize>,
    children: Vec<Child>,
    start: Option<usize>,
    end: Option<usize>,
}

enum Child {
    Token(usize),
    Tree(usize),
}

struct ParseTree {
    nodes: Vec<Tree>,
    tokens: Vec<Token>,
    comments: Vec<Token>,
}

impl ParseTree {
    pub fn new(tokens: Vec<Token>, comments: Vec<Token>, mut events: Vec<SyntaxEvent>) -> Self {
        let mut nodes = Vec::new();
        let mut toks = tokens.iter().enumerate();
        let mut stack = Vec::new();

        assert!(matches!(events.pop(), Some(SyntaxEvent::Exit)));

        for event in events {
            match event {
                SyntaxEvent::Enter(kind) => {
                    if let Some(i) = stack.last() {
                        let mark = nodes.len();
                        let parent: &mut Tree = nodes.get_mut(*i).unwrap();
                        parent.children.push(Child::Tree(mark));
                    }
                    stack.push(nodes.len());
                    nodes.push(Tree {
                        kind,
                        parent: None,
                        children: Vec::new(),
                        start: None,
                        end: None,
                    });
                }
                SyntaxEvent::Exit => {
                    let node = stack.pop().unwrap();
                    let tree = nodes.get_mut(node).unwrap();
                    tree.parent = if let Some(i) = stack.last() {
                        Some(*i)
                    } else {
                        None
                    };
                }
                SyntaxEvent::Advance => {
                    let (idx, _) = toks.next().unwrap();
                    let parent = nodes.get_mut(*stack.last().unwrap()).unwrap();
                    parent.children.push(Child::Token(idx));
                    if let None = parent.start {
                        parent.start = Some(idx);
                    }
                    parent.end = Some(idx);
                }
            }
        }

        ParseTree {
            nodes,
            tokens,
            comments,
        }
    }
}
