use rand::prelude::*;
use std::fmt;
use std::vec;

fn main() {
    let initial_teams: Vec<Team> = vec![
        Team::default(),
        Team::default(),
    ];

    println!("INITIAL TEAMS:");
    display_teams(&initial_teams);

    let updated_teams: Vec<Team> = initial_teams.into_iter().map(|team| team.update_score()).collect();

    println!("SCORED TEAMS:");
    display_teams(&updated_teams);
}

#[derive(Debug, Clone)]
enum ScoreColor {
    Red,
    Blue,
    Yellow,
    Green,
}

impl fmt::Display for ScoreColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScoreColor::Red => write!(f, "Red"),
            ScoreColor::Blue => write!(f, "Blue"),
            ScoreColor::Yellow => write!(f, "Yellow"),
            ScoreColor::Green => write!(f, "Green"),
        }
    }
}

#[derive(Debug, Clone)]
struct Player {
    score: u32,
    mulitplyer: f32,
    score_time: ScoreColor,
}

#[derive(Debug)]
struct Team {
    players: Vec<Player>,
}

impl Team {
    pub fn update_score(&self) -> Self {
        let updated_players = self.players.iter()
            .map(|p| Player {
                score: ((p.score as f32 * p.mulitplyer).floor() as u32),
                mulitplyer: p.mulitplyer,
                score_time: p.score_time.clone(),
            })
            .collect();

        Team { players: updated_players }
    }
}

impl Default for Team {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let players = (0..3).map(|_| {
            let multiplier = ((rng.gen_range(1.0..3.0) * 100.0) as f32).floor() / 100.0;
            let color = match rng.gen_range(0..4) {
                0 => ScoreColor::Red,
                1 => ScoreColor::Blue,
                2 => ScoreColor::Yellow,
                _ => ScoreColor::Green, //if it's 3 OR there was an error and it's bigger
            };
            Player {
                score: 10,
                mulitplyer: multiplier,
                score_time: color,
            }
        }).collect();

        Team { players }
    }
}

fn display_teams(teams: &[Team]) {
    for (i, team) in teams.iter().enumerate() {
        println!("Team {}", i);
        for (i, player) in team.players.iter().enumerate() {
            println!("Player {}, score: {}, mult: {}, col: {}", i, player.score, player.mulitplyer, player.score_time);
        }
    }
}


