use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

pub trait Handler<T>: 'static {
    fn call(&self, args: T);
}

impl<T, S> Handler<S> for Arc<T>
where
    T: Handler<S>,
{
    fn call(&self, args: S) {
        self.as_ref().call(args);
    }
}

pub struct Source<T: ?Sized> {
    handler: Arc<dyn Handler<T>>,
}

impl<T: 'static> Handler<T> for Source<T> {
    fn call(&self, args: T) {
        self.handler.call(args);
    }
}

#[derive(Default)]
pub struct Router {
    handlers: HashMap<TypeId, Box<dyn Any>>,
}

impl Router {
    pub fn on<T, Args>(&mut self, handler: T)
    where
        T: Handler<Args>,
        Args: 'static,
    {
        let source = Source {
            handler: Arc::new(handler),
        };
        self.handlers.insert(TypeId::of::<Args>(), Box::new(source));
    }

    pub fn emit<T: 'static>(&self, event: T) {
        if let Some(source) = self.source::<T>() {
            source.call(event);
        }
    }

    pub fn source<T: 'static>(&self) -> Option<&Source<T>> {
        if let Some(source) = self.handlers.get(&TypeId::of::<T>()) {
            source.downcast_ref::<Source<T>>()
        } else {
            None
        }
    }
}

impl<Func, T> Handler<T> for Func
where
    Func: 'static + Fn(T) -> (),
{
    fn call(&self, args: T) {
        (self)(args);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct Test {
        foo: i32,
    }

    pub struct Foo {
        bar: i32,
    }

    #[test]
    fn test_router() {
        let mut router = Router::default();
        router.on(|i: i32| {
            println!("{}", i * 10);
        });
        router.on(|(a, b): (i32, i32)| {
            println!("{}", a * b);
        });
        router.on(|test: Test| {
            println!("test {}", test.foo);
        });
        router.on(|foo: Foo| {
            println!("bar {}", foo.bar);
        });

        router.emit(3);
        router.emit((2, 3));
        router.emit(Foo { bar: 232 });
        router.emit(Test { foo: 232 });
    }
}
