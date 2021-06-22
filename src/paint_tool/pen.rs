use crate::data::input_state::InputState;
use crate::data::pointing::KusaPoint;
use crate::paint_tool::screen_to_image;
use crate::paint_tool::PaintTool;
use crate::piston_wrapper::kusa_image::KusaImage;
use crate::settings::Settings;
use crate::write_k_image;

pub struct Pen {}
impl PaintTool for Pen {
    fn on_mouse_pressed(
        &self,
        settings: &Settings,
        input_state: &InputState,
        k_image: &mut KusaImage,
    ) {
        // 点を置きます
        Pen::put_pixel(k_image, &input_state.pressed_point, &settings);

        // 保存
        write_k_image(&k_image, &settings.image_file);
    }
    fn on_mouse_moved(
        &self,
        settings: &Settings,
        input_state: &InputState,
        k_image: &mut KusaImage,
    ) {
        if input_state.is_mouse_pressed {
            // 移動した区間に連続した点を置きます
            self.draw_line(&settings, k_image, &input_state);
            //println!(
            //    "Trace   | Click ({}, {}) 保存",
            //    &k_mouse_cursor.x, &k_mouse_cursor.y
            //);
            write_k_image(&k_image, &settings.image_file);
            // 保存
            write_k_image(&k_image, &settings.image_file);
        }
    }
    fn on_mouse_released(
        &self,
        _settings: &Settings,
        _input_state: &InputState,
        _k_image: &mut KusaImage,
    ) {
    }
}
impl Pen {
    // 点を置くぜ（＾～＾）
    fn put_pixel(k_image: &mut KusaImage, sc_coord: &KusaPoint, settings: &Settings) {
        // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
        if let Some(coord) = screen_to_image(sc_coord.x, sc_coord.y, settings) {
            k_image.set_pixel(coord.0 as u32, coord.1 as u32, &settings.paint_color);
        }
    }

    // 線を引くぜ（＾～＾）
    fn draw_line(&self, settings: &Settings, k_image: &mut KusaImage, input_state: &InputState) {
        if let Some(pressed_im_coord) = screen_to_image(
            input_state.previous_point.x,
            input_state.previous_point.y,
            settings,
        ) {
            // println!(
            //     "Trace   | pressed_im_coord 0={} 1={}",
            //     pressed_im_coord.0, pressed_im_coord.1
            // );

            // // 1未満は 1に切り上げます。-1未満は-1に切り上げます
            // if 0.0 < sc_dx && sc_dx < 1.0 {
            //     sc_dx = 1.0;
            // } else if -1.0 < sc_dx && sc_dx < 0.0 {
            //     sc_dx = -1.0;
            // }
            // if 0.0 < sc_dy && sc_dy < 1.0 {
            //     sc_dy = 1.0;
            // } else if -1.0 < sc_dy && sc_dy < 0.0 {
            //     sc_dy = -1.0;
            // }
            // //println!("Trace   | sc_dx={} sc_dy={}", sc_dx, sc_dy);

            // 画像上のピクセル位置
            let start_cell = KusaPoint {
                x: input_state.previous_point.x / settings.canvas_dot_width,
                y: input_state.previous_point.y / settings.canvas_dot_height,
            };
            let end_cell = KusaPoint {
                x: (input_state.previous_point.x + input_state.moved_vector.x)
                    / settings.canvas_dot_width,
                y: (input_state.previous_point.y + input_state.moved_vector.y)
                    / settings.canvas_dot_height,
            };

            // 画像上のピクセル数を返します
            let im_dx = end_cell.x - start_cell.x;
            let im_dy = end_cell.y - start_cell.y;
            // ずっと |1|未満 だと何も描かれないので、
            // 1未満は 1に切り上げます。-1未満は-1に切り上げます
            let im_dx_len = {
                let mut im_width = im_dx.abs();
                if 0.0 < im_width && im_width < 1.0 {
                    im_width = 1.0;
                } else if -1.0 < im_width && im_width < 0.0 {
                    im_width = -1.0;
                }
                im_width
            };
            let im_dy_len = {
                let mut im_height = im_dy.abs();
                if 0.0 < im_height && im_height < 1.0 {
                    im_height = 1.0;
                } else if -1.0 < im_height && im_height < 0.0 {
                    im_height = -1.0;
                }
                im_height
            };
            //println!("Trace   | sc_dx={} sc_dy={}", sc_dx, sc_dy);

            // 横長
            let im_landscape = im_dy_len < im_dx_len;
            // 長い方の辺の正負を返します。 1 or -1
            let im_long_edge_sign = if im_landscape {
                if im_dx.is_sign_positive() {
                    1
                } else {
                    -1
                }
            } else {
                if im_dy.is_sign_positive() {
                    1
                } else {
                    -1
                }
            };
            // 短い方の辺の比を返します
            let im_short_edge_rate = if im_landscape {
                if 0.0 < im_dx_len {
                    im_dy_len / im_dx_len
                } else {
                    0.0
                }
            } else {
                if 0.0 < im_dy_len {
                    im_dx_len / im_dy_len
                } else {
                    0.0
                }
            };
            if im_landscape {
                // 横幅の方が長ければ。
                let draw_horizontal = &mut |im_d_col| {
                    let im_d_row = (im_short_edge_rate * im_d_col as f64) as i32;
                    // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                    let im_x = pressed_im_coord.0 + im_d_col;
                    let im_y = pressed_im_coord.1 + im_d_row;
                    println!("Trace   | 右へ（＾～＾） im_x={} im_y={}", im_x, im_y);
                    if 0 <= im_x
                        && im_x < settings.image_width as i32
                        && 0 <= im_y
                        && im_y < settings.image_height as i32
                    {
                        k_image.set_pixel(im_x as u32, im_y as u32, &settings.paint_color);
                    }
                };
                if 0 <= im_long_edge_sign {
                    println!("Trace   | 右へ☆（＾～＾） im_dx_len={}", im_dx_len);
                    for im_d_col in 1..(im_dx_len as i32 + 1) {
                        draw_horizontal(im_d_col);
                    }
                } else {
                    //println!("Trace   | 左へ☆（＾～＾）");
                    for im_d_col in (1..(im_dx_len as i32 + 1)).rev() {
                        draw_horizontal(im_long_edge_sign * im_d_col);
                    }
                }
            } else {
                // 縦幅の方が長いか同じなら。
                let draw_vertical = &mut |im_d_row| {
                    println!(
                        "Trace   | im_short_edge_rate={} im_d_row={}",
                        im_short_edge_rate, im_d_row
                    );
                    let im_d_col = (im_short_edge_rate * im_d_row as f64) as i32;
                    println!("Trace   | im_d_col={}", im_d_col,);
                    // 点を１個打って画像として保存するぜ☆（＾～＾）画面への描画は別のところでやってるぜ☆（＾～＾）
                    let im_x = pressed_im_coord.0 + im_d_col;
                    let im_y = pressed_im_coord.1 + im_d_row;
                    println!("Trace   | im_x={} im_y={}", im_x, im_y);
                    if 0 <= im_x
                        && im_x < settings.image_width as i32
                        && 0 <= im_y
                        && im_y < settings.image_height as i32
                    {
                        k_image.set_pixel(im_x as u32, im_y as u32, &settings.paint_color);
                    }
                };
                if 0 <= im_long_edge_sign {
                    //println!("Trace   | 下へ☆（＾～＾）");
                    for im_d_row in 1..(im_dy_len as i32 + 1) {
                        draw_vertical(im_d_row);
                    }
                } else {
                    //println!("Trace   | 上へ☆（＾～＾）");
                    for im_d_row in (1..(im_dy_len as i32 + 1)).rev() {
                        draw_vertical(im_long_edge_sign * im_d_row);
                    }
                }
            }
        } else {
            // 画像の外をクリックしていても無視します
        }
    }
}
