#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pose {
    pub x: i32,
    pub y: i32,
    pub heading: char,
}

impl Pose {
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Pose { x, y, heading }
    }
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            x: 0,
            y: 0,
            heading: 'N',
        }
    }
}

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
                        match self.pose.heading {
                        'E' => self.pose.x -= 1,
                        'S' => self.pose.y += 1,
                        'W' => self.pose.x += 1,
                        'N' => self.pose.y -= 1,
                        _ => (),
                        }
                    }else{
                        match self.pose.heading {
                        'E' => self.pose.x += 1,
                        'S' => self.pose.y -= 1,
                        'W' => self.pose.x -= 1,
                        'N' => self.pose.y += 1,
                        _ => (),
                        }
                    }
                },

                'L' => {
                    if self.is_reverse{
                        match self.pose.heading {
                            'E' => self.pose.heading = 'S',
                            'S' => self.pose.heading = 'W',
                            'W' => self.pose.heading = 'N',
                            'N' => self.pose.heading = 'E',
                            _ => (),
                        }
                    }else{
                        match self.pose.heading {
                            'E' => self.pose.heading = 'N',
                            'S' => self.pose.heading = 'E',
                            'W' => self.pose.heading = 'S',
                            'N' => self.pose.heading = 'W',
                            _ => (),
                            }
                        }
                    },

                'R' => {
                    if self.is_reverse{
                    match self.pose.heading {
                        'E' => self.pose.heading = 'N',
                        'S' => self.pose.heading = 'E',
                        'W' => self.pose.heading = 'S',
                        'N' => self.pose.heading = 'W',
                        _ => (),
                    }
                    }else{
                        match self.pose.heading {
                        'E' => self.pose.heading = 'S',
                        'S' => self.pose.heading = 'W',
                        'W' => self.pose.heading = 'N',
                        'N' => self.pose.heading = 'E',
                        _ => (),
                         }
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
