use crate::action;

use super::action::Action;

#[derive(Default, Debug, Clone, Copy)]
pub(crate)struct State {
    is_reverse: bool,
    is_fast: bool,
}

impl State {
    pub(crate) fn toggle_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }

    pub(crate) fn toggle_fast(&mut self) {
        self.is_fast = !self.is_fast;
    }

    pub(crate) fn is_fast(&self) -> bool {
        self.is_fast
    }

    pub(crate) fn is_reverse(&self) -> bool {
        self.is_reverse
    }

    
    pub(crate) fn assemble(&self, cmd: char) -> Vec<Action> {
        let mut actions = Vec::new();
        match cmd {
            'M' => {
                if self.is_fast{
                    actions.extend(self.move_assemble());
                }
                actions.extend(self.move_assemble());
                actions
            },
            'L' => {
                if self.is_fast{
                   actions.extend(self.move_assemble());
                }
                actions.extend(self.turn_left_assemble());
                actions
            },
            'R' => {
                if self.is_fast{
                    actions.extend(self.move_assemble());
                }
                actions.extend(self.turn_right_assemble());
                actions
            },
            _ => actions,
        }
    }

    fn move_assemble(&self) -> Vec<Action> {
        let direction = if self.is_reverse { -1 } else { 1 };
        vec![Action::Forward(direction)]
    }

    fn turn_left_assemble(&self) -> Vec<Action> {
        let act = if self.is_reverse {
            Action::TurnRight
        } else {
            Action::TurnLeft
        };
        vec![act]
    }

    fn turn_right_assemble(&self) -> Vec<Action> {
        let act = if self.is_reverse {
            Action::TurnLeft
        } else {
            Action::TurnRight
        };
        vec![act]
    }
}

