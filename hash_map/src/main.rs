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

    //let tup = [1,2,3];
    //let tup = (1,2);
    //let tup = (1,3.14,"昔我往矣，杨柳依依",String::from("今我来思，雨雪霏霏"));

    let teams = vec![String::from("白队"),String::from("黑队")];
    let initial_scores = vec![10,50];
    let _scores:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();


    let field_name = String::from("Favorite color");
    let field_value = String::from("Red");
    let mut h = HashMap::new();
    h.insert(field_name, field_value);
    //println!("{}{}",field_name,field_value);

    let field_name2 = "color";
    let field_value2 = 255;
    let mut h2 = HashMap::new();
    h2.insert(field_name2, field_value2);
    println!("{}{}",field_name2,field_value2);

    //访问HashMap中的值
    let mut scores = HashMap::new();
    scores.insert(String::from("云里雾"), 10);
    scores.insert(String::from("雾里云"), 50);

    //let team_name = String::from("雾里云");
    let score = scores.get("雾里云");
    match score {
        Some(i) => println!("{}",i),
        _ => println!("None"),
    }

    for (key,value) in scores {
        println!("{}:{}",key,value);
    }

    //更新HashMap
    //覆盖一个值
    let mut scores = HashMap::new();
    scores.insert("快如火", 10);
    println!("{:?}",scores);
    scores.insert("快如火", 50);
    println!("{:?}",scores);

    //只在键没有对应的值时插入
    let mut scores = HashMap::new();
    scores.insert(String::from("兴烘掀"), 127);
    scores.entry(String::from("兴烘掀")).or_insert(255);
    scores.entry(String::from("掀烘兴")).or_insert(255);
    println!("{:?}",scores);

    //根据旧值更新一个值
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let poem = "吃 葡 萄 不 吐 葡 萄 皮";
    let mut poem_map = HashMap::new();
    for hanzi in poem.split_whitespace() {
        let count = poem_map.entry(hanzi).or_insert(0);
        *count += 1;
    }
    println!("{:?}",poem_map);

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