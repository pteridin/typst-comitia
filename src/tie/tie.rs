use crate::input::arguments::ArgsTieMethod;
use crate::voting::results::VoteResultLine;

use crate::tie::tie_random::TieRandom;
use crate::tie::tie_all::TieAll;

pub trait TieMethod {
    fn break_winning_tie(&self, result_lines : &mut Vec<VoteResultLine>, ballots: &Vec<Vec<u8>>) -> Result<Vec<u8>, String>;
    fn break_eliminate_tie(&self, result_lines : &mut Vec<VoteResultLine>, ballots: &Vec<Vec<u8>>)  -> Result<Vec<u8>, String>;
}

pub fn get_tie_method(ties_method: &ArgsTieMethod, ballots : &Vec<Vec<u8>>) -> Result<Box<dyn TieMethod>, String> {
    match ties_method {
        ArgsTieMethod::Random => Ok(Box::new(TieRandom {})),
        ArgsTieMethod::All => Ok(Box::new(TieAll {})),
        ArgsTieMethod::Count => Ok(Box::new(crate::tie::tie_count::TieCount::new(&ballots))),
    }
}


