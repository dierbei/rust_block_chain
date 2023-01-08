use bincode;
use serde::{Serialize, Deserialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

// pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>>
// where
//     T: Serialize,
// 泛型：大小不确定；限制必须实现Serialize
pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
    where T: Serialize,
{
    let serialize = bincode::serialize(value).unwrap();
    serialize
}

// pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T>
// where
//     T: Deserialize<'a>,
pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
    where
        T: Deserialize<'a>
{
    let deserialize = bincode::deserialize(bytes).unwrap();
    deserialize
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{my_serialize, my_deserialize, get_hash};

    #[test]
    fn test_serialize() {
        let point = Point{ x: 1, y: 2};
        let se = my_serialize(&point);
        let de: Point = my_deserialize(&se);
        assert_eq!(de, point);
    }

    #[test]
    fn test_get_hash() {
        let point = Point{ x: 1, y: 2 };
        let se = my_serialize(&point);
        println!("{}", get_hash(&se))
    }
}