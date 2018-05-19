pub struct Regulation {
    pub content: &'static str,
    pub episode: u16,
    pub number: u16,
    pub violator: &'static str,
}

impl Regulation {
        pub(crate) fn echo(&self) -> String {
            return format!("パプリカ学園校則第{}条! {}", self.number.to_string(), self.content);
        }
}
