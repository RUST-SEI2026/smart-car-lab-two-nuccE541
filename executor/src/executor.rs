use crate::Pose;
use crate::state::State;

pub struct Executor {
    pose: Pose,
    state: State,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor { pose,
        state: State::default(),}
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            match cmd {
                'B'=> self.state.toggle_reverse(),
                'F'=> self.state.toggle_fast(),
                _ => {
                    let actions = self.state.assemble(cmd);
                    for action in actions{
                        action.perform(&mut self.pose);
                    }
                },
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
