use std::fs;

pub struct Res<'a, T, E> {
    t: Option<&'a T>,
    e: Option<&'a E>,
}

impl<'a, T, E> Res<'a, T, E> {
    pub fn unwrap(&self) -> T {
        if e != None {
            panic!("{}", e);
            fs::write("log/log.txt", format!("{}", e).bytes());
        }

        T
    }

    pub fn unwrap_or_else<F>(&self, f: F) -> T 
    where
        Fn(E) -> ()
    {
        if e != None {
            f(self.e)
            fs::write("log/log.txt", format!("{}", e).bytes());
            return T::new();
        }

        T
    }
}

pub fn Err<E>(e: E) -> Res<(), E> {
    Res<(), E> {
        t: None,
        e,
    }
}

pub fn Ok<T>(t: T) -> Res<T, ()> {
    Res<T, ()> {
        t,
        e: None,
    }
}
