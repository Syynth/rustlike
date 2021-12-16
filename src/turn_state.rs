#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
}
