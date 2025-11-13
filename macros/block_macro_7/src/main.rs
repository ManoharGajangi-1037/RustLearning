macro_rules! block_macro {
    ($sample:block) => {
        let x = $sample;
        println!("{}", x);
        // println!("{:?}", $sample);
    };
}

fn main() {
    block_macro!({
        let x = 2;
        x * 3
    });
}
