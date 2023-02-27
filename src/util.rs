
pub fn copy_data<T>(dest: &mut [T], data: &[T]) 
where T: Copy
{
    for (i, b) in data.iter().enumerate() {
        dest[i] = *b;
    }
}
