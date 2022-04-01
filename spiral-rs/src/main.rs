use serde_json::Value;
use spiral_rs::util::*;
use spiral_rs::client::*;
use std::env;

fn send_api_req_text(path: &str, data: Vec<u8>) -> Option<String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(None)
        .build().unwrap();
    client.post(format!("https://spiralwiki.com:8088{}", path))
        .body(data)
        .send()
        .ok()?
        .text()
        .ok()
}

fn send_api_req_vec(path: &str, data: Vec<u8>) -> Option<Vec<u8>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(None)
        .build().unwrap();
    Some(client.post(format!("https://spiralwiki.com:8088{}", path))
        .body(data)
        .send()
        .ok()?
        .bytes()
        .ok()?
        .to_vec())
}

fn main() {
    let cfg = r#"
        {'n': 2,
        'nu_1': 9,
        'nu_2': 6,
        'p': 256,
        'q_prime_bits': 20,
        's_e': 87.62938774292914,
        't_GSW': 8,
        't_conv': 4,
        't_exp': 8,
        't_exp_right': 56}
    "#;
    let cfg = cfg.replace("'", "\"");
    let params = params_from_json(&cfg);

    let args: Vec<String> = env::args().collect();
    let idx_target: usize = (&args[1]).parse().unwrap();

    println!("initializing client");
    let mut c = Client::init(&params);
    println!("generating public parameters");
    let pub_params = c.generate_keys();
    let pub_params_buf = pub_params.serialize();
    println!("pub_params size {}", pub_params_buf.len());
    let query = c.generate_query(idx_target);
    let mut query_buf = query.serialize();
    println!("initial query size {}", query_buf.len());

    let setup_resp_str = send_api_req_text("/setup", pub_params_buf).unwrap();
    println!("{}", setup_resp_str);
    let resp_json: Value = serde_json::from_str(&setup_resp_str).unwrap();
    let id = resp_json["id"].as_str().unwrap();
    let mut full_query_buf = id.as_bytes().to_vec();
    full_query_buf.append(&mut query_buf);
    let query_resp = send_api_req_vec("/query", full_query_buf).unwrap();
    println!("query_resp len {}", query_resp.len());

    let _result = c.decode_response(query_resp.as_slice());
    // println!("{:x?}", result);
}