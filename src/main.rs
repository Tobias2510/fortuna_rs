use opencv::{
    core::{Mat, MatTraitConstManual, no_array},
    highgui::{self, wait_key},
    videoio::{self, VideoCaptureTrait, VideoCaptureTraitConst},
};
use random::entropy_pool::EntropyPool;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    if !cap.is_opened()? {
        panic!("Unable to open default camera!");
    }

    let mut pool = EntropyPool::new();

    let mut frame = Mat::default();
    let mut inversed = Mat::default();
    loop {
        cap.read(&mut frame)?;

        opencv::core::bitwise_not(&frame, &mut inversed, &no_array())?;

        highgui::imshow("frame", &frame)?;

        let frame_bytes: &[u8] = frame.data_bytes()?;
        // frame_bytes = frame_bytes + b"hello";
        pool.mix(frame_bytes);

        println!("{:?}", hex::encode(pool.state));

        if wait_key(10 as i32)? == 27 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    cap.release()?;

    Ok(())
}
