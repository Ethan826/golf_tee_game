use crate::is_triangular;
use crate::position_data::PositionData;

#[derive(Debug, PartialEq, Eq)]
pub struct LegalMoves(Vec<PositionData>);

impl LegalMoves {
    /// Given a `Vec` of `PositionData`, return an instance of `LegalMoves`.
    pub fn new(input: Vec<PositionData>) -> Result<Self, Box<dyn std::error::Error>> {
        if is_triangular(input.len()) {
            Ok(LegalMoves(input))
        } else {
            Err("The length of the `LegalMoves` input must be a triangular number")?
        }
    }

    pub fn legal_moves_at_position<'a>(
        &'a self,
        position: usize,
    ) -> Result<&'a LegalMoves, Box<dyn std::error::Error>> {
        Ok(self.0.get(position)?)
    }
}

// =================================================================================================
// Tests
// =================================================================================================
