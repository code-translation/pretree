#![allow(unused)]
use std::collections::HashMap;

struct Pretree {
    pub tree_group: HashMap<String, Tree>,
}

impl Pretree {
    fn new() -> Pretree {
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

    pub fn store(&self, method: &str, url_rule: &str) {
        let t = self.tree_group.get(method).unwrap();
        t.insert(url_rule)
    }

    pub fn query(&self, method: &str, url_path: &str) -> (bool, String, HashMap<String, String>) {
        let t = self.tree_group.get(method).unwrap();
        let (is_exist, node, vars) = t.search(url_path);
        if is_exist {
            (true, node.rule(), vars)
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

    fn append_child(&mut self, node: &Tree) {
        self.nodes.push(node.clone());
    }

    pub fn child(&self) -> Vec<Tree> {
        self.nodes.clone()
    }

    pub fn rule(&self) -> String {
        self.rule.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn var_name(&self) -> String {
        let name = self.name.clone();
        name.trim_start_matches(':').to_string()
    }

    fn insert(&self, url_rule: &str) {
        let mut current = self.clone();
        let list = parse_path(url_rule);
        for word in &list {
            let mut is_exist = false;
            for n in current.child() {
                if n.name == word.to_string() {
                    is_exist = true;
                    current = n.clone();
                    break;
                }
            }

            if is_exist {
                continue;
            }
            let mut node = Tree::new(word);
            if is_variable(word) {
                node.is_variable = true
            };
            current.append_child(&node);
            current = node.clone()
        }

        current.rule = url_rule.to_string();
        current.is_end = true;
    }

    fn search(&self, url_path: &str) -> (bool, Tree, HashMap<String, String>) {
        let mut vars: HashMap<String, String> = HashMap::new();
        let mut current = self.clone();
        let list = parse_path(url_path);
        for (index, word) in list.iter().enumerate() {
            let mut is_exist = false;
            let mut has_var = false;
            for n in current.child() {
                if n.name == word.clone() {
                    has_var = false;
                    is_exist = true;
                    current = n;
                    break;
                }
            }
            if is_exist {
                continue;
            }

            for m in current.child() {
                if m.is_variable && index > 0 && !has_var {
                    has_var = true;
                    current = m.clone();
                    vars.insert(m.var_name(), word.clone());
                    break;
                }
            }
            if has_var {
                continue;
            }
        }
        if current.is_end {
            (true, current, vars)
        } else {
            (false, current.clone(), vars)
        }
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
