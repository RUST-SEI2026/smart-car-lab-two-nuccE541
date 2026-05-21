use crate::Pose;

#[derive(Copy, Clone)]
pub(crate) enum Action {
    Forward(i32),
    TurnLeft,
    TurnRight,
}

impl Action {
    pub(crate) fn perform(&self, pose: &mut Pose) {
        match self {
            Action::Forward(offset) => pose.forward(*offset),
            Action::TurnLeft => pose.turn_left(),
            Action::TurnRight => pose.turn_right(),
        }
    }
}