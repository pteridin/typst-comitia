use super::results::VoteResultLine;
use crate::voting::vote::VoteMethod;
use crate::tie::tie::TieMethod;

const MAX_ROUNDS: usize = 25;

pub struct VoteSTV {
    candidates: Vec<String>,
    ballots: Vec<Vec<u8>>,
    tie_method: Box<dyn TieMethod>,
}

impl VoteSTV {
    pub fn new(candidates : Vec<String>, ballots : Vec<Vec<u8>>, tie_method: Box<dyn TieMethod>) -> Result<Self, String> {
            Ok(Self { 
                candidates, 
                ballots, 
                tie_method 
            })
    }

    fn stv_mark_winner(&mut self, result_lines: &mut [VoteResultLine]) -> Option<u8>
    where Self: Sized {
        if result_lines.is_empty() {
            return None;
        }

        for line in result_lines.iter_mut() {
            if line.percentage > 50.0 {
                line.is_winner = true;
                line.is_winner_candidate = true;
                return Some(line.id);
            }
        }
        return None
    }
}

impl VoteMethod for VoteSTV {
    fn vote(&mut self) -> Result<Vec<VoteResultLine>, String> {
        let mut result_lines : Vec<VoteResultLine> = Vec::new();

        for i in 0..(MAX_ROUNDS+1) {
            if self.ballots.is_empty() {
                return Ok(result_lines);
            }

            let counts = Self::sum_first_ballots(&self.ballots);
            let total_votes = self.ballots.len();

            // First determine the overall voting results
            let mut current_result_lines : Vec<VoteResultLine> = counts.iter().map(|(id, &votes)| {
                let percentage = (votes as f64 / total_votes as f64) * 100.0;
                    VoteResultLine {
                        run: i+1,
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

            let winner_id = self.stv_mark_winner(current_result_lines.as_mut_slice());

            if winner_id.is_some() {
                current_result_lines.sort_by(|a, b| b.votes.cmp(&a.votes));
                result_lines.append(&mut current_result_lines);
                return Ok(result_lines)
            }

            Self::mark_elimination_candidates(&mut current_result_lines);
            let eliminated_ids = Self::mark_eliminations(&mut current_result_lines, &self.ballots, &self.tie_method)?;
            if eliminated_ids.is_empty() {
                current_result_lines.sort_by(|a, b| b.votes.cmp(&a.votes));
                result_lines.append(&mut current_result_lines);
                return Ok(result_lines);
            }

            // Remove eliminated candidates from ballots
            let eliminated_ids_set: std::collections::HashSet<u8> = eliminated_ids.into_iter().collect();
            
            let mut new_ballots = Vec::new();
            for ballot in &self.ballots {
                let new_ballot: Vec<u8> = ballot.iter()
                    .filter(|&candidate| !eliminated_ids_set.contains(candidate))
                    .cloned()
                    .collect();

                if new_ballot.is_empty() {
                    continue;
                }
                new_ballots.push(new_ballot);
            }
            self.ballots = new_ballots;

            current_result_lines.sort_by(|a, b| b.votes.cmp(&a.votes));
            result_lines.append(&mut current_result_lines);
        }
        
        Ok(result_lines)
    }
}