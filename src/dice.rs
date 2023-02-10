use rand::Rng;
#[derive(Debug)]
pub struct dice {
    pub value: usize,
    pub throw_count:usize
}

impl dice {
    pub fn create() -> Self 
    {
        Self { value: 0, throw_count: 0 }
    }

    pub fn throw_dice(&mut self) -> usize
    {
        self.value = rand::thread_rng().gen_range(1..=6);
        self.throw_count += 1;

        self.value
    }

}