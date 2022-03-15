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

pub enum Action{
    Increment(Jump),
    Decrement(Jump),
}
