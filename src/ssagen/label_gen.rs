pub struct LabelGenerator {
    for_id: u32,
    ifelse_id: u32,
}

impl LabelGenerator {
    pub fn new() -> Self {
        Self {
            for_id: 0_u32,
            ifelse_id: 0_u32,
        }
    }

    pub fn get_for_labels(&mut self) -> (String, String) {
        let res = (format!("__LForB{}", self.for_id), format!("__LForE{}", self.for_id)) ;
        self.for_id += 1;
        res
    }
}
