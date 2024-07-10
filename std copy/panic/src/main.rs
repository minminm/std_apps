#![allow(dead_code)]

use std::cell::RefCell;
use std::panic::{AssertUnwindSafe, UnwindSafe};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

struct Foo {
    a: i32,
}

fn assert_unwind_safe<T: UnwindSafe + ?Sized>() {
    println!("Type {:?} implements UnwindSafe", std::any::type_name::<T>());
}

fn check_panic_safety_traits() {
    assert_unwind_safe::<i32>();
    assert_unwind_safe::<&i32>();
    assert_unwind_safe::<*mut i32>();
    assert_unwind_safe::<*const i32>();
    assert_unwind_safe::<usize>();
    assert_unwind_safe::<str>();
    assert_unwind_safe::<&str>();
    assert_unwind_safe::<Foo>();
    assert_unwind_safe::<&Foo>();
    assert_unwind_safe::<Vec<i32>>();
    assert_unwind_safe::<String>();
    assert_unwind_safe::<RefCell<i32>>();
    assert_unwind_safe::<Box<i32>>();
    assert_unwind_safe::<Mutex<i32>>();
    assert_unwind_safe::<RwLock<i32>>();
    assert_unwind_safe::<&Mutex<i32>>();
    assert_unwind_safe::<&RwLock<i32>>();
    assert_unwind_safe::<Rc<i32>>();
    assert_unwind_safe::<Arc<i32>>();
    assert_unwind_safe::<Box<[u8]>>();

    {
        trait Trait: UnwindSafe {}
        assert_unwind_safe::<Box<dyn Trait>>();
    }

    fn bar<T>() {
        assert_unwind_safe::<Mutex<T>>();
        assert_unwind_safe::<RwLock<T>>();
    }

    fn baz<T: UnwindSafe>() {
        assert_unwind_safe::<Box<T>>();
        assert_unwind_safe::<Vec<T>>();
        assert_unwind_safe::<RefCell<T>>();
        assert_unwind_safe::<AssertUnwindSafe<T>>();
        assert_unwind_safe::<&AssertUnwindSafe<T>>();
        assert_unwind_safe::<Rc<AssertUnwindSafe<T>>>();
        assert_unwind_safe::<Arc<AssertUnwindSafe<T>>>();
    }

    bar::<i32>();
    baz::<i32>();
}

fn main() {
    check_panic_safety_traits();
}