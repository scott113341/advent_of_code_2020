use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Clone, Debug)]
#[display("{name}: {a}-{b} or {c}-{d}")]
pub struct TicketField {
    name: String,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
}

impl TicketField {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn includes(&self, number: usize) -> bool {
        (self.a..=self.b).contains(&number) || (self.c..=self.d).contains(&number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticket_field_includes() {
        let tf: TicketField = "rico meow: 10-25 or 36-129".parse().unwrap();
        dbg!(&tf);
        assert_eq!(tf.includes(10), true);
        assert_eq!(tf.includes(100), true);
        assert_eq!(tf.includes(30), false);
    }
}
