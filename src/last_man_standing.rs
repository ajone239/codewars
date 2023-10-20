pub fn last_man_standing(n: u32) -> u32 {
    let mut men: Vec<_> = (0..n).collect();

    while men.len() > 1 {
        men = men
            .into_iter()
            .enumerate()
            .filter(|(i, _)| i & 1 == 1)
            .map(|(_, v)| v)
            .rev()
            .collect();
    }
    men[0] + 1
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
