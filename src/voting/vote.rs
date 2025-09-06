use std::collections::BTreeMap;
use std::vec;
use crate::input::arguments::ArgsVoteMethod;
use crate::voting::results::VoteResultLine;
use crate::tie::tie::TieMethod;

use crate::voting::vote_stv::VoteSTV;
use crate::voting::vote_plurality::VotePlurality;

pub fn get_vote_method(vote_method: ArgsVoteMethod, candidates : Vec<String>, ballots : Vec<Vec<u8>>, tie_method : Box<dyn TieMethod>) -> Result<Box<dyn VoteMethod>, String> {
    let vote_method: Box<dyn VoteMethod> = match vote_method {
        ArgsVoteMethod::Plurality => Box::new(VotePlurality::new(candidates, ballots, tie_method)?),
        ArgsVoteMethod::STV => Box::new(VoteSTV::new(candidates, ballots, tie_method)?),
    };
    Ok(vote_method)
}

pub trait VoteMethod {
    fn vote(&mut self) -> Result<Vec<VoteResultLine>, String>;
    
    fn sum_first_ballots(ballots: &[Vec<u8>]) -> BTreeMap<u8, usize>      
    where Self: Sized {
        let mut counts = BTreeMap::new();
        for ballot in ballots {
            if let Some(first_choice) = ballot.first() {
                let entry = counts.entry(*first_choice).or_insert(0);
                *entry += 1;
            }
        }
        counts
    }

    fn mark_winner_candidates(result_lines: &mut [VoteResultLine])      
    where Self: Sized {
        if result_lines.is_empty() {
            return;
        }

        let max_votes = result_lines.iter().map(|line| line.votes).max().unwrap_or(0);
        for line in result_lines.iter_mut() {
            if line.votes == max_votes && max_votes > 0 {
                line.is_winner = false;
                line.is_winner_candidate = true;
            }
        }
    }

    fn mark_winner(result_lines: &mut Vec<VoteResultLine>, ballots : &Vec<Vec<u8>>, tie_method: &Box<dyn TieMethod>) -> Result<(), String>      
    where Self: Sized {
        if result_lines.is_empty() {
            return Ok(());
        }

        let winner_candidates = result_lines.iter().filter(|line| line.is_winner_candidate);
        let n_winners = winner_candidates.count();
        
        if n_winners == 0 {
            return Ok(());
        }

        if n_winners == 1 {
            result_lines.iter_mut()
                .find(|line| line.is_winner_candidate)
                .map(|line| line.is_winner = true);
            return Ok(());
        }

        //let tie_method = get_tie_method(&ties_method, ballots)?;
        let _ = tie_method.break_winning_tie(result_lines, ballots)?;
        Ok(())
    }

    fn mark_elimination_candidates(result_lines: &mut [VoteResultLine])      
    where Self: Sized {
        if result_lines.is_empty() {
            return;
        }

        let min_votes = result_lines.iter().map(|line| line.votes).min().unwrap_or(0);
        for line in result_lines.iter_mut() {
            if line.votes == min_votes {
                line.is_eliminated = false;
                line.is_elimination_candidate = true;
            }
        }
    }

    fn mark_eliminations(result_lines: &mut Vec<VoteResultLine>, ballots : &Vec<Vec<u8>>, tie_method: &Box<dyn TieMethod>) -> Result<Vec<u8>, String> 
    where Self: Sized {
        if result_lines.is_empty() {
            return Ok(vec![]);
        }

        let elimination_candidates = result_lines.iter().filter(|line| line.is_elimination_candidate);
        let n_eliminations = elimination_candidates.count();
        
        if n_eliminations == 0 {
            return Ok(vec![]);
        }

        if n_eliminations == 1 {
            let elimination_id = result_lines.iter_mut()
                .find(|line| line.is_elimination_candidate)
                .map(|line| {
                    line.is_eliminated = true;
                    line.id.clone()
                })
                .unwrap();
            return Ok(vec![elimination_id]);
        }

        let elimination_ids = tie_method.break_eliminate_tie(result_lines, &ballots)?;
        Ok(elimination_ids)
    }

    fn _check_winners(result_lines: &[VoteResultLine]) -> bool      
    where Self: Sized {
        result_lines.iter().any(|line| line.is_winner)
    }
}

