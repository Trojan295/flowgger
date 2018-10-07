use super::Output;
use crate::flowgger::config::Config;
use crate::flowgger::merger::Merger;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;


pub struct ElasticsearchOutput;

impl ElasticsearchOutput {
    pub fn new(config: &Config) -> ElasticsearchOutput {
        ElasticsearchOutput{}
    }
}

impl Output for ElasticsearchOutput {
    fn start(&self, arx: Arc<Mutex<Receiver<Vec<u8>>>>, merger: Option<Box<Merger>>) {
    }
}
