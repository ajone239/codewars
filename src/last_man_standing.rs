pub fn last_man_standing(n: u32) -> u32 {
    let mut men = vec![0u8; n as usize];

    let mut direction = false;
    loop {
        if men.iter().filter(|m| **m == 0).count() == 1 {
            break;
        }

        let miter = men.iter_mut().filter(|m| **m == 0);

        let miter: Box<dyn Iterator<Item = &mut u8>> = if direction {
            Box::new(miter.rev())
        } else {
            Box::new(miter)
        };

        for (i, m) in miter.enumerate() {
            if i & 1 == 0 {
                *m = 1;
            }
        }

        direction = !direction;
    }

    (men.into_iter()
        .enumerate()
        .find(|(_, x)| *x == 0)
        .unwrap()
        .0
        + 1) as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_last_man_standing() {
        assert_eq!(last_man_standing(1), 1);
        assert_eq!(last_man_standing(2), 2);
        assert_eq!(last_man_standing(9), 6);
        assert_eq!(last_man_standing(10), 8);
        assert_eq!(last_man_standing(100), 54);
        assert_eq!(last_man_standing(1000), 510);
    }
}
