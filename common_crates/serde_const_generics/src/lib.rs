use std::{convert::TryInto, marker::PhantomData};

use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeTuple,
    Deserialize, Deserializer, Serialize, Serializer,
};

pub fn serialize<S: Serializer, T: Serialize, const N: usize>(
    data: &[T; N],
    ser: S,
) -> Result<S::Ok, S::Error> {
    let mut s = ser.serialize_tuple(N)?;
    for item in data {
        s.serialize_element(item)?;
    }
    s.end()
}

struct ArrayVisitor<T, const N: usize>(PhantomData<T>);

impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
    where
        T: Deserialize<'de>,
{
    type Value = [T; N];

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("an array of length {}", N))
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
    {
        // can be optimized using MaybeUninit
        let mut data = Vec::with_capacity(N);
        for _ in 0..N {
            match (seq.next_element())? {
                Some(val) => data.push(val),
                None => return Err(serde::de::Error::invalid_length(N, &self)),
            }
        }
        match data.try_into() {
            Ok(arr) => Ok(arr),
            Err(_) => unreachable!(),
        }
    }
}

pub fn deserialize<'de, D, T, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
{
    deserializer.deserialize_tuple(N, ArrayVisitor::<T, N>(PhantomData))
}


#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use protocol;

    #[test]
    fn serde_array_test() {
        #[derive(Debug)]
        #[derive(Ord, PartialOrd, Eq, PartialEq)]
        #[derive(Serialize, Deserialize)]
        struct TypeWithArray {
            #[serde(with = "crate")]
            arr: [u8; 64],
        }

        let initial_array = TypeWithArray {
            arr: std::array::from_fn(|i| i as u8),
        };

        let bytes = protocol::serialize(&initial_array);

        let type_with_array: TypeWithArray = protocol::deserialize(bytes.as_ref()).unwrap();

        assert_eq!(initial_array, type_with_array);
    }

    #[test]
    fn serde_const_generics_test() {
        #[derive(Debug)]
        #[derive(Ord, PartialOrd, Eq, PartialEq)]
        #[derive(Serialize, Deserialize)]
        struct TypeWithArray<const SIZE: usize> {
            #[serde(with = "crate")]
            arr: [u8; SIZE],
        }

        let initial_array: TypeWithArray<10> = TypeWithArray {
            arr: std::array::from_fn(|i| i as u8),
        };

        let bytes = protocol::serialize(&initial_array);

        let type_with_array: TypeWithArray<10> = protocol::deserialize(bytes.as_ref()).unwrap();

        assert_eq!(initial_array, type_with_array);
    }


}