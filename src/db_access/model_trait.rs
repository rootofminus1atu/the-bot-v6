pub trait Model {
    const NAME_PLURAL: &'static str;

    fn stringify(&self) -> String;
}