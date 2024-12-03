use sha2::{Sha256, Digest};
use std::time::Instant;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct MiningJob {
    job_id: String,
    prev_hash: String,
    coinb1: String,
    coinb2: String,
    merkle_branch: Vec<String>,
    version: String,
    nbits: String,
    ntime: String,
    clean_jobs: bool,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <mining_address> <pool_url>", args[0]);
        return;
    }

    let mining_address = &args[1];
    let pool_url = &args[2];

    let client = Client::new();
    let job = get_mining_job(&client, pool_url).await.unwrap();

    let difficulty = 4; // 设定挖矿难度
    let prefix = "0".repeat(difficulty);
    let mut nonce = 0;

    let start = Instant::now();

    loop {
        let hash = calculate_hash(&job, nonce);
        if hash.starts_with(&prefix) {
            println!("Block mined! Nonce: {}, Hash: {}", nonce, hash);
            submit_solution(&client, pool_url, &job, nonce, mining_address).await.unwrap();
            break;
        }
        nonce += 1;
    }

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}

async fn get_mining_job(client: &Client, pool_url: &str) -> Result<MiningJob, reqwest::Error> {
    let response = client.get(pool_url).send().await?.json::<MiningJob>().await?;
    Ok(response)
}

async fn submit_solution(client: &Client, pool_url: &str, job: &MiningJob, nonce: u64, mining_address: &str) -> Result<(), reqwest::Error> {
    let solution = format!("{}:{}:{}", job.job_id, nonce, mining_address);
    let response = client.post(pool_url).json(&solution).send().await?;
    println!("Solution submitted: {:?}", response);
    Ok(())
}

fn calculate_hash(job: &MiningJob, nonce: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&job.prev_hash);
    hasher.update(nonce.to_string());
    let result = hasher.finalize();
    format!("{:x}", result)
}
