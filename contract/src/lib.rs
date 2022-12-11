// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::{ LookupMap, LazyOption, Vector, UnorderedSet, UnorderedMap };
use near_sdk::{log, near_bindgen, PanicOnDefault, AccountId, BorshStorageKey};

// Read-only methods
pub mod views;

// Possible game outcomes
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(PartialEq)]
pub enum GameOutcome {
    WinA,
    WinB,
}

// Possible game states
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum GameState {
    Ongoing,
    Finished{ outcome: GameOutcome},
    Resolved{ outcome: GameOutcome},
    Disputed,
}

// A single game and it's participants
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Game {
    state: GameState,
    reward: u64,
    team_a: Vec<AccountId>,
    team_b: Vec<AccountId>,    
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Games,
    Ongoing,
    Finished,
    Resolved,
    Disputed,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // Games setup data
    games: LookupMap<u64, Game>,
    // Relevant indexes on-chain
    ongoing: UnorderedSet<u64>,
    finished: UnorderedSet<u64>,
    resolved: UnorderedSet<u64>,
    disputed: UnorderedSet<u64>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self{
        Self{
            games: LookupMap::new(StorageKey::Games),
            ongoing: UnorderedSet::new(StorageKey::Ongoing),
            finished: UnorderedSet::new(StorageKey::Finished),
            resolved: UnorderedSet::new(StorageKey::Resolved),
            disputed: UnorderedSet::new(StorageKey::Disputed),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {    
    // Start a game by providing listing of teams, and the total reward in NEAR
    #[payable]
    pub fn start_game(&mut self, team_a: Vec<AccountId>, team_b: Vec<AccountId>) -> u64 {
        let game = Game {
            state: GameState::Ongoing,
            reward: 1, 
            team_a,
            team_b,
        };
        self.games.insert(&0, &game);
        log!("Created a new game");
        0
    }

    // Captains of the teams should declare result
    pub fn finish_game(&mut self, game_id: u64, result: GameOutcome) {        
        // update game state
        if let Some(mut game) = self.games.get(&game_id) {
            match game.state {
                GameState::Ongoing => {
                    game.state = GameState::Finished { outcome: result };
                    log!("Captain A declared game as finished");
                },
                GameState::Finished { outcome } => {
                    if outcome == result {
                        game.state = GameState::Resolved { outcome };    
                    } else {
                        game.state = GameState::Disputed;    
                    }                    
                    log!("Captain B declared game as finished");
                }
                _ => {
                    log!("Can't finish the game as it's already finished.");
                },
            }
            self.games.insert(&game_id, &game);
        } else {
            log!("Incorrect game id.")            
        }    
    }

    // In case of dispute only admin can resolve it    
    pub fn resolve_game(&mut self, game_id: u64, result: GameOutcome) {
        if let Some(mut game) = self.games.get(&game_id) {
            game.state = GameState::Resolved { outcome: result };
            self.games.insert(&game_id, &game);
        } else {
            log!("Incorrect game id.");            
        }        
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let mut contract = Contract::default();        
    }
}
