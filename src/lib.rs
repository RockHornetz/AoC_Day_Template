pub static INPUT_STR: &str = include_str!("input.txt");
#[derive(Debug)]
pub struct {{class_name}} {}

impl {{class_name}} {
    pub fn new(input: &str) -> Option<Self> {
        Some({{class_name}}{})
    }

    pub fn process_part_01(&self)->i32{
        0
    }

    pub fn process_part_02(&self)->i32{
        0
    }
}

impl Iterator for {{class_name}} {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_PART1_STR: &str = include_str!("test_part1.txt");
    static TEST_PART2_STR: &str = include_str!("test_part2.txt");

    #[test]
    fn test_part01() {
        if let Some(mut aoc_day) = {{class_name}}::new(TEST_PART1_STR) {
            assert_eq!(aoc_day.process_part_01(), 999)
        }
    }
    #[test]
    fn test_part02() {
        if let Some(mut aoc_day) = {{class_name}}::new(TEST_PART2_STR) {
            assert_eq!(aoc_day.process_part_02(), 999)
        }
    }
    #[test]
    fn test_part01_final() {
        if let Some(mut aoc_day) = {{class_name}}::new(INPUT_STR) {
            let result = aoc_day.process_part_01();
            println!("Final Result Part01: {result}");
            assert_eq!(result, 999)
        }
    }

    #[test]
    fn test_part02_final() {
        if let Some(mut aoc_day) = {{class_name}}::new(INPUT_STR) {
            let result = aoc_day.process_part_02();
            println!("Final Result Part02: {result}");
            assert_eq!(result, 999)
        }
    }

}
