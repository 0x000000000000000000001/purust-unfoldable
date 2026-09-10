pub fn Data_Unfoldable1_unfoldr1ArrayImpl(
    is_nothing: purust_core::Func1<std::rc::Rc<Purs_Data_Maybe::Maybe>, bool>,
    from_just: purust_core::Func1<std::rc::Rc<Purs_Data_Maybe::Maybe>, crate::UnknownType>,
    first: purust_core::Func1<std::rc::Rc<Purs_Data_Tuple::Tuple>, crate::UnknownType>,
    second: purust_core::Func1<std::rc::Rc<Purs_Data_Tuple::Tuple>, crate::UnknownType>,
    step: purust_core::Func1<crate::UnknownType, std::rc::Rc<Purs_Data_Tuple::Tuple>>,
    initial: crate::UnknownType,
) -> crate::UnknownType {
    let mut result = Vec::new();
    let mut value = initial;
    loop {
        let tuple = step(value);
        result.push(first(tuple.clone()));
        let maybe = second(tuple)
            .unwrap_class::<std::rc::Rc<Purs_Data_Maybe::Maybe>>()
            .clone();
        if is_nothing(maybe.clone()) {
            break;
        }
        value = from_just(maybe);
    }
    crate::Value::Array(std::rc::Rc::new(result))
}
