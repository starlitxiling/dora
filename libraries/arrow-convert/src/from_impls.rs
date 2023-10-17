use arrow::{
    array::{Array, AsArray, PrimitiveArray, StringArray},
    datatypes::ArrowPrimitiveType,
};
use eyre::ContextCompat;

use crate::ArrowData;

impl From<ArrowData> for arrow::array::ArrayRef {
    fn from(value: ArrowData) -> Self {
        value.0
    }
}

impl From<arrow::array::ArrayRef> for ArrowData {
    fn from(value: arrow::array::ArrayRef) -> Self {
        Self(value)
    }
}

impl TryFrom<&ArrowData> for bool {
    type Error = eyre::Report;
    fn try_from(value: &ArrowData) -> Result<Self, Self::Error> {
        let bool_array = value.as_boolean_opt().context("not a bool array")?;
        if bool_array.is_empty() {
            eyre::bail!("empty array");
        }
        if bool_array.len() != 1 {
            eyre::bail!("expected length 1");
        }
        if bool_array.null_count() != 0 {
            eyre::bail!("bool array has nulls");
        }
        Ok(bool_array.value(0))
    }
}
impl TryFrom<&ArrowData> for u8 {
    type Error = eyre::Report;
    fn try_from(value: &ArrowData) -> Result<Self, Self::Error> {
        let array = value
            .as_primitive_opt::<arrow::datatypes::UInt8Type>()
            .context("not a primitive array")?;
        extract_single_primitive(array)
    }
}
impl TryFrom<&ArrowData> for u16 {
    type Error = eyre::Report;
    fn try_from(value: &ArrowData) -> Result<Self, Self::Error> {
        let array = value
            .as_primitive_opt::<arrow::datatypes::UInt16Type>()
            .context("not a primitive array")?;
        extract_single_primitive(array)
    }
}
impl TryFrom<&ArrowData> for u32 {
    type Error = eyre::Report;
    fn try_from(value: &ArrowData) -> Result<Self, Self::Error> {
        let array = value
            .as_primitive_opt::<arrow::datatypes::UInt32Type>()
            .context("not a primitive array")?;
        extract_single_primitive(array)
    }
}
impl TryFrom<&ArrowData> for u64 {
    type Error = eyre::Report;
    fn try_from(value: &ArrowData) -> Result<Self, Self::Error> {
        let array = value
            .as_primitive_opt::<arrow::datatypes::UInt64Type>()
            .context("not a primitive array")?;
        extract_single_primitive(array)
    }
}
impl TryFrom<&ArrowData> for i8 {
    type Error = eyre::Report;
    fn try_from(value: &ArrowData) -> Result<Self, Self::Error> {
        let array = value
            .as_primitive_opt::<arrow::datatypes::Int8Type>()
            .context("not a primitive array")?;
        extract_single_primitive(array)
    }
}
impl TryFrom<&ArrowData> for i16 {
    type Error = eyre::Report;
    fn try_from(value: &ArrowData) -> Result<Self, Self::Error> {
        let array = value
            .as_primitive_opt::<arrow::datatypes::Int16Type>()
            .context("not a primitive array")?;
        extract_single_primitive(array)
    }
}
impl TryFrom<&ArrowData> for i32 {
    type Error = eyre::Report;
    fn try_from(value: &ArrowData) -> Result<Self, Self::Error> {
        let array = value
            .as_primitive_opt::<arrow::datatypes::Int32Type>()
            .context("not a primitive array")?;
        extract_single_primitive(array)
    }
}
impl TryFrom<&ArrowData> for i64 {
    type Error = eyre::Report;
    fn try_from(value: &ArrowData) -> Result<Self, Self::Error> {
        let array = value
            .as_primitive_opt::<arrow::datatypes::Int64Type>()
            .context("not a primitive array")?;
        extract_single_primitive(array)
    }
}

impl<'a> TryFrom<&'a ArrowData> for &'a str {
    type Error = eyre::Report;
    fn try_from(value: &'a ArrowData) -> Result<Self, Self::Error> {
        let array: &StringArray = value.as_string_opt().wrap_err("not a string array")?;
        if array.is_empty() {
            eyre::bail!("empty array");
        }
        if array.len() != 1 {
            eyre::bail!("expected length 1");
        }
        if array.null_count() != 0 {
            eyre::bail!("array has nulls");
        }
        Ok(array.value(0))
    }
}

impl<'a> TryFrom<&'a ArrowData> for &'a [u8] {
    type Error = eyre::Report;
    fn try_from(value: &'a ArrowData) -> Result<Self, Self::Error> {
        let array: &PrimitiveArray<arrow::datatypes::UInt8Type> =
            value.as_primitive_opt().wrap_err("not a primitive array")?;
        if array.is_empty() {
            eyre::bail!("empty array");
        }
        if array.len() != 1 {
            eyre::bail!("expected length 1");
        }
        if array.null_count() != 0 {
            eyre::bail!("array has nulls");
        }
        Ok(array.values())
    }
}

fn extract_single_primitive<T>(array: &PrimitiveArray<T>) -> Result<T::Native, eyre::Error>
where
    T: ArrowPrimitiveType,
{
    if array.is_empty() {
        eyre::bail!("empty array");
    }
    if array.len() != 1 {
        eyre::bail!("expected length 1");
    }
    if array.null_count() != 0 {
        eyre::bail!("array has nulls");
    }
    Ok(array.value(0))
}
