pub trait GameState: Sized {
    fn new() -> Self;
    fn evaluate(&self, first_player: bool) -> i32;
    fn get_children(&self, first_player: bool) -> Vec<Self>;
    fn to_string(&self) -> String;
}
