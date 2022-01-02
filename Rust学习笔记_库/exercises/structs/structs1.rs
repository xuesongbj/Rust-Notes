struct ColorClassicStruct<'a> {
    name: &'a str,
    hex: &'a str,
}

struct ColorTupleStruct(
    String,
    String,
);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_structs() {
        let green = ColorClassicStruct{
            name: "green",
            hex: "#00FF00",
        };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct(
            "green".to_string(),
            "#00FF00".to_string(),
        );

        assert_eq!(&green.0, "green");
        assert_eq!(&green.1, "#00FF00");
    }

    #[test]
    fn unit_struct() {
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);
        assert_eq!(message, "UnitStructs are fun!");
    }
}