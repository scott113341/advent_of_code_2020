use std::iter::FromIterator;

#[derive(Eq, PartialEq, Debug)]
pub struct Mask(pub String);

impl Mask {
    pub fn apply(&self, value: usize) -> usize {
        let value: Vec<char> = format!("{:036b}", value).chars().collect();
        let mut result = String::new();

        for (idx, match_char) in self.0.chars().enumerate() {
            let c = match match_char {
                '0' => '0',
                '1' => '1',
                'X' => value[idx],
                _ => panic!(),
            };
            result.push(c.into());
        }

        usize::from_str_radix(&result, 2).unwrap() as usize
    }

    pub fn all_floating_addresses(&self, orig_address: usize) -> Vec<usize> {
        let mut addresses = vec![];

        // First, apply the mask to the originally given address
        let mut masked_addr: Vec<char> = format!("{:036b}", orig_address).chars().collect();
        let mut floating_indexes = vec![];
        for (idx, mask_char) in self.0.chars().enumerate() {
            match mask_char {
                '0' => (), // Unchanged
                '1' => masked_addr[idx] = '1',
                'X' => {
                    floating_indexes.push(idx);
                    masked_addr[idx] = 'X';
                }
                _ => panic!(),
            }
        }

        // Let's say there are 3 X (aka "floating") characters in the resulting masked address.
        // This means that there are actually 2^3=8 resulting addresses. To find them, count up to
        // 8-1=7 in binary, substituting "000", "001", "010", etc for the 3 X characters.
        let x_count = masked_addr.iter().filter(|&&c| c == 'X').count();
        for n in 0..=(2_usize.pow(x_count as u32) - 1) {
            let mut addr = masked_addr.clone();

            // For example: ['0', '0', '0']. Use these chars to substitute each X value.
            let mut float_subs = format!("{:0width$b}", n, width = x_count)
                .chars()
                .collect::<Vec<_>>();

            for &idx in floating_indexes.iter() {
                let sub_with = float_subs.remove(0);
                addr[idx] = sub_with;
            }

            addresses.push(usize::from_str_radix(&String::from_iter(addr), 2).unwrap() as usize);
        }

        addresses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        let mask = Mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string());
        assert_eq!(mask.apply(11), 73);
    }

    #[test]
    fn test_all_floating_addresses() {
        let mask = Mask("000000000000000000000000000000X1001X".to_string());
        assert_eq!(mask.all_floating_addresses(42), vec![26, 27, 58, 59]);

        let mask = Mask("00000000000000000000000000000000X0XX".to_string());
        assert_eq!(
            mask.all_floating_addresses(26),
            vec![16, 17, 18, 19, 24, 25, 26, 27]
        );
    }
}
