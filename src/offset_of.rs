#[macro_export]
macro_rules! offset_of {
    ($structType:ty, $field:ident) => {
        &(*(0 as *const $structType)).$field as *const _ as usize
    };
}
