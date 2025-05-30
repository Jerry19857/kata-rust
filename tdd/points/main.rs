fn points(games: &[String]) -> u32 {
    let mut total_result:u32 = 0;
    for game in games  {
        let g:Vec<&str> = game.split(":").collect();
        if g[0] > g[1] {
            total_result = total_result + 3
        }
        if g[0] == g[1] {
            total_result = total_result + 1
        }
        if g[0] < g[1] {
            total_result = total_result + 0
        }
    }
    total_result
}

fn main() {
    let games = vec![
        "1:0".to_string(),
        "2:2".to_string(),
        "3:1".to_string(),
        "0:1".to_string(),
    ];
    let result = points(&games);
    println!("Total points: {}", result);
}