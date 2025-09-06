use std::usize;

use crate::tie::tie::TieMethod;
use crate::voting::results::VoteResultLine;

#[derive(serde::Serialize)]
pub struct TieCount {
    counts : Vec<usize>,
}

impl TieCount {
    fn count_all_ballots(ballots: &[Vec<u8>]) -> Vec<usize> {
        let mut counts = vec![0; u8::MAX as usize + 1];
        for ballot in ballots {
            for &choice in ballot {
                counts[choice as usize] += 1;
            }
        }
        counts
    }

    pub fn new(ballots: &Vec<Vec<u8>>) -> Self {
        Self { 
            counts: Self::count_all_ballots(&ballots) 
        }
    }
}

impl TieMethod for TieCount {
    fn break_winning_tie(&self, result_lines: &mut Vec<VoteResultLine>, _ballots: &Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
        let mut max_score = 0;
        for candidate in result_lines.iter() {
            let id: usize = candidate.id as usize;
            if candidate.is_winner_candidate && self.counts[id as usize] > max_score {
                max_score = self.counts[id];
            }
        }

        if max_score == 0 {
            return Ok(Vec::new());
        }

        let mut winners = Vec::new();
        for candidate in result_lines {
            let id: usize = candidate.id as usize;
            if candidate.is_winner_candidate && self.counts[id] == max_score {
                candidate.is_winner = true;
                winners.push(candidate.id);
            }
        }

        Ok(winners)
    }

    fn break_eliminate_tie(&self, result_lines: &mut Vec<VoteResultLine>, _ballots: &Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
        let mut min_score = usize::MAX;
        for candidate in result_lines.iter() {
            let id: usize = candidate.id as usize;
            if candidate.is_elimination_candidate && self.counts[id as usize] < min_score {
                min_score = self.counts[id];
            }
        }

        if min_score == usize::MAX {
            return Ok(Vec::new());
        }

        let mut eliminations = Vec::new();
        for candidate in result_lines {
            let id: usize = candidate.id as usize;
            if candidate.is_elimination_candidate && self.counts[id] == min_score {
                candidate.is_eliminated = true;
                eliminations.push(candidate.id);
            }
        }

        Ok(eliminations)
    }
}