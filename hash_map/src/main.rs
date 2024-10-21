use std::collections::HashMap;

fn main() {
    println!("hash map 储存键值对");
    let mut h = HashMap::new();
    h.insert("value", 3);
    h.insert("color", 255);
    println!("{:?}",h);
    for i in &h {
        println!("{:?}",i);
    }

    let mut scores: HashMap<String,i32> = HashMap::new();
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 10);

    //let mut team_score = vec![TeamScore::Team(String::from("Green"))];

    let team_score = vec![
        TeamScore {
            team:String::from("Green"),
            score:20,
        },
        TeamScore {
            team:String::from("Red"),
            score:80,
        },
    ];
    for i in &team_score {
        println!("{:?}",i);
    }
}

// enum TeamScore {
//     Team(String),
//     Score(i32),
// }

#[derive(Debug)]
struct TeamScore {
    team:String,
    score:i32,
}