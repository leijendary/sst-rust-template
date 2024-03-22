pub trait Translation {
    fn language(&self) -> String;
    fn ordinal(&self) -> i16;
}
