const COLOR_BASE: u8 = 128;

pub fn get_true_color(str: &String, is_dark_text: bool) -> [u8; 3] {
    let digest: [u8; 16] = md5::compute(str).into();
    let mut red = digest[0] >> 1;
    let mut green = digest[1] >> 1;
    let mut blue = digest[2] >> 1;
    let palette_nbr = digest[0] % 3;
    if is_dark_text {
        red = 245 - red;
        green = 245 - green;
        blue = 245 - blue;
    }
    match palette_nbr {
        0 => {
            return [
                COLOR_BASE + red,
                COLOR_BASE + green,
                COLOR_BASE
            ]
        }
        1 => {
            return [
                COLOR_BASE + red,
                COLOR_BASE,
                COLOR_BASE + blue
            ]
        }
        2 => {
            return [
                COLOR_BASE,
                COLOR_BASE + green,
                COLOR_BASE + blue
            ]
        }
        _ => {
            return [
                COLOR_BASE + 64,
                COLOR_BASE + 64,
                COLOR_BASE + 64
            ]
        }
    }
}
