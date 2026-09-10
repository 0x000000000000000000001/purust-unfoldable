mod Purs_Data_Maybe {
    pub enum Maybe {
        Nothing,
        Just(crate::Value),
    }
}
mod Purs_Data_Tuple {
    pub enum Tuple {
        Tuple(crate::Value, crate::Value),
    }
}
fn tuple(a: Value, b: Value) -> Value {
    Value::Class(std::rc::Rc::new(std::rc::Rc::new(
        Purs_Data_Tuple::Tuple::Tuple(a, b),
    )))
}
fn main() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let calls = std::sync::Arc::new(AtomicUsize::new(0));
    let extracted = std::sync::Arc::new(AtomicUsize::new(0));
    for nonempty in [false, true] {
        for length in [0, 3] {
            calls.store(0, Ordering::SeqCst);
            extracted.store(0, Ordering::SeqCst);
            let calls2 = calls.clone();
            let state_step = Func1::Shared(std::rc::Rc::new(move |state: Value| {
                calls2.fetch_add(1, Ordering::SeqCst);
                let state = state.unwrap_class::<std::rc::Rc<Purs_Data_Tuple::Tuple>>();
                let Purs_Data_Tuple::Tuple::Tuple(n, total) = state.as_ref();
                (n.unwrap_int(), total.unwrap_int())
            }));
            let is_nothing = Func1::Static(|value: std::rc::Rc<Purs_Data_Maybe::Maybe>| {
                matches!(value.as_ref(), Purs_Data_Maybe::Maybe::Nothing)
            });
            let extracted2 = extracted.clone();
            let from_just = Func1::Shared(std::rc::Rc::new(
                move |value: std::rc::Rc<Purs_Data_Maybe::Maybe>| {
                    extracted2.fetch_add(1, Ordering::SeqCst);
                    match value.as_ref() {
                        Purs_Data_Maybe::Maybe::Just(value) => value.clone(),
                        _ => panic!("fromJust must not inspect Nothing"),
                    }
                },
            ));
            let first = Func1::Static(|value: std::rc::Rc<Purs_Data_Tuple::Tuple>| {
                let Purs_Data_Tuple::Tuple::Tuple(a, _) = value.as_ref();
                a.clone()
            });
            let second = Func1::Static(|value: std::rc::Rc<Purs_Data_Tuple::Tuple>| {
                let Purs_Data_Tuple::Tuple::Tuple(_, b) = value.as_ref();
                b.clone()
            });
            let initial = tuple(Value::Int(length), Value::Int(10));
            let result = if nonempty {
                let step = Func1::Shared(std::rc::Rc::new(move |state: Value| {
                    let (n, total) = state_step(state);
                    let rest = if n == 0 {
                        Purs_Data_Maybe::Maybe::Nothing
                    } else {
                        Purs_Data_Maybe::Maybe::Just(tuple(
                            Value::Int(n - 1),
                            Value::Int(total + n),
                        ))
                    };
                    std::rc::Rc::new(Purs_Data_Tuple::Tuple::Tuple(
                        Value::Int(total),
                        Value::Class(std::rc::Rc::new(std::rc::Rc::new(rest))),
                    ))
                }));
                Data_Unfoldable1_unfoldr1ArrayImpl(
                    is_nothing, from_just, first, second, step, initial,
                )
            } else {
                let step = Func1::Shared(std::rc::Rc::new(move |state: Value| {
                    let (n, total) = state_step(state);
                    std::rc::Rc::new(if n == 0 {
                        Purs_Data_Maybe::Maybe::Nothing
                    } else {
                        Purs_Data_Maybe::Maybe::Just(tuple(
                            Value::Int(total),
                            tuple(Value::Int(n - 1), Value::Int(total + n)),
                        ))
                    })
                }));
                Data_Unfoldable_unfoldrArrayImpl(
                    is_nothing, from_just, first, second, step, initial,
                )
            };
            let actual: Vec<_> = result
                .unwrap_array()
                .iter()
                .map(Value::unwrap_int)
                .collect();
            let expected = match (nonempty, length) {
                (false, 0) => vec![],
                (true, 0) => vec![10],
                (false, _) => vec![10, 13, 15],
                (true, _) => vec![10, 13, 15, 16],
            };
            assert_eq!(actual, expected);
            assert_eq!(calls.load(Ordering::SeqCst), length as usize + 1);
            assert_eq!(extracted.load(Ordering::SeqCst), length as usize);
        }
    }
}
