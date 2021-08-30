use std::collections::HashMap;

#[derive(Debug)]
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
        p
    }
}

#[derive(Debug, Clone)]
struct Tree {
    rule: String,
    name: String,
    nodes: Vec<Tree>,
    is_end: bool,
    is_variable: bool,
}
impl Tree {
    fn new(name: &str) -> Tree {
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
        self.rule.to_string()
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn var_name(&self) -> String {
        let name = self.name.clone();
        name.trim_start_matches(':').to_string()
    }

    fn insert(&self, url_rule: &str) {
        let mut cur = self.clone();
        let list = parse_path(url_rule);
        for word in &list {
            let mut is_exist = false;
            for n in cur.child() {
                if n.name == word.to_string() {
                    is_exist = true;
                    cur = n.clone();
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
            cur.append_child(&node);
            cur = node.clone()
        }

        cur.rule = url_rule.to_string();
        cur.is_end = true;
    }
}

fn parse_path(path: &str) -> Vec<String> {
    let path = format_rule(path);
    let split = path.split("/");
    let paths: Vec<String> = split.map(|s| s.to_owned()).collect();
    return paths;
}
fn format_rule(rule: &str) -> String {
    let r = rule.replace("{", ":");
    let r = r.replace("}", "");
    return r;
}

fn is_variable(s: &str) -> bool {
    return s.starts_with(":");
}
