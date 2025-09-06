use std::vec;

use crate::tie::tie::TieMethod;
use crate::voting::results::VoteResultLine;

pub struct TieRandom;

const RANDOM_SEED: u64 = 2_389_702_938_709;

impl TieRandom {
    fn deterministic_not_so_random_number(input: Vec<u8>) -> u64 {
        use std::hash::Hasher;
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        hasher.write(&input);
        hasher.finish()
    }

    fn determine_random_id(candidates : &[&mut VoteResultLine]) -> Result<u8, String> {
        if candidates.is_empty() {
            return Err("No candidates provided for random selection".to_string());
        }

        let mut number = Vec::new();
        
        for line in candidates.iter() {
            let last_number = number.last().map(|(_, x)| *x).unwrap_or(RANDOM_SEED);
            let rand_input = [
                last_number.to_le_bytes().to_vec(),
                line.run.to_le_bytes().to_vec(),
                line.candidate.clone().into_bytes().to_vec(),
                line.id.to_le_bytes().to_vec(),
                line.votes.to_le_bytes().to_vec(),
                line.total_votes.to_le_bytes().to_vec(),
            ].concat();
            let rand_number = Self::deterministic_not_so_random_number(rand_input);
            number.push((line.id as usize, rand_number));
        }

        number.sort_by(|a, b| a.1.cmp(&b.1));
        let id = number.get(0).map(|(id, _)| *id).expect("Expected at least one candidate");
        Ok(id as u8)
    }
}


impl TieMethod for TieRandom {
    fn break_winning_tie(&self, result_lines: &mut Vec<VoteResultLine>, _ballots: &Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
        // determine winning candidates
        let mut winning_candidates: Vec<&mut VoteResultLine> = result_lines.iter_mut()
            .filter(|line| line.is_winner_candidate)
            .collect();

        if winning_candidates.is_empty() {
            return Ok(Vec::new());
        }

        // set one random winner
        let winner_id = Self::determine_random_id(&winning_candidates)?;
        winning_candidates
            .iter_mut()
            .filter(|line| line.id == winner_id)
            .for_each(|line| line.is_winner = true);

        Ok(vec![winner_id])
    }

    fn break_eliminate_tie(&self, result_lines: &mut Vec<VoteResultLine>, _ballots: &Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
        // determine winning candidates
        let mut elimination_candidates: Vec<&mut VoteResultLine> = result_lines.iter_mut()
            .filter(|line| line.is_elimination_candidate)
            .collect();

        if elimination_candidates.is_empty() {
            return Ok(Vec::new());
        }

        // set one random winner
        let elimination_id = Self::determine_random_id(&elimination_candidates)?;
        elimination_candidates
            .iter_mut()
            .filter(|line| line.id == elimination_id)
            .for_each(|line| line.is_eliminated = true);

        Ok(vec![elimination_id])
    }
}