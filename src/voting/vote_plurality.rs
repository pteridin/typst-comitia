use super::results::VoteResultLine;
use crate::{voting::vote::VoteMethod};
use crate::tie::tie::TieMethod;

pub struct VotePlurality {
    candidates: Vec<String>,
    ballots: Vec<Vec<u8>>,
    tie_method: Box<dyn TieMethod>,
}

impl VotePlurality {
    pub fn new(candidates : Vec<String>, ballots : Vec<Vec<u8>>, tie_method: Box<dyn TieMethod>) -> Result<Self, String> {
            Ok(Self { 
                candidates, 
                ballots, 
                tie_method 
            })
    }
}

impl VoteMethod for VotePlurality {
    fn vote(&mut self) -> Result<Vec<VoteResultLine>, String> {
        let counts = Self::sum_first_ballots(&self.ballots);
        let total_votes = self.ballots.len();

        // First determine the overall voting results
        let mut result_lines : Vec<VoteResultLine> = counts.iter().map(|(id, &votes)| {
            let percentage = (votes as f64 / total_votes as f64) * 100.0;
            VoteResultLine {
                run: 1,
                candidate: self.candidates[*id as usize].clone(),
                id: *id,
                votes,
                total_votes,
                percentage,
                is_winner: false, // To be determined
                is_winner_candidate: false, // To be determined
                is_eliminated: false, // Not applicable in plurality
                is_elimination_candidate: false, // Not applicable in plurality
            }
        }).collect();

        Self::mark_winner_candidates(&mut result_lines);
        Self::mark_winner(&mut result_lines, &self.ballots, &self.tie_method)?;

        result_lines.sort_by(|a, b| b.votes.cmp(&a.votes));
        Ok(result_lines)
    }
}