#![allow(deprecated)]

extern crate processing as p5;
extern crate num_traits;
extern crate time;
extern crate rand;

use std::f32;
use std::f64;

use rand::distributions::{IndependentSample, Normal};
use num_traits::float::Float;
use p5::shapes::ellipse::Ellipse;
use p5::shapes::rect::Rect;
use p5::shapes::arc::Arc;
use p5::shapes::line::Line;
use p5::shapes::point::Point;
use p5::shapes::triangle::Triangle;
use p5::shapes::quad::Quad;
use p5::shapes::cube::Cube;

use p5::errors::ProcessingErr;

fn main() -> Result<(), ProcessingErr> {
    let normal = Normal::new(0.0, 1.0);
    let mut rng = rand::thread_rng();
    //let glf = p5::Screen::init()?;
	//let mut screen = p5::Screen::new(300, 300, glf, true, false, false)?;
    let mut screen = p5::Screen::new(300, 300, true, false, true)?;

    let mut ftbf = [0; 600];
    let mut ft = [0; 600];
    let mut x = 1;
    let mut t = 0.;

    let img = p5::load_image("test.jpg")?;
    let (tex, _, _) = screen.texture(&img)?;

    screen.space_wait();
    screen.no_cursor()?;

    screen.stroke(&[0.], &[0.], &[0.], &[1.]);
    screen.fill(&[0.7], &[0.7], &[0.7], &[1.0]);
    let e1 = Ellipse::new(&screen, &[0.], &[0.], &[0.], &[0.3], &[0.5])?;
    let e2 = Ellipse::new(&screen, &[-0.5], &[0.5], &[0.], &[0.2], &[0.2])?;
    let tr1 = Triangle::new(&screen,
                            &[0.3],
                            &[0.75],
                            &[0.],
                            &[0.58],
                            &[0.20],
                            &[0.],
                            &[0.86],
                            &[0.75],
                            &[0.])?;
    let tr2 = Triangle::new(&screen,
                            &[0.3],
                            &[0.6],
                            &[0.],
                            &[0.58],
                            &[0.40],
                            &[0.],
                            &[0.86],
                            &[0.7],
                            &[0.])?;
    screen.stroke_weight(5f32);
    let p = Point::new(&mut screen,
                       &(0..10)
                           .map(|_| normal.ind_sample(&mut rng))
                           .collect::<Vec<_>>(),
                       &(0..10)
                           .map(|_| normal.ind_sample(&mut rng))
                           .collect::<Vec<_>>(),
                       &(0..10)
                           .map(|_| 0.)
                           .collect::<Vec<_>>())?;
    screen.stroke_weight(1f32);
    screen.fill(&[0.], &[0.], &[0.9], &[1.]);
    let q = Quad::new(&screen,
                      &[-0.3],
                      &[-0.75],
                      &[0.0],
                      &[-0.58],
                      &[-0.20],
                      &[0.0],
                      &[-0.86],
                      &[-0.75],
                      &[0.0],
                      &[-0.2],
                      &[-0.4],
                      &[0.0])?;
    let l1 = Line::new(&screen, &[0.5], &[-0.4], &[0.], &[0.7], &[-0.5], &[0.])?;
    let r1 = Rect::new(&screen, &[-0.6], &[-0.4], &[0.], &[0.2], &[0.5])?;
    let l2 = Line::new(&screen, &[0.7], &[-0.4], &[0.], &[0.5], &[-0.5], &[0.])?;
    screen.stroke(&[0.9], &[0.], &[0.], &[1.]);
    let a = Arc::new(&screen,
                     &[-0.6],
                     &[0.6],
                     &[0.0],
                     &[0.3],
                     &[0.3],
                     &[0.4 * f64::consts::PI],
                     &[f64::consts::PI])?;
    screen.stroke(&[0.], &[0.9], &[0.], &[1.]);
    screen.fill(&[1.0], &[1.0], &[1.0], &[1.0]);
    let e3 = Ellipse::new(&screen, &[0.], &[0.], &[0.], &[0.2], &[0.2])?;
    screen.fill(&[t.sin() / 2. + 0.5],
                &[t.cos() / 2. + 0.5],
                &[(t.sin() * t.cos()) / 2. + 0.5],
                &[1.0]);
    let c = Cube::new(&screen, &[0.15])?;
    screen.fill(&[1.], &[1.], &[1.], &[1.]);
    let mut r2 = Rect::new(&screen, &[-0.1], &[0.6], &[0.], &[0.2], &[0.2])?;
    r2.attach_texture(&tex);

    while x < 600 {
        let st = time::precise_time_ns();
        screen.background(0.94, 0.92, 0.9, 1.0);
        screen.stroke_on();
        screen.draw(&e1)?;
        screen.draw(&e2)?;
        screen.draw(&tr1)?;
        screen.draw(&tr2)?;
        screen.draw(&p)?;
        // screen.text("processing-rs", &[0.25], -&[0.85]);
        screen.draw(&q)?;
        screen.draw(&l1)?;
        screen.fill_off();
        screen.draw(&r1)?;
        screen.draw(&l2)?;
        screen.draw(&a)?;
        screen.fill_on();
        screen.draw(&e3)?;
        screen.push_matrix();
        screen.rotate_y(f32::consts::PI / 10. * t);
        screen.rotate_x(f32::consts::PI / 10. * t);
        screen.translate(0.2, -0.5, 0.);
        screen.draw(&c)?;
        screen.pop_matrix();
        screen.stroke_off();
        screen.draw(&r2)?;
        // if screen.key_press(p5::Key::Space) {
        //     screen.save("screenshot.png");
        //     println!("key pressed and screenshot saved.");
        // }
        if screen.mouse_press(p5::MouseButton::Left) {
            println!("bye!");
            break;
        }
        ftbf[x - 1] = time::precise_time_ns() - st;
        screen.reveal()?;
        ft[x - 1] = time::precise_time_ns() - st;
        t += 1. / 60.;
        x += 1;
    }

    let duration_s = (ft.iter().fold(0, |acc, v| acc + v) as f64) / 1_000_000_000f64;
    let fps = (x as f64) / duration_s;
    let dropped = (duration_s - (x as f64 * (1f64 / 60f64))) / (1f64 / 60f64);

    println!("{} frames in {:.6} seconds = {:.3} fps (estimated {:.1} frames dropped)",
             x,
             duration_s,
             fps,
             dropped);

    println!("{:?}", &ftbf[..]);

    println!("{:?}",
             (ft[35..ft.len() - 2].iter().fold(0., |acc, &v| acc + v as f64)) /
             (ft[35..ft.len() - 2].len() as f64));


    let mut t = Vec::with_capacity(ft[35..ft.len() - 2].len());
    for (_, x) in ft[35..ft.len() - 2].iter().enumerate() {
        t.push(*x as f64);
    }
    println!("{:?}", std(&t));
    
    Ok(())
}

pub fn mean(x: &Vec<f64>) -> f64 {
    x.iter().fold(0f64, |total, y| total + y) / (x.len() as f64)
}

fn std(x: &Vec<f64>) -> f64 {
    // This uses the corrected two-pass algorithm (1.7), from "Algorithms for computing
    // the sample variance: Analysis and recommendations" by Chan, Tony F., Gene H. Golub,
    // and Randall J. LeVeque.

    let m = mean(x);
    let mut ss = 0f64;
    let mut compensation = 0f64;
    for v in x {
        let d = *v - m;
        ss += d * d;
        compensation += d;
    }
    let variance = (ss - compensation * compensation / (x.len() as f64)) / (x.len() as f64 - 1.);
    variance.sqrt()
}
