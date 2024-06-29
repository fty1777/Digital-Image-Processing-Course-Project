use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub fn create_menu() -> Result<Menu, String> {
    let geometric_menu = Submenu::new(
        "几何变换",
        Menu::new()
            .add_item(CustomMenuItem::new(
                "geometric/translate".to_string(),
                "平移",
            ))
            .add_item(CustomMenuItem::new("geometric/rotate".to_string(), "旋转"))
            .add_item(CustomMenuItem::new("geometric/resize".to_string(), "缩放"))
            .add_item(CustomMenuItem::new("geometric/mirror".to_string(), "镜像"))
            .add_item(CustomMenuItem::new("geometric/stretch".to_string(), "拉伸")),
    );

    let color_menu = Submenu::new(
        "灰度/色彩变换",
        Menu::new()
            .add_item(CustomMenuItem::new(
                "color/to_gray".to_string(),
                "转换为灰度图",
            ))
            .add_item(CustomMenuItem::new("color/to_binary".to_string(), "二值化"))
            .add_item(CustomMenuItem::new("color/invert".to_string(), "反色"))
            .add_item(CustomMenuItem::new(
                "color/exponential".to_string(),
                "指数变换",
            ))
            .add_item(CustomMenuItem::new(
                "color/hist_equalize".to_string(),
                "直方图均衡化",
            )),
    );

    let binary_menu = Submenu::new(
        "二元运算",
        Menu::new()
            .add_item(CustomMenuItem::new("binary_op/add".to_string(), "图像相加"))
            .add_item(CustomMenuItem::new("binary_op/sub".to_string(), "图像相减"))
            .add_item(CustomMenuItem::new("binary_op/mul".to_string(), "图像相乘"))
            .add_item(CustomMenuItem::new("binary_op/div".to_string(), "图像相除")),
    );

    let interpolation_menu = Submenu::new(
        "插值",
        Menu::new()
            .add_item(CustomMenuItem::new(
                "interpolate/nearest".to_string(),
                "最近邻",
            ))
            .add_item(CustomMenuItem::new(
                "interpolate/bilinear".to_string(),
                "双线性",
            ))
            .add_item(CustomMenuItem::new(
                "interpolate/bicubic".to_string(),
                "双三次",
            )),
    );

    let filter_menu = Submenu::new(
        "滤波",
        Menu::new()
            .add_item(CustomMenuItem::new("filter/mean".to_string(), "均值滤波"))
            .add_item(CustomMenuItem::new("filter/median".to_string(), "中值滤波"))
            .add_item(CustomMenuItem::new(
                "filter/gaussian".to_string(),
                "高斯滤波",
            ))
            .add_item(CustomMenuItem::new(
                "filter/homomorphic".to_string(),
                "同态滤波",
            ))
            .add_item(CustomMenuItem::new(
                "filter/sobel_sharpen".to_string(),
                "Sobel锐化",
            ))
            .add_item(CustomMenuItem::new(
                "filter/laplacian_sharpen".to_string(),
                "Laplacian锐化",
            ))
            .add_item(CustomMenuItem::new(
                "filter/prewitt_sharpen".to_string(),
                "Prewitt锐化",
            ))
            .add_item(CustomMenuItem::new(
                "filter/roberts_sharpen".to_string(),
                "Roberts锐化",
            )),
    );

    let edge_menu = Submenu::new(
        "边缘检测",
        Menu::new()
            .add_item(CustomMenuItem::new(
                "border/sobel".to_string(),
                "Sobel边缘检测",
            ))
            .add_item(CustomMenuItem::new(
                "border/laplacian".to_string(),
                "Laplacian边缘检测",
            ))
            .add_item(CustomMenuItem::new(
                "border/prewitt".to_string(),
                "Prewitt边缘检测",
            ))
            .add_item(CustomMenuItem::new(
                "border/roberts".to_string(),
                "Roberts边缘检测",
            )),
    );

    let fft_menu = Submenu::new(
        "快速傅里叶变换 (FFT)",
        Menu::new()
            .add_item(CustomMenuItem::new(
                "fft/dft".to_string(),
                "离散傅里叶变换 (DFT)",
            ))
            .add_item(CustomMenuItem::new(
                "fft/dft_non_shifted".to_string(),
                "非平移DFT (non-shifted DFT)",
            ))
            .add_item(CustomMenuItem::new(
                "fft/idft".to_string(),
                "离散傅里叶逆变换 (iDFT)",
            ))
            .add_item(CustomMenuItem::new(
                "fft/idft_non_shifted".to_string(),
                "非平移iDFT (non-shifted iDFT)",
            ))
            .add_item(CustomMenuItem::new(
                "fft/shift_to_center".to_string(),
                "平移至中心",
            ))
            .add_item(CustomMenuItem::new(
                "fft/log_enhance".to_string(),
                "对数增强",
            )),
    );

    let fourier_desc_menu = Submenu::new(
        "傅里叶描述子",
        Menu::new().add_item(CustomMenuItem::new(
            "fourier_desc".to_string(),
            "傅里叶描述子",
        )),
    );

    let menu = Menu::os_default("数字图像处理")
        .add_submenu(geometric_menu)
        .add_submenu(color_menu)
        .add_submenu(binary_menu)
        .add_submenu(interpolation_menu)
        .add_submenu(filter_menu)
        .add_submenu(edge_menu)
        .add_submenu(fft_menu)
        .add_submenu(fourier_desc_menu);

    Ok(menu)
}
