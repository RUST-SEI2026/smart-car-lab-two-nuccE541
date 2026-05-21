use super::action::Action;

#[derive(Default, Debug, Clone, Copy)]
pub(crate)struct State {
    is_reverse: bool,
}

impl State {
    pub(crate) fn toggle_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }

    pub(crate) fn is_reverse(&self) -> bool {
        self.is_reverse
    }

    
    pub(crate) fn assemble(&self, cmd: char) -> Vec<Action> {
        match cmd {
            'M' => self.move_assemble(),
            'L' => self.turn_left_assemble(),
            'R' => self.turn_right_assemble(),
            _ => Vec::new(),
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

