use std::io::prelude::*;

fn bench_read_slice() {
    let buf = [5; 1024];
    let mut dst = [0; 128];

    for _ in 0..1000 { 
        let mut rd = &buf[..];
        for _ in 0..8 {
            let _ = rd.read(&mut dst);
            std::hint::black_box(&dst);
        }
    }
}

fn bench_write_slice() {
    let mut buf = [0; 1024];
    let src = [5; 128];

    for _ in 0..1000 { 
        let mut wr = &mut buf[..];
        for _ in 0..8 {
            let _ = wr.write_all(&src);
            std::hint::black_box(&wr);
        }
    }
}

fn bench_read_vec() {
    let buf = vec![5; 1024];
    let mut dst = [0; 128];

    for _ in 0..1000 {
        let mut rd = &buf[..];
        for _ in 0..8 {
            let _ = rd.read(&mut dst);
            std::hint::black_box(&dst);
        }
    }
}

fn bench_write_vec() {
    let mut buf = Vec::with_capacity(1024);
    let src = [5; 128];

    for _ in 0..1000 {
        let mut wr = &mut buf[..];
        for _ in 0..8 {
            let _ = wr.write_all(&src);
            std::hint::black_box(&wr);
        }
    }
}

fn main() {
    bench_read_slice();
    bench_write_slice();
    bench_read_vec();
    bench_write_vec();
}