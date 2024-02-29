use hyper::Request;

fn make_request() -> Request<()> {
    Request::builder()
        .method("GET")
        .uri("http://localhost:8000/api/v1/tree?repo_path=/projects/fuser")
        .body(())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_request() {
        let req = make_request();
    }
}
