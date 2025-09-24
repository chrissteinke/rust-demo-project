use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dataset {
    point1: Point,
    point2: Point,
    point_list: Vec<Point>,
}

fn main() {
    let point1 = Point { x: 1, y: 2 };
    let point2 = Point { x: 2, y: 4 };
    let point3 = Point { x: 3, y: 6 };
    let point_list = vec![point1, point2, point3];
    let dataset = Dataset {
        point1,
        point2,
        point_list,
    };
    let serialized = serde_json::to_string(&dataset).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Dataset = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

#[cfg(test)]
mod tests {
    use super::*;

    ///
    ///
    ///
    #[test]
    fn point() {
        let point = Point { x: 1, y: 2 };
        let serialized = serde_json::to_string(&point).unwrap();
        println!("serialized = {}", serialized);
        // raw-string, no need to escape quotes or newline
        let raw_string = r#"{"x":1,"y":2}"#;
        assert_eq!(serialized.as_str(), raw_string);
    }
}
