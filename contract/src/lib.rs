/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

use std::collections::{HashMap};

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VoteApp{
    has_vote: HashMap<String, String>, // has votes
    votes: HashMap<String, u128>, // vote score
    candidates: Vec<String>,
    round: u128,
    winner: String,
    score: u128,
}

impl Default for VoteApp {
  fn default() -> Self {
    Self {
      has_vote: HashMap::new(),
      votes: HashMap::new(),
      candidates: Vec::new(),
      round: 0,
      winner: String::from(""),
      score: 0,
    }
  }
}

#[near_bindgen]
impl VoteApp {
    pub fn last_winner(&self) -> (String, u128) {
      (self.winner.clone(), self.score)
    }
    pub fn winner(&self) -> (String, u128) {
      if self.candidates.len() == 0 {
        return ("none".to_string(), 0);
      }
      let mut winner = self.candidates.get(0).unwrap();
      let mut max_score = *self.votes.get(winner).unwrap();
      for i in 1..self.candidates.len() {
        let candidate = self.candidates.get(i).unwrap();
        let score = *self.votes.get(candidate).unwrap();
        if score > max_score {
          winner = candidate;
          max_score = score;
        }
      }
      (winner.to_string(), max_score)
    }
    pub fn get_round(&self) -> u128 {
      self.round
    }
    pub fn get_score(&self, candidate: String) -> u128 {
      *self.votes.get(&candidate).unwrap_or(&0)
    }
    pub fn get_candidates(&self) -> Vec<String> {
        self.candidates.iter().map(|x| x.clone()).collect()
    }
    pub fn next_round(&mut self) -> (String, u128) {
      let result = self.winner();
      self.winner = result.0.clone();
      self.score = result.1;
      self.round += 1;
      self.has_vote = HashMap::new();
      self.votes = HashMap::new();
      self.candidates = Vec::new();
      result
    }
    pub fn add_candidate(&mut self, candidate: String) {
        self.candidates.push(candidate);
    }
    pub fn vote(&mut self, candidate: String) -> bool {
      if self.has_vote.get(&env::predecessor_account_id()).unwrap_or(&String::from("false")) == "true" {
        return false
      }
      self.has_vote.insert(env::predecessor_account_id(), String::from("true"));
      let cnt = self.votes.get(&candidate).unwrap_or(&0);
      let incr = *cnt + 1;
      self.votes.insert(candidate, incr);
      return true
    }
}
