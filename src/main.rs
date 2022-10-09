
use rand::{thread_rng, Rng};


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        panic!("Not enough args");
    }

    let (cmd, arg) = (&args[1], &args[2]);

    match cmd.as_str() {
        "roll" => {
            let arg: i32 = arg.parse().unwrap();
            roll(arg)
        },
       _ => panic!("illegal command") 
    }

}

fn roll(num: i32) {
    let mut res: Vec<u8> = Vec::new();
    let mut rng = thread_rng();

    for _ in 0..num {
        let n = rng.gen_range(1..=6);
        res.push(n);
    }

    let hits = res.iter().filter(|x| x > &&4).count();

    println!("{hits} hits! \n{:?}", res);
}
