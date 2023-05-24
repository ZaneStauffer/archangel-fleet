use rand::Rng;

pub fn generate_hex_ID() -> String{
    let mut rng = rand::thread_rng();
    let random_num: u32 = rng.gen_range(0..=0xFFFFFF);

    format!("{:06X}", random_num)
}