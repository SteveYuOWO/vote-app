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

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::collections::{LookupMap, Vector};

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VoteApp{
    has_vote: LookupMap<String, String>, // has votes
    votes: LookupMap<String, u128>, // vote score
    candidates: Vector<String>,
    round: u128,
    winner: Vector<String>,
}

impl Default for VoteApp {
  fn default() -> Self {
    Self {
      has_vote: LookupMap::new(b"b".to_vec()),
      votes: LookupMap::new(b"c".to_vec()),
      candidates: Vector::new(b"d".to_vec()),
      round: 0,
      winner: Vector::new(b"e".to_vec()),
    }
  }
}

#[near_bindgen]
impl VoteApp {
    pub fn winner(&self) -> (String, u128) {
      if self.candidates.len() == 0 {
        return ("none".to_string(), 0);
      }
      let mut winner = self.candidates.get(0).unwrap();
      let mut max_score = self.votes.get(&winner).unwrap_or(0);
      for i in 1..self.candidates.len() {
        let candidate = self.candidates.get(i).unwrap();
        let score = self.votes.get(&candidate).unwrap_or(0);
        if score > max_score {
          winner = candidate;
          max_score = score;
        }
      }
      (winner, max_score)
    }
    pub fn next_round(&mut self) -> (String, u128) {
      self.round += 1;
      self.has_vote = LookupMap::new(b"b".to_vec());
      self.votes = LookupMap::new(b"c".to_vec());
      self.candidates.clear();
      self.winner()
    }
    pub fn add_candidate(&mut self, candidate: String) {
        self.candidates.push(&candidate);
    }
    pub fn get_candidates(&self) -> Vec<String> {
        self.candidates.iter().map(|x| x.clone()).collect()
    }
    pub fn vote(&mut self, candidate: String) -> bool {
      if self.has_vote.get(&env::predecessor_account_id()).unwrap_or(String::from("false")) == "false" {
        return false
      }
      self.has_vote.insert(&candidate, &String::from("true"));
      let mut cnt = self.votes.get(&candidate).unwrap_or(0);
      cnt = cnt + 1;
      self.votes.insert(&candidate, &cnt);
      return true
    }
    pub fn get_score(&mut self, candidate: String) -> u128 {
      self.votes.get(&candidate).unwrap_or(0)
    }
}