use wasm_minimal_protocol::*;
use serde_json::{from_slice, to_vec};

mod input;
mod tie;
mod voting;

use crate::input::arguments::{Args,prepare_ballots,prepare_candidates};

use crate::tie::tie::get_tie_method;
use crate::voting::vote::get_vote_method;


initiate_protocol!();

fn _output_text(text: &str) -> Vec<u8> {
    text.as_bytes().to_vec()
}

fn output_json(value: &impl serde::Serialize) -> Result<Vec<u8>, String> {
    let output = to_vec(value)
        .map_err(|e| format!("Failed to serialize output: {}", e))?;
    Ok(output)
}

#[wasm_func]
pub fn vote(data : &[u8]) -> Result<Vec<u8>, String> {
    let args = from_slice::<Args>(data)
        .map_err(|e| format!("Failed to parse args: {}", e))?;

    let candidates = prepare_candidates(&args.ballots)?;
    let ballots = prepare_ballots(args.ballots, &candidates);

    //return output_json(&tie::tie_count::TieCount::new(&ballots));

    let tie_method = get_tie_method(&args.tie_method, &ballots)?;
    let mut vote_method = get_vote_method(args.vote_method, candidates, ballots, tie_method)?;
    let result_lines = vote_method.vote()?;

    Ok(output_json(&result_lines)?)
}
