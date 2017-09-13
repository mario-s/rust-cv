extern crate opencv;
use std::fs::File;
use opencv::core;
use opencv::core::Mat;
//use opencv::contrib;

pub struct FisherFaces {
    faceRecognizer: bool
}

impl FisherFaces {
    pub fn new(path: &str) -> FisherFaces {
        FisherFaces {faceRecognizer: false}
    }

    fn train(&self) {

    }

    pub fn predict(&self, image: &Mat) -> String {
        return "unicorn".to_string();
    }
}
