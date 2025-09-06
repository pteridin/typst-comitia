use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct VoteResultLine {
    // Marks the number of iteration cycles
    pub run : usize,
    // The candidate name
    pub candidate : String,
    // Unique id of the candidate
    pub id : u8,
    // Number of votes the candidate received in this round
    pub votes : usize,
    // Total number of votes available
    pub total_votes : usize,
    // Percentage of votes the candidate received in this round
    pub percentage : f64,
    // Whether the candidate is a winner or not
    pub is_winner : bool,
    // Whether the candidate could have been a winner or not - e.g. in ties
    pub is_winner_candidate : bool,
    // Whether the candidate has been eliminated or not
    pub is_eliminated : bool,
    // Whether the candidate could be eliminated or not - e.g. in ties
    pub is_elimination_candidate : bool,
}