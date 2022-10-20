fn main() {
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use list_comprehension_macro::{comp, i_comp};

    #[test]
    fn simple_comp() {
        let arr: Vec<u32> = vec![1, 2, 3, 4];
        let result = comp![x * 2 for x in arr];
        assert_eq!(result, vec![2, 4, 6, 8]);
    }

    #[test]
    fn simple_comp_with_condition() {
        let arr: Vec<u32> = vec![1, 2, 3, 4];
        let result = comp![x * 2 for x in arr if x % 2 == 0];
        assert_eq!(result, vec![4, 8]);
    }

    #[test]
    fn flatten_matrix() {
        let arr: Vec<Vec<u32>> = vec![vec![1, 2], vec![3, 4]];
        let result = comp![x for row in arr for x in row];
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn map() {
        let arr: Vec<u32> = vec![1, 2, 3];
        let result = comp!{x: x.to_string() for x in arr};
        assert_eq!(result, HashMap::from([
            (1, String::from("1")),
            (2, String::from("2")),
            (3, String::from("3"))
        ]));
    }

    #[test]
    fn while_loop() {
        let mut i = 1;
        assert_eq!(
            comp![{i *= 2; i} while i < 10],
            vec![2, 4, 8, 16]
        )
    }

    #[test]
    fn iter_comp() {
        let arr: Vec<u32> = vec![1, 2, 3, 4];
        let result: Vec<u32> = i_comp![x * 2 for x in arr.iter()].collect();
        assert_eq!(result, vec![2, 4, 6, 8]);
    }

    #[test]
    fn iter_comp_with_condition() {
        let arr: Vec<u32> = vec![1, 2, 3, 4];
        let result: Vec<u32> = i_comp![x * 2 for x in arr.iter() if x % 2 == 0].collect();
        assert_eq!(result, vec![4, 8]);
    }

    #[test]
    fn iter_map() {
        let arr: Vec<u32> = vec![1, 2, 3];
        let result: HashMap<u32, String> = i_comp!{*x: x.to_string() for x in arr.iter()}.collect();
        assert_eq!(result, HashMap::from([
            (1, String::from("1")),
            (2, String::from("2")),
            (3, String::from("3"))
        ]));
    }
}