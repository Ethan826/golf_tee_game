use std::fmt;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct GameMove {
    pub starting_space: usize,
    pub leapt_space: usize,
    pub destination_space: usize,
}

impl fmt::Display for GameMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Jump tee at position {} over position {} to position {}",
            self.starting_space, self.leapt_space, self.destination_space
        )
    }
}
