pub fn run1() -> String {
    (134792..675811).filter(|&it| test(it)).count().to_string()
}

pub fn run2() -> String {
    (134792..675811).filter(|&it| test2(it)).count().to_string()
}


fn test(x: i32) -> bool {
    let chars: Vec<char> = x.to_string().chars().collect();
    let adj = chars[..].windows(2).any(|w| w[0] == w[1]);
    let inc = chars[..].windows(2).all(|w| w[0] <= w[1]);
    adj && inc
}

fn test2(x: i32) -> bool {
    let padded: Vec<char> = format!(" {} ", x).chars().collect();
    let only2 = (&padded[..]).windows(4).any(|w| {
        w[0] != w[1] && w[1] == w[2] && w[2] != w[3]
    });
    test(x) && only2
}

#[cfg(test)]
mod test {
    use crate::day4::test2;

    #[test]
    fn pt2() {
        assert!(test2(112233));
        assert!(test2(111122));
        assert!(!test2(123444));
    }
}
