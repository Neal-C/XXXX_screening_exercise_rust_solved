#![allow(unused_parens)]
#![allow(unused_imports)]

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::hash_map::{Keys, Values};
use std::collections::{HashMap, HashSet};
use std::collections::btree_set::BTreeSet;
use std::rc::{Rc};


// enum PlayerMatchUp {
//     Draw(Ordering::Equal, Ordering::Equal),
//     Elimination(Ordering::Greater, Ordering::Less),
//     Weaker(Ordering::Less),
//     Stronger(Ordering::Greater)
// }

#[derive(Debug, PartialEq, Eq, Default, Hash, Ord, PartialOrd)]
struct ChessPlayer {
    elo: u16,
    age: u16,
    name: String
}

impl ChessPlayer {
    fn new<'player>(name: String, elo: u16, age: u16) -> Self {
        return Self {
            name,
            elo,
            age
        };
    }
}

impl ChessPlayer {

    fn match_up<'player>(self, other_player: Rc<Box<ChessPlayer>>) -> Rc<Box<ChessPlayer>>{

        if self.elo > other_player.elo {
            return Rc::new(Box::new(self));
        } else {
            return other_player;
        }
        // return self.elo > other_player.elo ? self : other_player;
        // return ((self.age.cmp(&other_player.age)), (self.elo.cmp(&other_player.elo)));
    }

    fn is_draw_against_player<'player>(&'player self, other_player: &ChessPlayer) -> bool {
        return self.elo == other_player.elo;
    }

    fn is_eliminated_by<'player>(&'player self, other_player : &ChessPlayer) -> bool {
        return (self.age >= other_player.age && self.elo < other_player.elo) || (self.age >=
            other_player.age && self.elo == other_player.elo)
    }
}


fn get_champions(participants: Vec<ChessPlayer>) -> Vec<Rc<Box<ChessPlayer>>> {
    if participants.is_empty() {
        return vec![];
    }

    // let record_register : HashMap<String, RecordRegister> = HashMap::new();
    let mut record  :HashMap<u16,Rc<Box<ChessPlayer>>> = HashMap::new();
    //Maybe BTreeMap or BinaryHeap ?
    let mut draws = Vec::new();

    for player in participants {
        let category = player.age;
        let current_champion = record.entry(category).or_insert(Rc::new(Box::new(ChessPlayer::default())));
        // let current_champion = match record.get(&category){
        //     Some(champion) => champion,
        //     None => {
        //       Rc::new(Player::default())
        //     }
        // };


        if player.is_draw_against_player(current_champion){
            draws.push((Rc::clone(current_champion)));
            draws.push(Rc::new(Box::new(player)));
            continue;
        }
        let strongest_at_category_age = player.match_up(Rc::clone(current_champion));
        record.insert(category, strongest_at_category_age);
    }

    let mut bests_by_age= Vec::from_iter(record.values());
    for player in draws.iter() {
        bests_by_age.push(player);
    }


    let mut bests_by_age = Vec::from_iter((BTreeSet::<_>::from_iter(bests_by_age)));

    bests_by_age.sort_by_key(|element| element.age);

    let mut ascending_ordered_ages= Vec::from_iter(record.keys());
    ascending_ordered_ages.sort();

    let mut champions_list = Vec::new();

    for player in bests_by_age.iter() {

        for age_category in ascending_ordered_ages.iter() {
            if(**age_category == player.age) {
                champions_list.push(Rc::clone(player));
                break;
            }
            if(player.is_eliminated_by(record.get(age_category).unwrap())){
                break;
            }
        }

    }

    return champions_list;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mary_and_peter_are_champions(){
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer::new(String::from("Jean"), 1000, 10),
            ChessPlayer::new(String::from("Mary"), 1100, 9),
            ChessPlayer::new(String::from("Peter"), 1200, 11)
        ];
        let mary_and_peter= vec![
            Rc::new(Box::new(ChessPlayer::new(String::from("Mary"), 1100, 9))),
            Rc::new(Box::new(ChessPlayer::new(String::from("Peter"), 1200, 11)))
        ];
        //ACT
        let result = get_champions(player_list);
        //ASSERT
        assert_eq!(result, mary_and_peter);
    }

    #[test]
    fn a_non_empty_vec_must_return_champions(){
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer::new(String::from("Jean"), 1000, 10),
            ChessPlayer::new(String::from("Mary"), 1100, 9),
            ChessPlayer::new(String::from("Peter"), 1200, 11)
        ];
        //ACT
        let result = get_champions(player_list);
        //ASSERT
        assert!(!result.is_empty());
    }

    #[test]
    fn return_goats(){
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer::new(String::from("Sherlock"), 3800, 30),
            ChessPlayer::new(String::from("Magnus"), 4200, 31),
            ChessPlayer::new(String::from("Francis"), 3700, 18),
            ChessPlayer::new(String::from("Erik Lehnsherr"), 3200, 31),
            ChessPlayer::new(String::from("Charles Xavier"), 3100, 32),
            ChessPlayer::new(String::from("Daniil"), 3300, 33),
        ];
        let goats= vec![
            Rc::new(Box::new(ChessPlayer::new(String::from("Francis"), 3700, 18))),
            Rc::new(Box::new(ChessPlayer::new(String::from("Sherlock"), 3800, 30))),
            Rc::new(Box::new(ChessPlayer::new(String::from("Magnus"), 4200, 31))),
        ];
        //ACT
        let result = get_champions(player_list);
        //ASSERT
        assert_eq!(result, goats)
    }

    #[test]
    fn account_for_draws(){
        //ARRANGE
        let player_list: Vec<ChessPlayer> = vec![
            ChessPlayer::new(String::from("Sherlock"), 4000, 30),
            ChessPlayer::new(String::from("Magnus"), 4200, 31),
            ChessPlayer::new(String::from("Francis"), 3700, 18),
            ChessPlayer::new(String::from("Erik Lehnsherr"), 4000, 30),
            ChessPlayer::new(String::from("Charles Xavier"), 3500, 40),
            ChessPlayer::new(String::from("Daniil"), 3200, 31),
        ];
        let goats= vec![
            Rc::new(Box::new(ChessPlayer::new(String::from("Francis"), 3700, 18))),
            Rc::new(Box::new(ChessPlayer::new(String::from("Erik Lehnsherr"), 4000, 30))),
            Rc::new(Box::new(ChessPlayer::new(String::from("Sherlock"), 4000, 30))),
            Rc::new(Box::new(ChessPlayer::new(String::from("Magnus"), 4200, 31)))
        ];
        //ACT
        let result = get_champions(player_list);
        //ASSERT
        assert_eq!(result, goats)
    }

}