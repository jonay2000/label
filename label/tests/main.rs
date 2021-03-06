use label::create_label;
use std::any::Any;
use std::collections::HashSet;

// TODO: allow for creating multiple label in one create_annotation! macro.
// Create two label.
create_label!(
    // test that comments work here
    // V test that pub works (does nothing)
    pub(self) fn test() -> &'static str;
    fn test2(usize) -> usize;
);

pub mod child {
    // annotate a function by giving the path to the annotation and postfixing ::annotate.
    #[super::test::label]
    fn my_fn() -> &'static str {
        "Test2!"
    }
}

pub mod folder {
    // multiple label living in any submodule or supermodule are possible.
    #[crate::test::label]
    #[child::test1::label]
    fn fn_four() -> &'static str {
        "Test4!"
    }

    pub mod child {
        use label::create_label;

        #[super::super::test::label]
        fn my_fn() -> &'static str {
            "Test3!"
        }

        create_label!(fn test1() -> &'static str);
    }
}

#[test::label]
#[folder::child::test1::label]
fn another_fn() -> &'static str {
    "Test1!"
}

#[test2::label]
// label are typed, so functions annotated with test2 must take a usize and return one.
fn my_usize_fn(x: usize) -> usize {
    x + 1
}

#[test]
fn test_simple() {
    // using iter you can go through all functions with this annotation.
    let mut ret = HashSet::new();
    for i in test::iter() {
        ret.insert(i());
    }

    assert!(ret.contains("Test1!"));
    assert!(ret.contains("Test2!"));
    assert!(ret.contains("Test3!"));
    assert!(ret.contains("Test4!"));
}

#[test]
fn test_call_normal() {
    // Test to see if calling an annotated function normally still works
    assert_eq!(my_usize_fn(2), 3);
}

#[test]
fn test_label_in_module() {
    let mut ret = HashSet::new();

    for i in folder::child::test1::iter() {
        ret.insert(i());
    }

    assert!(ret.contains("Test1!"));
    assert!(ret.contains("Test4!"));
}

#[test]
fn test_add_one() {
    for i in test2::iter() {
        assert_eq!(i(3), 4);
    }
}

#[test]
fn test_simple_named() {
    // using iter you can go through all functions with this annotation.
    let mut ret = HashSet::new();
    for (name, i) in test::iter_named() {
        ret.insert((name, i()));
    }

    assert!(ret.contains(&("another_fn", "Test1!")));
    assert!(ret.contains(&("my_fn", "Test2!")));
    assert!(ret.contains(&("my_fn", "Test3!")));
    assert!(ret.contains(&("fn_four", "Test4!")));
}

pub struct Test<'a, 'b> {
    a: &'a usize,
    _b: &'b usize,
}

create_label!(
    fn with_lifetime<'a, 'b>(Test<'a, 'b>)-> &'a usize
);

#[with_lifetime::label]
fn fn_test_with_lifetimes<'a, 'b>(val: Test<'a, 'b>) -> &'a usize {
    val.a
}

#[test]
fn test_with_generics() {
    for i in with_lifetime::iter() {
        assert_eq!(i(Test { a: &10, _b: &15 }), &10);
    }
}

#[test]
fn test_traits() {
    fn implements<T: Send + Sync + Any>(_value: T) {}

    for i in with_lifetime::iter() {
        implements(i);
    }
    for i in test::iter() {
        implements(i);
    }
    for i in test2::iter() {
        implements(i);
    }
}
