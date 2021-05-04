use std::{fs::File, env::args, io::Read};
use yarustc8e::Chip8;
use std::time::{Duration, Instant};
use std::thread::sleep;
use booldisplay::display;

const FRAME: Duration = Duration::from_millis(1000/60);

fn main() {

    let mut args = args();

    args.next();
    
    let filename = match args.next() {
        Some(v) => v,
        None => panic!("Usage: yarustc8e-cli [rom]")
    };

    if let Some(v) = args.next() {
        panic!("Too many arguments supplied! Usage: yarustc8e-cli [rom]")
    }

    let mut buf = Vec::new();
    let mut f = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("{}", e)
    };

    match f.read_to_end(&mut buf) {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    }

    let mut chip8 = Chip8::new(0x200, false, None, &|_| false, &|| 0);

    chip8.load(buf, None);

    let mut last_tick = Instant::now();

    // println!("{:?}", chip8.internal_state().ram);

    loop {
        if FRAME > last_tick.elapsed(){
            sleep(FRAME-last_tick.elapsed())
        }
        last_tick = Instant::now();
        if let Err(e) = chip8.evolve() {
            panic!("{}", e)
        };
        chip8.timer_step();
        if chip8.display.dirty() {
            let d = chip8.display.read();
            display(d.iter().map(|x| x.to_vec()).collect());
        }
    }
}
