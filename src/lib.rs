pub static INPUT_STR: &str = include_str!("input.txt");
#[derive(Debug)]
pub struct {{class_name}} {
    pub input:&'static str,
}

impl {{class_name}} {
    pub fn new(input: &'static str) -> Option<Self> {
        Some({{class_name}}{input})
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
    use super::{{{class_name}} as AocDay, INPUT_STR};
    use _common::anyhow;
    use _common::anyhow::Context;
    
    static TEST_PART_01_STR: &str = include_str!("test_part_01.txt");
    static TEST_PART_02_STR: &str = include_str!("test_part_02.txt");

    #[test]
    fn test_part_01() -> anyhow::Result<()> {
        let mut aoc_day = AocDay::new(TEST_PART_01_STR).context("Couldn't create")?;
        Ok(assert_eq!(aoc_day.process_part_01(), 999))
    }

    #[test]
    fn test_part_01_final() -> anyhow::Result<()> {
        let mut aoc_day = AocDay::new(INPUT_STR).context("Couldn't create")?;
        let result = aoc_day.process_part_01();
        println!("Final Result Part01: {result}");
        Ok(assert_eq!(result, 999))
    }

    #[test]
    fn test_part_02() -> anyhow::Result<()> {
        let mut aoc_day = AocDay::new(TEST_PART_02_STR).context("Couldn't create")?;
        Ok(assert_eq!(aoc_day.process_part_02(), 999))
    }

    #[test]
    fn test_part_02_final() -> anyhow::Result<()> {
        let mut aoc_day = AocDay::new(INPUT_STR).context("Couldn't create")?;
        let result = aoc_day.process_part_02();
        println!("Final Result Part02: {result}");
        Ok(assert_eq!(result, 999))
    }
}