pub trait Solution {
    fn new(input: &str) -> Self;

    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}
