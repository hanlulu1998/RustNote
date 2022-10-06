use image::Luma;
use pyo3::prelude::*;
use qrcode::render::{svg, unicode};
use qrcode::{EcLevel, QrCode, Version};

/// 二维码图片生成函数
#[pyfunction]
fn qrcode_img(data: String, save_path: String) {
    // Encode some data into bits.
    let code = QrCode::new(data.as_bytes()).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save(save_path).unwrap();
}

/// 二维码字符生成函数
#[pyfunction]
fn qrcode_str(data: String) {
    let code = QrCode::new(data.as_bytes()).unwrap();
    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", string);
}

/// 二维码svg生成函数
#[pyfunction]
fn qrcode_svg(data: String) {
    let code = QrCode::with_version(data.as_bytes(), Version::Micro(2), EcLevel::L).unwrap();
    let image = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#ffff80"))
        .build();
    println!("{}", image);
}

/// 二维码unicode生成函数
#[pyfunction]
fn qrcode_unicode(data: String) {
    let code = QrCode::new(data.as_bytes()).unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", image);
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(qrcode_img, m)?)?;
    m.add_function(wrap_pyfunction!(qrcode_str, m)?)?;
    m.add_function(wrap_pyfunction!(qrcode_svg, m)?)?;
    m.add_function(wrap_pyfunction!(qrcode_unicode, m)?)?;
    Ok(())
}
