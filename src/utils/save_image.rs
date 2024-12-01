use image::GrayImage;
use ndarray::Array2;

pub trait ArrayAsGrayImage: Sized {
    fn to_gray_pixel_array(&self) -> Array2<u8>;

    fn to_gray_image(&self) -> GrayImage {
        let arr = self.to_gray_pixel_array();

        assert!(arr.is_standard_layout());

        let (height, width) = arr.dim();
        let raw = arr.into_raw_vec();

        GrayImage::from_raw(width as u32, height as u32, raw)
            .expect("container should have the right size for the image dimensions")
    }

    fn save_as_gray_image(&self, path: &str) {
        self.to_gray_image().save(path).unwrap()
    }
}

impl ArrayAsGrayImage for Array2<bool> {
    fn to_gray_pixel_array(&self) -> Array2<u8> {
        self.map(|&v| if v { 255 } else { 0 })
    }
}

impl ArrayAsGrayImage for Array2<u8> {
    fn to_gray_pixel_array(&self) -> Array2<u8> {
        self.clone()
    }
}
