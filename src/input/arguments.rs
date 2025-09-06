use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArgsVoteMethod {
    Plurality,
    STV,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArgsTieMethod {
    Random,
    All,
    Count,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Args {
    pub ballots : Vec<Vec<String>>,
    pub vote_method : ArgsVoteMethod,
    pub tie_method : ArgsTieMethod,
}


pub fn prepare_candidates(ballots: &[Vec<String>]) -> Result<Vec<String>, String> {
    let mut candidates = HashSet::new();
    for ballot in ballots {
        for candidate in ballot {
            if !candidates.contains(candidate) {
                candidates.insert(candidate.clone());
            }
        }
    } 
    let mut candidates : Vec<String> = candidates.into_iter().collect();
    candidates.sort();

    if candidates.is_empty() {
        return Err("No candidates found in ballots".to_string());
    }

    if candidates.len() > u8::MAX as usize {
        return Err("Too many candidates (max 255)".to_string());
    }

    Ok(candidates)
}

pub fn prepare_ballots(ballots: Vec<Vec<String>>, candidates: &[String]) -> Vec<Vec<u8>> {
    let candidate_ids = candidates.iter().enumerate()
        .map(|(i, c)| (c.clone(), i as u8))
        .collect::<std::collections::HashMap<String, u8>>();

    let mut prepared_ballots = Vec::new();
    for ballot in ballots {
        let mut prepared_ballot = Vec::new();
        for candidate in ballot {
            if let Some(index) = candidate_ids.get(&candidate) {
                prepared_ballot.push(*index);
            }
        }
        prepared_ballots.push(prepared_ballot);
    }

    prepared_ballots
}

