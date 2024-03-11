use opencv::prelude::*;
use opencv::{core, highgui, imgproc, videoio, Result};

fn main() -> Result<()> {
    // Open the default camera
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;

    // Check if camera opened successfully
    if !cap.is_opened()? {
        println!("Unable to open camera");
        return Ok(());
    }

    // Create window for raw video and edges
    highgui::named_window("Edges", highgui::WINDOW_NORMAL)?;

    // Initialize frame
    let mut frame = core::Mat::default();

    loop {
        // Read frame from camera
        cap.read(&mut frame)?;

        // Check if the video has ended
        if frame.empty() {
            break;
        }

        // Run the Canny edge detector
        let mut edges = core::Mat::default();
        imgproc::canny(&frame, &mut edges, 50.0, 150.0, 3, false)?;

        // Display the video frame in the window
        highgui::imshow("Edges", &edges)?;

        // Wait for a key press to exit
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }

    // Release the camera and destroy the window
    cap.release()?;
    highgui::destroy_window("Video")?;

    Ok(())
}
