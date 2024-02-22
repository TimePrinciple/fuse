use hyper::Request;

fn make_request() -> String {
    let request = Request::builder()
        .method("GET")
        .uri("http://localhost:5000/")
        .body(())
        .unwrap();
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_request() {
        make_request();
    }
}
