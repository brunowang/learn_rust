fn main() {
    let player_scores = [("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19)];
    let players: Vec<_> = player_scores
        .iter()
        .map(|&(player, _score)| player)
        .collect();
    println!("{:?}", players)
}
