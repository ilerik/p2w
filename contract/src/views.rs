use crate::*;

#[near_bindgen]
impl Contract {

    /// Return game data
    pub fn get_game_by_id(self, game_id: u64) -> Option<Game> {        
        let data = self.games.get(&game_id);
        match data {
            Some(game) => Some(game),
            None => None
        }
    }

}