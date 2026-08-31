#[derive(Debug)]
pub enum UsersErrors {
    NameAlreadyRegistered,
    EmailAlreadyRegistered,
    NotFound,
    InvalidPassword,
    SqlxError
}