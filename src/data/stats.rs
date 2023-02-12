#[derive(Debug)]
pub(crate) struct CatCount {
    pub(crate) counts: Vec<u64>,
    pub(crate) total: u64,
}

impl CatCount {
    fn new() -> CatCount {
        CatCount{
            counts: vec![0; 4],
            total: 0,
        }
    }
    fn increment(&mut self, cat: TitleType) {
        self.counts[cat as usize]+= 1;
        self.total += 1;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum TitleType {
    MixedCase = 0,
    UppercaseOnly,
    LowercaseOnly,
    SomethingElse,
}
