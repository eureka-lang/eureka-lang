pub use number_type::NumberType;
mod number_type;

pub use value_type::ValueType;
mod value_type;

pub type ResultType = crate::wasm::Vector<ValueType>;
