#[derive(Debug)]
pub struct Ship {
    pub x: isize,
    pub y: isize,
    pub wpt_dx: isize,
    pub wpt_dy: isize,
}

impl Ship {
    pub fn do_action(&mut self, action_str: &String) {
        let mut chars = action_str.chars();
        let action = chars.nth(0).unwrap();
        let value: isize = chars.as_str().parse().unwrap();

        match action {
            // Move the waypoint this direction for some distance
            'N' => self.wpt_dy += value,
            'S' => self.wpt_dy -= value,
            'E' => self.wpt_dx += value,
            'W' => self.wpt_dx -= value,

            // Rotate the waypoint around the ship this direction for some degrees
            'L' | 'R' => match (action, value) {
                ('L', 90) | ('R', 270) => {
                    let new_wpt = r90(r90(r90((self.wpt_dx, self.wpt_dy))));
                    self.wpt_dx = new_wpt.0;
                    self.wpt_dy = new_wpt.1;
                }
                ('L', 270) | ('R', 90) => {
                    let new_wpt = r90((self.wpt_dx, self.wpt_dy));
                    self.wpt_dx = new_wpt.0;
                    self.wpt_dy = new_wpt.1;
                }
                (_, 180) => {
                    self.wpt_dx *= -1;
                    self.wpt_dy *= -1;
                }
                _ => panic!("Unsupported turn command: {}", action_str),
            },

            // Go to the waypoint the given number of times (waypoint is always relative to ship)
            'F' => {
                self.x += self.wpt_dx * value;
                self.y += self.wpt_dy * value;
            }

            // Whoops
            _ => panic!("Unknown action: {}", action),
        }
    }
}

// Returns a new tuple that's rotated 90 degrees clockwise (R). Each tuple is (wpt_dx, wpt_dy).
fn r90(from: (isize, isize)) -> (isize, isize) {
    (from.1, -from.0)
}
