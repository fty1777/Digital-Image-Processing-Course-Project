use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel};

fn translate_impl<I, P>(image: &I, x: i32, y: i32) -> ImageBuffer<P, Vec<P::Subpixel>>
where
    I: GenericImageView<Pixel = P>,
    P: Pixel + 'static,
    P::Subpixel: 'static,
{
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);

    for (old_x, old_y, pixel) in image.pixels() {
        let new_x = old_x as i32 + x;
        let new_y = old_y as i32 + y;
        if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
            out.put_pixel(new_x as u32, new_y as u32, pixel);
        }
    }

    out
}

fn rotate_impl<I, P>(image: &I, angle: f32) -> ImageBuffer<P, Vec<P::Subpixel>>
where
    I: GenericImageView<Pixel = P>,
    P: Pixel + 'static,
    P::Subpixel: 'static,
{
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);

    let angle_rad = angle.to_radians();
    let (sin_angle, cos_angle) = angle_rad.sin_cos();

    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;

    for new_x in 0..width {
        for new_y in 0..height {
            let old_x = (new_x as f32 - center_x) * cos_angle
                + (new_y as f32 - center_y) * sin_angle
                + center_x;
            let old_y = -(new_x as f32 - center_x) * sin_angle
                + (new_y as f32 - center_y) * cos_angle
                + center_y;

            let old_x = old_x.round() as i32;
            let old_y = old_y.round() as i32;

            if old_x >= 0 && old_x < width as i32 && old_y >= 0 && old_y < height as i32 {
                let pixel = image.get_pixel(old_x as u32, old_y as u32);
                out.put_pixel(new_x, new_y, pixel);
            }
        }
    }

    out
}

fn resize_impl<I, P>(image: &I, width: u32, height: u32) -> ImageBuffer<P, Vec<P::Subpixel>>
where
    I: GenericImageView<Pixel = P>,
    P: Pixel + 'static,
    P::Subpixel: 'static,
{
    let (old_w, old_h) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let old_x = ((x + 1) as f32 / width as f32 * old_w as f32)
                .clamp(1.0, old_w as f32)
                .round() as u32
                - 1;
            let old_y = ((y + 1) as f32 / height as f32 * old_h as f32)
                .clamp(1.0, old_h as f32)
                .round() as u32
                - 1;
            let pixel = image.get_pixel(old_x, old_y);
            out.put_pixel(x, y, pixel);
        }
    }

    out
}

fn mirror_impl<I, P>(image: &I, axis: &str) -> ImageBuffer<P, Vec<P::Subpixel>>
where
    I: GenericImageView<Pixel = P>,
    P: Pixel + 'static,
    P::Subpixel: 'static,
{
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);

    for (old_x, old_y, pixel) in image.pixels() {
        let (new_x, new_y) = match axis {
            "x" => (width - 1 - old_x, old_y),
            "y" => (old_x, height - 1 - old_y),
            _ => (old_x, old_y), // No mirroring if axis is not "x" or "y"
        };

        out.put_pixel(new_x, new_y, pixel);
    }

    out
}

fn stretch_impl<I, P>(image: &I, x: f32, y: f32) -> ImageBuffer<P, Vec<P::Subpixel>>
where
    I: GenericImageView<Pixel = P>,
    P: Pixel + 'static,
    P::Subpixel: 'static,
{
    let (width, height) = image.dimensions();
    let new_w = (width as f32 * x).round() as u32;
    let new_h = (height as f32 * y).round() as u32;

    resize_impl(image, new_w, new_h)
}

pub fn translate(img: DynamicImage, x: Option<i32>, y: Option<i32>) -> DynamicImage {
    let x = x.unwrap_or(0);
    let y = y.unwrap_or(0);
    match img {
        DynamicImage::ImageLuma8(img) => DynamicImage::ImageLuma8(translate_impl(&img, x, y)),
        DynamicImage::ImageRgb8(img) => DynamicImage::ImageRgb8(translate_impl(&img, x, y)),
        _ => DynamicImage::ImageRgb8(translate_impl(&img.to_rgb8(), x, y)),
    }
}

pub fn rotate(img: DynamicImage, angle: Option<f32>) -> DynamicImage {
    let angle = angle.unwrap_or(0.0);
    match img {
        DynamicImage::ImageLuma8(img) => DynamicImage::ImageLuma8(rotate_impl(&img, angle)),
        DynamicImage::ImageRgb8(img) => DynamicImage::ImageRgb8(rotate_impl(&img, angle)),
        _ => DynamicImage::ImageRgb8(rotate_impl(&img.to_rgb8(), angle)),
    }
}

pub fn resize(img: DynamicImage, width: Option<u32>, height: Option<u32>) -> DynamicImage {
    let width = width.unwrap_or(img.width());
    let height = height.unwrap_or(img.height());
    match img {
        DynamicImage::ImageLuma8(img) => DynamicImage::ImageLuma8(resize_impl(&img, width, height)),
        DynamicImage::ImageRgb8(img) => DynamicImage::ImageRgb8(resize_impl(&img, width, height)),
        _ => DynamicImage::ImageRgb8(resize_impl(&img.to_rgb8(), width, height)),
    }
}

pub fn mirror(img: DynamicImage, axis: &str) -> DynamicImage {
    match img {
        DynamicImage::ImageLuma8(img) => DynamicImage::ImageLuma8(mirror_impl(&img, axis)),
        DynamicImage::ImageRgb8(img) => DynamicImage::ImageRgb8(mirror_impl(&img, axis)),
        _ => DynamicImage::ImageRgb8(mirror_impl(&img.to_rgb8(), axis)),
    }
}

pub fn stretch(img: DynamicImage, x: Option<f32>, y: Option<f32>) -> DynamicImage {
    let x = x.unwrap_or(1.0);
    let y = y.unwrap_or(1.0);
    match img {
        DynamicImage::ImageLuma8(img) => DynamicImage::ImageLuma8(stretch_impl(&img, x, y)),
        DynamicImage::ImageRgb8(img) => DynamicImage::ImageRgb8(stretch_impl(&img, x, y)),
        _ => DynamicImage::ImageRgb8(stretch_impl(&img.to_rgb8(), x, y)),
    }
}
