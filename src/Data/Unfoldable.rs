

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
        let maybe = f.call.as_ref().unwrap()(value);
        let is_nothing_val = isNothing.call.as_ref().unwrap()(maybe.clone());
        if is_nothing_val.init_bool.unwrap() {
            break;
        }
        let tuple = fromJust.call.as_ref().unwrap()(maybe);
        let first = fst.call.as_ref().unwrap()(tuple.clone());
        result.push(first);
        value = snd.call.as_ref().unwrap()(tuple);
    }
    crate::UnknownType::new(crate::Record_a {
        init_array: Some(std::rc::Rc::new(result)),
        ..Default::default()
    })
}
