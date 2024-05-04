pub fn actions(value: u8) -> Vec<&'static str> {
    let reverse = value / 16 % 2 != 0;
    let value = value % 16;
    let mut result = Vec::<&'static str>::new();
    if value & 0x1 == 0x1 {
        result.push("wink");
    }
    if value & 0x2 == 0x2 {
        result.push("double blink");
    }
    if value & 0x4 == 0x4 {
        result.push("close your eyes");
    }
    if value & 0x8 == 0x8 {
        result.push("jump");
    }
    if reverse {
        result.reverse();
    }
    result
}
