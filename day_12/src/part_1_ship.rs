#[derive(Debug)]
pub struct Ship {
    pub x: isize,
    pub y: isize,
    pub facing: isize,
}

impl Ship {
    pub fn do_action(&mut self, action_str: &String) {
        let mut chars = action_str.chars();
        let action = chars.nth(0).unwrap();
        let value: isize = chars.as_str().parse().unwrap();

        match action {
            // Move in a this direction for some distance
            'N' => self.y += value,
            'S' => self.y -= value,
            'E' => self.x += value,
            'W' => self.x -= value,

            // Turn in this direction some number of degrees
            'L' => self.facing -= value,
            'R' => self.facing += value,

            // Go in the direction we're currently facing for some distance
            'F' => match self.facing {
                0 => self.y += value,
                180 => self.y -= value,
                90 => self.x += value,
                270 => self.x -= value,
                _ => panic!("Unsupported direction: {}", self.facing),
            },

            // Whoops
            _ => panic!("Unknown action: {}", action),
        }

        // Normalize the direction we're facing
        self.facing = (self.facing + 360) % 360;
    }
}
