fn main() {
    let mut teams: Vec<Team> = vec![
        Team::new(vec![
        Player{score: 10,mulitplyer:1.2},
        Player{score: 10,mulitplyer:1.4},
        Player{score: 10,mulitplyer:2.0}
        ]),
        Team::new(vec![
        Player{score: 10,mulitplyer:1.2},
        Player{score: 10,mulitplyer:1.4},
        Player{score: 10,mulitplyer:2.0},
        Player{score: 10,mulitplyer:2.0},
        Player{score: 10,mulitplyer:2.0},
        Player{score: 10,mulitplyer:2.0},
        Player{score: 10,mulitplyer:2.0},
        ])
    ];
    //do a thing x times to teams
    //
    display_teams(&teams);
    teams[0].update_score();
    display_teams(&teams);
}

use std::vec;

#[derive(Debug, Clone)]
struct Player {
    score: u32,
    mulitplyer: f32
}

#[derive(Debug)]
struct Team {
    players: Vec<Player>,
}

impl Team {
    pub fn new(players: Vec<Player>) -> Self{ //constructor checks that max team size < 3
        //if players.len() == 0
        if players.len() > 3 {
            println!("Too many players, dropping {}", players.len() - 3);
            Team{players:players[0..3].to_vec()}
        } else {Team{players:players}}
    }
    pub fn update_score(&mut self) {
        self.players.iter_mut()
        .for_each(|p| p.score = (p.score as f32 * p.mulitplyer).floor() as u32);
    }
}

// impl Default for Team {
//     fn default() -> Self {
        
//     }
// }

fn display_teams(teams: &Vec<Team>) {
    for (i,team) in teams.iter().enumerate(){ //using iters and the enumerate function in a for loop
        println!("Team {}", i);
        for (i,player) in team.players.iter().enumerate(){
            println!("Player {}, score: {}, mult: {}",i,player.score, player.mulitplyer);
        }
    }
}

