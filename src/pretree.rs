#![allow(unused)]
use std::collections::HashMap;

pub struct Pretree {
    tree_group: HashMap<String, Tree>,
}

impl Pretree {
    pub fn new() -> Pretree {
        let mut p = Pretree {
            tree_group: HashMap::new(),
        };
        let methods = [
            "GET", "HEAD", "POST", "PUT", "PATCH", "DELETE", "CONNECT", "OPTIONS", "TRACE",
        ];
        for method in &methods {
            let tree = Tree::new(method);
            p.tree_group.insert(method.to_string(), tree);
        }
        return p;
    }

    pub fn store(&mut self, method: &str, url_rule: &str) {
        let t = self.tree_group.get_mut(method).unwrap();
        t.insert(url_rule);
    }

    pub fn query(&self, method: &str, url_path: &str) -> (bool, String, HashMap<String, String>) {
        let t = self.tree_group.get(method).unwrap();
        let (is_exist, node, vars) = t.search(url_path);
        if is_exist {
            (true, node.rule().into(), vars)
        } else {
            (false, "".to_string(), vars)
        }
    }
}

#[derive(Clone)]
struct Tree {
    rule: String,
    name: String,
    nodes: Vec<Tree>,
    is_end: bool,
    is_variable: bool,
}

impl Tree {
    pub fn new(name: &str) -> Tree {
        Tree {
            rule: String::from(""),
            name: name.to_string(),
            nodes: vec![],
            is_end: false,
            is_variable: false,
        }
    }

    pub fn with_variable(name: &str, is_variable: bool) -> Tree {
        Tree {
            rule: String::from(""),
            name: name.to_string(),
            nodes: vec![],
            is_end: false,
            is_variable,
        }
    }

    fn append_child(&mut self, node: Tree) {
        self.nodes.push(node);
    }

    pub fn child(&self) -> &Vec<Tree> {
        &self.nodes
    }

    pub fn rule(&self) -> &str {
        &self.rule
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn var_name(&self) -> String {
        self.name.trim_start_matches(':').to_string()
    }

    fn insert(&mut self, url_rule: &str) {
        let mut current = Some(self); // 作为游标指针使用，好像没有达到游标的效果
        let list = parse_path(url_rule);
        for word in &list {
            let now = current.take().unwrap();
            let mut index = None;
            for (idx, tree) in now.nodes.iter().enumerate() {
                if tree.name() == word {
                    index = Some(idx);
                    break;
                }
            }
            if let Some(i) = index {
                current = now.nodes.get_mut(i);
            } else {
                let node = Tree::with_variable(word, is_variable(word));
                now.append_child(node);

                current = now.nodes.last_mut();
            }
        }
        assert!(current.is_some());
        if let Some(current) = current.take() {
            current.rule = url_rule.into();
            current.is_end = true;
        }
    }

    fn search(&self, url_path: &str) -> (bool, &Tree, HashMap<String, String>) {
        let mut vars: HashMap<String, String> = HashMap::new();
        let mut current = Some(self);
        let list = parse_path(url_path);
        'for_list: for (index, word) in list.into_iter().enumerate() {
            let now = current.take().unwrap();

            for n in now.nodes.iter() {
                if n.name() == word {
                    current.replace(n);
                    continue 'for_list;
                }
            }

            for m in now.nodes.iter() {
                if m.is_variable && index > 0 {
                    vars.insert(m.var_name(), word);
                    current.replace(m);
                    continue 'for_list;
                }
            }
            current.replace(now);
        }
        let res = current.unwrap();

        let is_end = res.is_end;

        (is_end, res, vars)
    }
}

fn parse_path(path: &str) -> Vec<String> {
    let path = format_rule(path);
    let split = path.split("/");
    let paths: Vec<String> = split.map(|s| s.to_owned()).collect();
    paths
}

fn format_rule(rule: &str) -> String {
    let r = rule.replace("{", ":");
    let r = r.replace("}", "");
    r
}

fn is_variable(s: &str) -> bool {
    s.starts_with(":")
}
