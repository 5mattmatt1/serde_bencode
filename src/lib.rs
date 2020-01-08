pub mod de;
pub mod error;
pub mod ser;
mod read;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    struct TestStruct
    {
        a: String,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    struct TestStructInt
    {
        a: i32
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    struct TestStructVec
    {
        a: Vec<String>
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    struct TestStructMap
    {
        a: String,
        b: TestStruct,
    }

    #[test]
    fn ser_str()
    {
        use crate::ser;
        let example: String = "a".to_string();
        let example_ser: String = ser::to_str(&example).unwrap();
        println!("Serialized String: {}", example_ser);
        assert!(example_ser == "1:a")
    }

    #[test]
    fn ser_int()
    {
        use crate::ser;
        let example: i32 = 10;
        let example_ser: String = ser::to_str(&example).unwrap();
        println!("Serialized Integer: {}", example_ser);
        assert!(example_ser == "i10e")
    }

    #[test]
    fn ser_vec()
    {
        use crate::ser;
        let example: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let example_ser: String = ser::to_str(&example).unwrap();
        println!("Serialized Vector: {}", example_ser);
        assert!(example_ser == "l1:a1:b1:ce")
    }

    #[test]
    fn ser_map()
    {
        use crate::ser;
        let example: TestStruct = TestStruct {
            a: "hello".to_string()
        };
        let example_ser: String = ser::to_str(&example).unwrap();
        println!("Serialized Str Map: {}", example_ser);
        assert!(example_ser == "d1:a5:helloe")
    }

    #[test]
    fn ser_map_int()
    {
        use crate::ser;
        let example: TestStructInt = TestStructInt {
            a: 10
        };
        let example_ser: String = ser::to_str(&example).unwrap();
        println!("Serialized Int Map: {}", example_ser);
        assert!(example_ser == "d1:ai10ee")
    }

    #[test]
    fn ser_map_vec()
    {
        use crate::ser;
        let example: TestStructVec = TestStructVec {
            a: vec!["a".to_string(), "b".to_string(), "c".to_string()]
        };
        let example_ser: String = ser::to_str(&example).unwrap();
        println!("Serialized Vec Map: {}", example_ser);
        assert!(example_ser == "d1:al1:a1:b1:cee")
    }

    #[test]
    fn ser_map_map()
    {
        use crate::ser;
        let example: TestStructMap = TestStructMap {
            a: "hello".to_string(),
            b: TestStruct {
                a: "world".to_string()
            }
        };
        let example_ser: String = ser::to_str(&example).unwrap();
        println!("Serialized Map Map: {}", example_ser);
        assert!(example_ser == "d1:a5:hello1:bd1:a5:worldee")
    }

    #[test]
    fn de_str()
    {
        use crate::de;
        let example: String = "1:a".to_string();
        let example_de: String = de::from_str(&example).unwrap();
        println!("Deserialized String: {}", example_de);
        assert!(example_de == "a")
    }

    #[test]
    fn de_int()
    {
        use crate::de;
        let example: String = "i10e".to_string();
        let example_de: i32 = de::from_str(&example).unwrap();
        println!("Deserialized Integer: {}", example_de);
        assert!(example_de == 10)
    }

    #[test]
    fn de_vec()
    {
        use crate::de;
        let example: String = "l1:a1:b1:ce".to_string();
        let example_de: Vec<String> = de::from_str(&example).unwrap();
        for (i, de_val) in example_de.iter().enumerate()
        {
            println!("Deserialized Vec<String>[{}]: {}", i, de_val);
        }
        assert!(example_de == vec!["a", "b", "c"])
    }

    #[test]
    fn de_map() 
    {
        use crate::de;
        let example: String = "d1:a5:helloe".to_string();
        let example_de: TestStruct = de::from_str(&example).unwrap();
        assert!(example_de.a == "hello");
    }

    #[test]
    fn de_map_int()
    {
        use crate::de;
        let example: String = "d1:ai10ee".to_string();
        let example_de: TestStructInt = de::from_str(&example).unwrap();
        assert!(example_de.a == 10)
    }

    #[test]
    fn de_map_vec()
    {
        use crate::de;
        let example: String = "d1:al1:a1:b1:cee".to_string();
        let example_de: TestStructVec = de::from_str(&example).unwrap();
        // for (i, de_val) in example_de.iter().enumerate()
        // {
        //     println!("Deserialized Vec<String>[{}]: {}", i, de_val);
        // }
        assert!(example_de.a == vec!["a", "b", "c"])
    }

    #[test]
    fn de_map_map() 
    {
        use crate::de;
        let example: String = "d1:a5:hello1:bd1:a5:worldee".to_string();
        let example_de: TestStructMap = de::from_str(&example).unwrap();
        assert!(example_de.a == "hello");
        assert!(example_de.b.a == "world");
    }
}
