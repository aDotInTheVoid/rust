// use-jsondocck-ng

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub struct MyError {}

pub type MyResult<T, E = MyError> = Result<T, E>;
