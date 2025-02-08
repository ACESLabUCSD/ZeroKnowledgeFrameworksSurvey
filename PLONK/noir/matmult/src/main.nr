fn main(a : [Field; 1024], b : [Field; 1024], c : pub [Field; 1024]) {
    let mut temp_field = [0; 1024];
    for i in 0..32 {
        for j in 0..32 {
            for k in 0..32 {
                temp_field[i*32 + j] += temp_field[i*32 + j] + a[i * 32 + k] * b[k * 32 + j];
            }
        }
    }

    for i in 0..1024 {
        constrain c[i] == temp_field[i];
    }
}