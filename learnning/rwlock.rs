use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::{ops::Deref, ptr::NonNull};

struct Inner {
    a: String,
    _b: String,
}

pub struct MyValue(Arc<RwLock<Inner>>);

impl MyValue {
    pub fn new(a: String, _b: String) -> Self {
        Self(Arc::new(RwLock::new(Inner { a, _b })))
    }

    pub fn get_a(&self) -> ValueRef<'_, String> {
        let guard = self.0.read().expect("should not be poisoned");

        let mut value_ref = ValueRef {
            _guard: guard,
            value: NonNull::dangling(),
        };

        let value = NonNull::from(&value_ref._guard.a);
        value_ref.value = value;

        value_ref
    }
}

pub struct ValueRef<'a, T> {
    _guard: RwLockReadGuard<'a, Inner>,
    value: NonNull<T>,
}

impl<'a, T> ValueRef<'a, T> {
    pub fn addr(&self) -> String {
        println!("{:#?}", self.value);
        format!("{:p}", self.value.as_ptr())
    }
}

impl<'a, T> Deref for ValueRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

pub fn main() {
    let value = MyValue::new("a value".to_owned(), "b_value".to_owned());

    let a_value = value.get_a();

    {
        let addr_1 = a_value.addr();

        let unmoved_a = a_value;
        let addr_2 = unmoved_a.addr();

        assert_eq!(addr_1, addr_2);

        let other_ref = NonNull::from(&unmoved_a._guard.a);

        // assert_eq!(unmoved_a.value, other_ref);
        assert_eq!(unmoved_a.deref(), unsafe { other_ref.as_ref() });
    }
}
