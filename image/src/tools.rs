pub fn index_as_u32(file: &Vec<u8>, index: usize) -> usize {
    let a = file.as_slice();
    let b = a[index..index + 4]
        .try_into()
        .unwrap_or_else(|x| panic!("X is {}", x));
    u32::from_le_bytes(b) as usize
}
