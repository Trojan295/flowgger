use super::Output;
use crate::flowgger::config::Config;
use crate::flowgger::merger::Merger;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

use std::io::{self, Write};
use hyper::{Method, Request, Client, Body};
use hyper::rt::{self, Future, Stream};


pub struct ElasticsearchOutput {
    endpoint: String,
    index: String,
}

struct ElasticsearchWorker {
    endpoint: String,
    index: String,
    arx: Arc<Mutex<Receiver<Vec<u8>>>>,
}

impl ElasticsearchWorker {
    fn new(endpoint: String, index: String, arx: Arc<Mutex<Receiver<Vec<u8>>>>) -> ElasticsearchWorker {
        ElasticsearchWorker{
            endpoint,
            index,
            arx,
        }
    }

    fn run(&self) {
        let client = Client::new();

        let uri = format!("{}/{}", &self.endpoint, &self.index);

        loop {
            let bytes = match self.arx.lock().unwrap().recv() {
                Ok(data) => data,
                Err(_) => return,
            };

            println!("New data");

            //let out = String::from_utf8_lossy(&bytes);

            let mut request = Request::new(bytes);
            client.request(*request)
                .and_then(|res| {
                    println!("Response {}", res.status());
                });
        }
    }
}

impl ElasticsearchOutput {
    pub fn new(config: &Config) -> ElasticsearchOutput {
        let endpoint = config
            .lookup("output.elasticsearch_endpoint")
            .expect("output.elasticsearch_endpoint is required")
            .as_str()
            .expect("output.elasticsearch_endpoint must be a string")
            .to_string();
        let index = config
            .lookup("output.elasticsearch_index")
            .expect("output.elasticsearch_index is required")
            .as_str()
            .expect("output.elasticsearch_index must be a string")
            .to_string();
        ElasticsearchOutput{ endpoint, index }
    }
}

impl Output for ElasticsearchOutput {
    fn start(&self, arx: Arc<Mutex<Receiver<Vec<u8>>>>, merger: Option<Box<Merger>>) {
        let arx = Arc::clone(&arx);
        let endpoint = self.endpoint.clone();
        let index = self.index.clone();
        thread::spawn(move || {
            let mut worker = ElasticsearchWorker::new(endpoint, index, arx);
            worker.run();
        });
    }
}
