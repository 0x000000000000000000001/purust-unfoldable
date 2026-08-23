

pub fn Data_Unfoldable_unfoldrArrayImpl(
    mut isNothing: crate::UnknownType,
    mut fromJust: crate::UnknownType,
    mut fst: crate::UnknownType,
    mut snd: crate::UnknownType,
    mut f: crate::UnknownType,
    mut b: crate::UnknownType,
) -> crate::UnknownType {
    let mut result = Vec::new();
    let mut value = b;
    loop {
        let maybe = f.unwrap_func1()(value);
        let is_nothing_val = isNothing.unwrap_func1()(maybe.clone());
        if is_nothing_val.unwrap_bool() {
            break;
        }
        let tuple = fromJust.unwrap_func1()(maybe);
        let first = fst.unwrap_func1()(tuple.clone());
        result.push(first);
        value = snd.unwrap_func1()(tuple);
    }
    crate::Value::Array(std::rc::Rc::new(result))
}
