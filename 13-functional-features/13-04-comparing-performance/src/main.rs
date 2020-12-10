fn main() {
    println!("\"3..4\": {:?}", 3..4);
    println!("\"3..3\": {:?}", 3..3);
    println!("\"3..2\": {:?}", 3..2);
    println!("\"3..1\": {:?}", 3..1);
    println!("\"3..0\": {:?}", 3..0);

println!("\"3 - 3..4\": {:?}", 3 - 3..4);
println!("\"3 - 3..3\": {:?}", 3 - 3..3);
println!("\"3 - 3..2\": {:?}", 3 - 3..2);
println!("\"3 - 3..1\": {:?}", 3 - 3..1);
println!("\"3 - 3..0\": {:?}", 3 - 3..0);
}

fn foo() {
    let buffer: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;

    for i in 12..buffer.len() {
        let prediction = coefficients
            .iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>()
            >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}
