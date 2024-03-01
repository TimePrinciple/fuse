use std::{str, sync::Arc};

use anyhow::Result;
use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use hyper::{client::conn::http1::SendRequest, Request};
use hyper_util::rt::TokioIo;
use tokio::{net::TcpStream, runtime::Runtime};

/// MegaClient is used to handling connection details.
/// Adapting the remote server's asynchronous nature, this client is also
/// implemented in an asynchronous manner. But an async client in a synchronous
/// context. This is achieved by constructing an async *tokio runtime* within.
#[derive(Debug)]
pub struct MegaClient {
    rt: Arc<Runtime>,
    sender: SendRequest<Empty<Bytes>>,
}

impl MegaClient {
    /// Creates a MegaClient from a given runtime. The reason it exists instead
    /// of providing a default `Runtime` is to enable customization on
    /// `Runtime`, like tunning the number of worker_threads or else.
    pub fn from(rt: Arc<Runtime>) -> Result<MegaClient> {
        let host = "localhost";
        let port = "8000";
        let addr = format!("{}:{}", host, port);
        let stream = rt.block_on(TcpStream::connect(addr))?;
        let io = TokioIo::new(stream);

        let (sender, conn) = rt.block_on(hyper::client::conn::http1::handshake::<
            TokioIo<tokio::net::TcpStream>,
            Empty<Bytes>,
        >(io))?;

        rt.spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });
        dbg!(&rt);
        dbg!(&sender);
        Ok(MegaClient { rt, sender })
    }

    /// Send a `Request` to the server pointed by this MegaClient, retrieve the
    /// content in response to comprise a `String`.
    pub fn request(&mut self, req: Request<Empty<Bytes>>) -> Result<String> {
        let mut response = self.rt.block_on(self.sender.send_request(req))?;

        let output = self.rt.block_on(async {
            let mut output = String::new();
            while let Some(next) = response.frame().await {
                let frame = next.unwrap();
                if let Some(chunk) = frame.data_ref() {
                    let c = chunk.as_ref();
                    output.push_str(str::from_utf8(c).unwrap());
                    // io::stdout().write_all(&chunk).await?;
                }
            }
            output
        });

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use tokio::runtime;

    use super::*;

    #[test]
    fn test_create_mega_client() {
        let rt = runtime::Builder::new_multi_thread()
            .worker_threads(10)
            .enable_all()
            .build()
            .unwrap();
        assert!(MegaClient::from(Arc::new(rt)).is_ok());
    }

    fn create_mega_client() -> MegaClient {
        let rt = runtime::Builder::new_multi_thread()
            .worker_threads(10)
            .enable_all()
            .build()
            .unwrap();
        MegaClient::from(Arc::new(rt)).unwrap()
    }

    fn form_request_to(target: &str) -> Request<Empty<Bytes>> {
        Request::builder()
            .method("GET")
            .uri(target)
            .body(Empty::<Bytes>::new())
            .unwrap()
    }

    /// This test requires a working mega server
    #[test]
    fn test_mage_client_make_request() {
        let mut mc = create_mega_client();

        let req = form_request_to("/api/v1/tree?repo_path=/projects/fuser");
        let output = mc.request(req).unwrap();
        dbg!(output);

        let req = form_request_to("/api/v1/tree?repo_path=/projects/mega");
        let output = mc.request(req).unwrap();
        dbg!(output);

        let req = form_request_to("/api/v1/object?object_id=8452eaa54f8482f9b36a70326393d169df654c28&repo_path=/projects/mega");
        let output = mc.request(req).unwrap();
        dbg!(output);

        let req =
            form_request_to("/api/v1/blob?object_id=8452eaa54f8482f9b36a70326393d169df654c28");
        let output = mc.request(req).unwrap();
        dbg!(output);
    }
}
