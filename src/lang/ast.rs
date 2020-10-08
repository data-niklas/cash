use crate::interpreter::Rule;
use pest::iterators::{Pair,Pairs};

#[derive(Debug, Clone)]
pub struct Node{
    pub rule: Rule,
    pub content: String,
    pub inner: Vec<Node>
}

impl Node{
    pub fn new(rule: Rule, content: String, inner: Vec<Node>) -> Node{
        return Node{
            rule: rule,
            content: content,
            inner: inner
        }
    }

    pub fn content(&self) -> &String{
        return &self.content;
    }

    pub fn rule(&self) -> &Rule{
        return &self.rule;
    }

    pub fn inner(&self) -> &Vec<Node>{
        return &self.inner;
    }
}

pub fn build_ast(pair: Pair<Rule>) -> Node{
    let rule = pair.as_rule();
    let mut text = pair.as_span().as_str().to_owned();
    let children = pair.into_inner().map(|pair| build_ast(pair)).collect::<Vec<Node>>();
    if children.len() > 0{
        text = "".to_owned();
    }
    return Node::new(rule, text, children);
}