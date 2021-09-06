# Pretree
pretree is a package for storing and querying routing rules with prefix tree .

pretree 是一个用于存储和查询路由规则的包。它用前缀树存储路由规则，支持包含变量的路由。

pretree is a package for storing and querying routing rules. It uses prefix tree to store routing rules and supports routing with variables.


Inspired by [obity/pretree](https://github.com/obity/pretree) (golang)

# Doc

See this document at [API documentation](https://docs.rs/pretree)

# Install

Add the following line to your Cargo.toml file:
    
    colorstyle = "1.0.1"
# Example

```
use pretree::Pretree;
let mut p = Pretree::new();
p.store("GET","account/{id}/info/:name");
p.store("GET","account/:id/login");
p.store("GET","account/{id}");
p.store("GET","bacteria/count_number_by_month");
let (ok,rule,vars) = p.query("GET","account/929239");
println!("ok:{} rule:{} vars:{:#?}",ok,rule,vars);

```
