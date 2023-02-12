pub type Intern<T> = internment::Intern<T>;

pub type String = Intern<str>;
pub type StdString = std::string::String;

pub type BigInt = Intern<num::BigInt>;
pub type StdBigInt = num::BigInt;
