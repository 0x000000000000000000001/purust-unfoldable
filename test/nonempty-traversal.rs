fn run(action: &Value) -> Value {
    action.unwrap_func1()(Value::Unit)
}
fn effect_map(function: Value, action: Value) -> Value {
    let function = function.unwrap_func1();
    Value::Func1(Func1::Shared(std::rc::Rc::new(move |_| {
        function(run(&action))
    })))
}
fn effect_apply(functions: Value, action: Value) -> Value {
    Value::Func1(Func1::Shared(std::rc::Rc::new(move |_| {
        let function = run(&functions).unwrap_func1();
        function(run(&action))
    })))
}
fn check_effects(length: i64) {
    let constructed = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let executed = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let constructed2 = constructed.clone();
    let executed2 = executed.clone();
    let function = Value::Func1(Func1::Shared(std::rc::Rc::new(move |value: Value| {
        let n = value.unwrap_int();
        constructed2.lock().unwrap().push(n);
        let executed = executed2.clone();
        Value::Func1(Func1::Shared(std::rc::Rc::new(move |_| {
            executed.lock().unwrap().push(n);
            Value::Int(n * 2)
        })))
    })));
    // Exercise the generated Fn3 calling convention through its curried adapter.
    let traversal = Data_Array_NonEmpty_Internal_traverse1Impl().unwrap_func1()(Value::Func2(
        Func2::Static(effect_apply),
    ))
    .unwrap_func1()(Value::Func2(Func2::Static(effect_map)))
    .unwrap_func1()(function);
    let input = mk_array((0..length).map(Value::Int).collect());
    let action = traversal.unwrap_func1()(input.clone());
    assert!(
        executed.lock().unwrap().is_empty(),
        "traversal construction must defer effects"
    );
    assert_eq!(
        *constructed.lock().unwrap(),
        (0..length).rev().collect::<Vec<_>>()
    );
    for _ in 0..2 {
        let output = run(&action);
        let actual: Vec<_> = output
            .unwrap_array()
            .iter()
            .map(Value::unwrap_int)
            .collect();
        assert_eq!(actual, (0..length).map(|n| n * 2).collect::<Vec<_>>());
    }
    assert_eq!(
        *executed.lock().unwrap(),
        (0..length).chain(0..length).collect::<Vec<_>>()
    );
    assert_eq!(input.unwrap_array().len(), length as usize);
    assert_eq!(
        constructed.lock().unwrap().len(),
        length as usize,
        "replay must reuse the constructed actions"
    );
}
fn main() {
    check_effects(1);
    check_effects(5);
    let array_map = Func2::Static(|function: Value, array: Value| {
        Data_Functor_arrayMap(function.unwrap_func1(), array)
    });
    let choose = Func1::Static(|value: Value| {
        let n = value.unwrap_int();
        mk_array(vec![Value::Int(n), Value::Int(n * 10)])
    });
    let traversal = Data_Array_NonEmpty_Internal_traverse1Impl().unwrap_func3()(
        Value::Func2(Func2::Static(Control_Apply_arrayApply)),
        Value::Func2(array_map),
        Value::Func1(choose),
    );
    let input = mk_array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
    let expected = vec![
        vec![1, 2, 3],
        vec![1, 2, 30],
        vec![1, 20, 3],
        vec![1, 20, 30],
        vec![10, 2, 3],
        vec![10, 2, 30],
        vec![10, 20, 3],
        vec![10, 20, 30],
    ];
    for _ in 0..2 {
        let result = traversal.unwrap_func1()(input.clone());
        let actual: Vec<Vec<_>> = result
            .unwrap_array()
            .iter()
            .map(|row| row.unwrap_array().iter().map(Value::unwrap_int).collect())
            .collect();
        assert_eq!(actual, expected);
    }
    // All values are constructed and dropped inside this thread in both modes.
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| check_effects(20000))
        .unwrap()
        .join()
        .unwrap();
}
