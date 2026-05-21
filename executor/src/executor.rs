use crate::Pose;

pub struct Executor {
    pose: Pose,
    is_reverse: bool,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor { pose,
        is_reverse: false, }
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            match cmd {
                'B'=> self.is_reverse = !self.is_reverse,
                
                'M' => {
                    if self.is_reverse{
                        self.pose.backward();
                    }else{
                        self.pose.forward();
                    }
                },

                'L' => {
                    if self.is_reverse{
                        self.pose.turn_right();
                    }else{
                        self.pose.turn_left();
                    }
                }

                'R' => {
                    if self.is_reverse{
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
