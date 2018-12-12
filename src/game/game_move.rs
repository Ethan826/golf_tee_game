#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct GameMove {
    pub starting_space: usize,
    pub leapt_space: usize,
    pub destination_space: usize,
}
