#[macro_use]
extern crate processingrs as p5;
extern crate image;
extern crate num_traits;
extern crate time;

use std::f32;

use p5::Screen;
use p5::shapes2d::Rect;
use num_traits::float::Float;

fn main() {
    let mut screen = Screen::new(300, 300, false, true);

    let mut ftbf = [0; 600];
    let mut ft = [0; 600];
    let mut x = 1;
    let mut t = 0.;

    let img = image::open("test.jpg").unwrap().to_rgba();
    let (tex, _, _) = screen.texture(&img);

    println!("yo");

    while x < 600 {
        let st = time::precise_time_ns();
        let uniforms = create_uniforms!{screen, tex: &tex};
        screen.background(0.94, 0.92, 0.9, 1.0);
        screen.stroke(&[0.], &[0.], &[0.], &[1.]);
        screen.fill(&[0.7], &[0.7], &[0.7], &[1.0]);
        // screen.ellipse(0, 0, &[0.3], &[0.5]);
        // screen.ellipse(-.5, .5, &[0.2], &[0.2]);
        // screen.triangle(.3, .75, .58, .20, .86, .75);
        // screen.triangle(.3, .6, .58, .40, .86, .7);
        // screen.strokeWeight(&[5.]);
        // screen.point(randn(10), randn(10));
        // screen.text("screen.jl", &[0.25], -&[0.85]);
        // screen.strokeWeight(&[1.]);
        screen.fill(&[0.], &[0.], &[0.9], &[1.]);
        // screen.quad(-.3, -.75, -.58, -.20, -.86, -.75, -.2, -.4);
        // screen.line(.5, -.4, .7, -.5);
        screen.noFill();
        let r = Rect::new(&screen, &[-0.6], &[-0.4], &[0.], &[0.2], &[0.5], "CORNER");
        screen.draw(&r, &uniforms);
        // screen.line(.7, -.4, .5, -.5);
        screen.stroke(&[0.9], &[0.], &[0.], &[1.]);
        // screen.arc(-.6, .6, .3, .3, &[0.4]*f32::consts::PI, f32::consts::PI);
        screen.stroke(&[0.], &[0.9], &[0.], &[1.]);
        screen.fill(&[1.0], &[1.0], &[1.0], &[1.0]);
        // screen.ellipse(0, 0, &[0.2], &[0.2]);
        screen.fill(
            &[t.sin() / 2. + 0.5],
            &[t.cos() / 2. + 0.5],
            &[(t.sin() * t.cos()) / 2. + 0.5],
            &[1.0],
        );
        screen.pushMatrix();
        screen.rotateY(f32::consts::PI / 10. * t);
        screen.rotateX(f32::consts::PI / 10. * t);
        screen.translate(0.2, -0.5, 0.);
        // screen.box(&[0.15]);
        screen.popMatrix();
        screen.noStroke();
        screen.fill(&[1.], &[1.], &[1.], &[1.]);
        let r = Rect::new(&screen, &[-0.1], &[0.6], &[0.], &[0.2], &[0.2], "CORNER");
        screen.draw_with_texture(&r, &uniforms);
        // if screen.keyPress(window, GLFW.KEY_SPACE) {
        // screen.save("screenshot.tiff");
        // println!("key pressed and screenshot saved.");
        // }
        // if screen.mousePress(window, GLFW.MOUSE_BUTTON_LEFT) {
        // println!("bye!");
        // break;
        // }
        ftbf[(screen.frameCount() + 1) as usize] = time::precise_time_ns() - st;
        screen.reveal();
        ft[(screen.frameCount()) as usize] = time::precise_time_ns() - st;
        t += 1. / 60.;
        x += 1;
    }

    let duration_s = (ft.iter().fold(0, |acc, v| acc + v) as f64) / 1_000_000_000f64;
    let fps = (x as f64) / duration_s;
    let dropped = (duration_s - (x as f64 * (1f64 / 60f64))) / (1f64 / 60f64);

    println!(
        "{} frames in {:.6} seconds = {:.3} fps (estimated {:.1} frames dropped)",
        x,
        duration_s,
        fps,
        dropped
    );

    println!("{:?}", ftbf);
}
