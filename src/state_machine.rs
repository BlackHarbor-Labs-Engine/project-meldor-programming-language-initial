#[derive(Debug, PartialEq)] pub enum State {
    Idle,
    Processing,
    Error,
}

pub struct StateMachine {
    pub current_state: State,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine { current_state: State::Idle }
    }

    pub fn transition(&mut self, next_state: State) -> Result<(), String> {
        match self.current_state {
            State::Idle => {
                if next_state == State::Processing {
                    self.current_state = State::Processing;
                    Ok(())
                } else {
                    Err(format!("Cannot transition from Idle to {:?}", next_state))
                }
            },
            State::Processing => {
                if next_state == State::Idle || next_state == State::Error {
                    self.current_state = next_state;
                    Ok(())
                } else {
                    Err(format!("Cannot transition from Processing to {:?}", next_state))
                }
            },
            State::Error => {
                if next_state == State::Idle {
                    self.current_state = State::Idle;
                    Ok(())
                } else {
                    Err(format!("Cannot transition from Error to {:?}", next_state))
                }
            }
        }
    }
}
