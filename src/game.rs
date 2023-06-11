struct Game {
    state: GameState,
    map: String,
}

enum GameState {
    Ongoing,
    Paused,
    Finished,
}
