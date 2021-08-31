pub mod pretree;
#[cfg(test)]
mod tests {
    #[test]
    fn test_match() {
        use crate::pretree::Pretree;
        // 测试数据data包括 http请求方法，路由规则，客户端请求路径
        //         let data: [[&str; 3]; 20] = [
        // ["POST", "/pet/{petId}/uploadImage", "/pet/12121/uploadImage"],
        // ["POST", "/pet", "/pet"],
        // ["PUT", "/pet", "/pet"],
        // ["GET", "/pet/findByStatus", "/pet/findByStatus"],
        // ["GET", "/pet/{petId}", "/pet/113"],
        // ["GET", "/pet/{petId}/info", "/pet/12121/info"],
        // ["POST", "/pet/{petId}", "/pet/12121"],
        // ["DELETE", "/pet/{petId}", "/pet/12121"],
        // ["GET", "/store/inventory", "/store/inventory"],
        // ["POST", "/store/order", "/store/order"],
        // ["GET", "/store/order/{orderId}", "/store/order/939"],
        // ["DELETE", "/store/order/{orderId}", "/store/order/939"],
        // ["POST", "/user/createWithList", "/user/createWithList"],
        // ["GET", "/user/{username}", "/user/1002"],
        // ["PUT", "/user/{username}", "/user/1002"],
        // ["DELETE", "/user/{username}", "/user/1002"],
        // ["GET", "/user/login", "/user/login"],
        // ["GET", "/user/logout", "/user/logout"],
        // ["POST", "/user/createWithArray", "/user/createWithArray"],
        // ["POST", "/user", "/user"],
        // ];
        let data: [[&str; 3]; 3] = [
            ["GET", "/pet/findByStatus", "/pet/findByStatus"],
            ["GET", "/pet/{petId}", "/pet/113"],
            ["GET", "/pet/{petId}/info", "/pet/12121/info"],
        ];
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
