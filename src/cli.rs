use crate::oneiromancer::{OLLAMA_BASEURL, OLLAMA_MODEL};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "Oneiromancer", about = "Reverse engineering assistant that uses a locally running LLM to aid with pseudocode analysis.", long_about = None, version)]
pub struct Args {
    pub binary: PathBuf,

    #[arg(short, long, env = "OLLAMA_BASEURL", default_value = OLLAMA_BASEURL)]
    pub base_url: String,

    #[arg(short, long, env = "OLLAMA_MODEL", default_value = OLLAMA_MODEL)]
    pub model: String,
}
