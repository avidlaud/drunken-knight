#[derive(Debug)]
pub enum Jump {
    N2E1,
    N1E2,
    S1E2,
    S2E1,
    S2W1,
    S1W2,
    N1W2,
    N2W1,
    Still,
}

#[derive(Debug)]
pub enum Action{
    Increment(Jump),
    Decrement(Jump),
}

pub fn get_action(b: u8) -> Action {
    match b {
        0 => Action::Increment(Jump::N2E1),
        1 => Action::Increment(Jump::N1E2),
        2 => Action::Increment(Jump::S1E2),
        3 => Action::Increment(Jump::S2E1),
        4 => Action::Increment(Jump::S2W1),
        5 => Action::Increment(Jump::S1W2),
        6 => Action::Increment(Jump::N1W2),
        7 => Action::Increment(Jump::N2W1),
        8 => Action::Decrement(Jump::N2E1),
        9 => Action::Decrement(Jump::N1E2),
        10 => Action::Decrement(Jump::S1E2),
        11 => Action::Decrement(Jump::S2E1),
        12 => Action::Decrement(Jump::S2W1),
        13 => Action::Decrement(Jump::S1W2),
        14 => Action::Decrement(Jump::N1W2),
        15 => Action::Decrement(Jump::N2W1),
        _ => panic!("Expected half-byte to be between 0 and 15, got {}.", b),
    }
}