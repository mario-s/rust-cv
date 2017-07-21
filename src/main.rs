extern crate opencv;
use opencv::highgui;
use opencv::highgui::VideoCapture;
use opencv::core;
use opencv::imgproc;
use opencv::objdetect;

use std::{thread, time};

fn run() -> Result<(),String> {
    let sleep_duration = time::Duration::from_millis(50);
    let window = "video capture";
    let xml = "resources/haarcascade_frontalface_alt.xml";

    try!(highgui::named_window(window,1));

    let mut cam = try!(VideoCapture::device(1));
    let opened = try!(VideoCapture::is_opened(&cam));

    if ! opened {
        println!("Using different camera");
        cam = try!(VideoCapture::device(0));
    }

    let mut face = try!(objdetect::CascadeClassifier::new(xml));

    loop {
        let mut frame = try!(core::Mat::new());
        try!(cam.read(&mut frame));

        if try!(frame.size()).width == 0 {
            thread::sleep(sleep_duration);
            continue;
        }

        let mut gray = try!(core::Mat::new());
        try!(imgproc::cvt_color(&frame, &mut gray, imgproc::CV_BGR2GRAY, 0));

        let mut reduced = try!(core::Mat::new());
        try!(imgproc::resize(&gray, &mut reduced, core::Size{width:0,height:0},
            0.25f64, 0.25f64, imgproc::INTER_LINEAR));

        let mut faces = ::opencv::types::VectorOfRect::new();
        try!(face.detect_multi_scale(&reduced, &mut faces, 1.1, 2,
            objdetect::CV_HAAR_SCALE_IMAGE,
            core::Size{ width:30, height:30 },
            core::Size{ width:0, height:0 }));

        let len = faces.len();
        if len > 0 {
            println!("faces: {}", len);
        }

        for face in faces.iter() {
            println!("face {:?}", face);
            let scaled_face = core::Rect{
                x: face.x*4, y:face.y*4,
                width:face.width*4, height:face.height*4
            };
            try!(core::rectangle(&frame,
                scaled_face,
                core::Scalar{ data:[0f64, 255f64, 0f64, 125f64] },
                2, 8, 0));
        }
        try!(highgui::imshow(window, &frame));

        if try!(highgui::wait_key(10)) > 0 {
            break;
       }
    }

    Ok(())
}

fn main() {
    run().unwrap()
}
