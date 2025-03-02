
fn main() {
    println!("iterator VS loop");
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_loop() {
        let buffer: &mut [i32] = &mut [1,2,3,4,5];
        let coefficients: [i64; 12] = [3;12];
        let qlp_shift: i16 = 30;

        for i in 12..buffer.len() {
            let prediction = coefficients.iter()
                                        .zip(&buffer[i - 12..i])
                                        .map(|(&c, &s)| c * s as i64)
                                        .sum::<i64>() >> qlp_shift;
            let delta = buffer[i];
            buffer[i] = prediction as i32 + delta;
        }
    }
}