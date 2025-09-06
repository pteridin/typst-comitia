use crate::tie::tie::TieMethod;
use crate::voting::results::VoteResultLine;

pub struct TieAll;

impl TieMethod for TieAll {
    fn break_winning_tie(&self, result_lines: &mut Vec<VoteResultLine>, _ballots: &Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
        let mut winners = Vec::new();
        for line in result_lines.iter_mut() {
            if line.is_winner_candidate {
                line.is_winner = true;
                winners.push(line.id);
            }
        }
        Ok(winners)
    }

    fn break_eliminate_tie(&self, result_lines: &mut Vec<VoteResultLine>, _ballots: &Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
        let mut eliminations = Vec::new();
        for line in result_lines.iter_mut() {
            if line.is_elimination_candidate {
                line.is_eliminated = true;
                eliminations.push(line.id);
            }
        }
        Ok(eliminations)
    }
}