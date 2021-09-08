// you should install gnuplot on your os
use gnuplot::*;
use glicol::Engine;

fn main () {
    let mut engine = Engine::<128>::new(44100);
    engine.set_code("out: seq 60 >> ks 60 0.99 0.01");
    // engine.set_code("~left: sin 10; ~right: sin 20; out: balance ~left ~right 0.5;");
    // engine.set_code("tt: sin 44 >> amplfo 1.0");
    plot(engine, 88200);
}

fn plot(mut engine: Engine::<128>, step: usize) {
    engine.make_graph().unwrap();
    println!("node_by_chain {:?}", engine.node_by_chain);
    let mut x = Vec::<i32>::new();
    let mut y = Vec::<f32>::new();
    let mut y2 = Vec::<f32>::new();
    let mut n = 0;

    for _ in 0..(step / 128) {
        let out = engine.gen_next_buf(&mut [0.0;128]).unwrap().0;
        // let out = engine.gen_next_buf_64().unwrap();
        for i in 0..128 {
            x.push(n);
            n += 1;
            y.push(out[i]);
            y2.push(out[i+128])
        }
        // print!("out: {:?}", out);
    }
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Glicol output", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .lines(
            &x,
            &y,
            &[Caption("left")],
        ).lines(
            &x,
            &y2,
            &[Caption("right")],
        );
    fg.show().unwrap();
}