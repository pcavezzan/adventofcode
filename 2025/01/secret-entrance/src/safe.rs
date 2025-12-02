
struct Safe {
    n: Vec<i8>,
    arrow: i8
}

impl Safe {
    pub fn new() -> Self {
        let mut default = vec![];
        for i in 0..100 {
            default.push(i);
        }
        Self { n: default, arrow: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_safe_with_default_values() {
        let d = Safe::new();

        // check dial values
        assert_eq!(0, d.n[0] );
        assert_eq!(99, d.n[99]);
        // check arrow value
        assert_eq!(0, d.arrow);
    }
}