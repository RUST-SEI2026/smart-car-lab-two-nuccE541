use crate::Pose;

#[derive(Default, Debug, Clone, Copy)]
pub struct State {
    is_reverse: bool,
}

impl State {
    pub fn toggle_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }

    pub fn is_reverse(&self) -> bool {
        self.is_reverse
    }
}

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
                
                'M' => {
                    if self.state.is_reverse(){
                        self.pose.backward();
                    }else{
                        self.pose.forward();
                    }
                },

                'L' => {
                    if self.state.is_reverse(){
                        self.pose.turn_right();
                    }else{
                        self.pose.turn_left();
                    }
                }

                'R' => {
                    if self.state.is_reverse(){
                        self.pose.turn_left();
                    }else{
                        self.pose.turn_right();
                    }
                },
                _ => (),
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
