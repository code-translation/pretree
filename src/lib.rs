//! pretree 一个用于存储和查询路由规则的包。它用前缀树存储路由规则，支持包含变量的路由。
//!
//! pretree is a package for storing and querying routing rules. It uses prefix tree to store routing rules and supports routing with variables.
#![allow(unused)]
use std::collections::HashMap;

#[derive(Default)]
/// Prefix tree group object,Group storage by each http request method.
///
/// 前缀树组的对象,按每种http请求方法分组存储
pub struct Pretree {
    tree_group: HashMap<String, Tree>,
}

impl Pretree {
    /// Initialize object
    ///
    /// 初始化对象
    pub fn new() -> Pretree {
        let mut p = Pretree::default();
        let methods = [
            "GET", "HEAD", "POST", "PUT", "PATCH", "DELETE", "CONNECT", "OPTIONS", "TRACE",
        ];
        for method in &methods {
            let tree = Tree::new(method);
            p.tree_group.insert(method.to_string(), tree);
        }
        p
    }

    /// Store routing rules
    ///
    /// 存储路由规则
    ///
    /// # Parameters
    ///
    /// * `method` - HTTP method, such as GET, POST,DELETE ...
    ///
    /// * `url_rule` - url routing rule, such as  /user/:id
    /// # Example
    /// ```
    /// use pretree::Pretree;
    /// let mut p = Pretree::new();
    /// p.store("GET","account/{id}/info/:name");
    /// p.store("GET","account/:id/login");
    /// p.store("GET","account/{id}");
    /// p.store("GET","bacteria/count_number_by_month");
    /// ```
    pub fn store(&mut self, method: &str, url_rule: &str) {
        let t = self.tree_group.get_mut(method).unwrap();
        t.insert(url_rule);
    }

    /// Query the tree node with matching URL and return variables
    ///
    /// 查询URL匹配的树节点并返回变量
    /// # Parameters
    ///
    /// * `method` - HTTP method, such as GET, POST,DELETE ...
    ///
    /// * `url_path` - URL path to access, such as account/929239
    ///
    /// # Results
    /// * bool -  Does it exist
    /// * String - url routing rule
    /// * HashMap<String, String> - Routing variables
    /// ```
    /// use pretree::Pretree;
    /// let mut p = Pretree::new();
    /// p.store("GET","account/{id}/info/:name");
    /// p.store("GET","account/:id/login");
    /// p.store("GET","account/{id}");
    /// p.store("GET","bacteria/count_number_by_month");
    /// let (ok,rule,vars) = p.query("GET","account/929239");
    /// println!("ok:{} rule:{} vars:{:#?}",ok,rule,vars);
    /// assert_eq!(ok,true);
    /// assert_eq!(rule,"account/{id}");
    /// assert_eq!(vars.get("id"),Some(&"929239".to_string()));
    /// ```
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
// Prefix tree data structure
//
// 前缀树数据结构
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

    fn with_variable(name: &str, is_variable: bool) -> Tree {
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

    /// Get the routing rule of the current node
    ///
    /// 获取当前节点的路由规则
    pub fn rule(&self) -> &str {
        &self.rule
    }

    /// Get the name of the current node
    ///
    /// 获取当前节点的名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the variable name of the current node
    ///
    /// 获取当前节点的变量名
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
    let newpath = format_rule(path);
    let split = newpath.split('/');
    let paths: Vec<String> = split.map(|s| s.to_owned()).collect();
    paths
}

fn format_rule(rule: &str) -> String {
    rule.replace("{", ":").replace("}", "")
}

fn is_variable(s: &str) -> bool {
    s.starts_with(':')
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_match() {
        use crate::Pretree;
        // 测试数据data包括 http请求方法，路由规则，客户端请求路径
        let data: [[&str; 3]; 20] = [
            ["POST", "/pet/{petId}/uploadImage", "/pet/12121/uploadImage"],
            ["POST", "/pet", "/pet"],
            ["PUT", "/pet", "/pet"],
            ["GET", "/pet/findByStatus", "/pet/findByStatus"],
            ["GET", "/pet/{petId}", "/pet/113"],
            ["GET", "/pet/{petId}/info", "/pet/12121/info"],
            ["POST", "/pet/{petId}", "/pet/12121"],
            ["DELETE", "/pet/{petId}", "/pet/12121"],
            ["GET", "/store/inventory", "/store/inventory"],
            ["POST", "/store/order", "/store/order"],
            ["GET", "/store/order/{orderId}", "/store/order/939"],
            ["DELETE", "/store/order/{orderId}", "/store/order/939"],
            ["POST", "/user/createWithList", "/user/createWithList"],
            ["GET", "/user/{username}", "/user/1002"],
            ["PUT", "/user/{username}", "/user/1002"],
            ["DELETE", "/user/{username}", "/user/1002"],
            ["GET", "/user/login", "/user/login"],
            ["GET", "/user/logout", "/user/logout"],
            ["POST", "/user/createWithArray", "/user/createWithArray"],
            ["POST", "/user", "/user"],
        ];
        /* let data: [[&str; 3]; 3] = [
            ["GET", "/pet/findByStatus", "/pet/findByStatus"],
            ["GET", "/pet/{petId}", "/pet/113"],
            ["GET", "/pet/{petId}/info", "/pet/12121/info"],
        ]; */
        println!("{:?}", data);
        let mut p = Pretree::new();
        for v in data {
            let method = v[0];
            let source_rule = v[1];
            p.store(method, source_rule);
        }

        for v in data {
            let method = v[0];
            let url_path = v[2];
            let source_rule = v[1];
            let (ok, rule, _) = p.query(method, url_path);
            println!("ok:{},rule: {} ", ok, rule,);
            assert_eq!(ok, true);
            assert_eq!(rule, source_rule);
        }
    }
}
