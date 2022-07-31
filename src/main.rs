
use std::env;

mod image_processor_core;
use image_processor_core::{file_io,Image};
mod process;
// 画像読み込みの構造体
mod test_mod;
fn main() {
    // コマンドライン引数を受け取る
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let export_fname = &args[2];
    let gamma:f32 = args[3].parse().unwrap();
    let gamma:f32 = 1. + gamma/ 10.;
    // 画像読み込み
    let image_data = file_io::file_load(fname);
    // RGBAフォーマットに変換
    // let image_data = image_data.to_hsva();
    let image_data = image_data.to_rgba();
    // let image_data = test_mod::saturation_correction(image_data,0.45);
    // let image_data = process::test_function(&image_data);
    let image_data = process::pc88_like(image_data,gamma);
    // // 画像の出力
    file_io::file_save(export_fname, &image_data);

}

// PC8801風の画像に変換する関数
// fn pc88_like(
//   img_data: &ImageData,
//   gamma:[f32;2]
// ) -> ImageData {
//   // 固定サイズ(横)
//   const WIDTH:usize = 640;
//   // 実縮小サイズ(横)
//   const HARF_SCALE:usize = 320;
//   // 画像データ
//   let width = img_data.width as usize;
//   let height = img_data.height as usize;
//   // 画像縮小サイズの決定
//   let scale = WIDTH as f64 / (2. * (width as f64));
//   let scaled_height = (scale * (height as f64)) as usize;
//   let scaled_height:usize = if (scaled_height & 1) == 1 {
//     scaled_height - 1
//   } else {
//     scaled_height
//   };
//   // 横方向の縮小画像バッファ
//   let mut hrzn = vec![0u8; HARF_SCALE * 3 * height];
//   let mut i:usize = 0;
//   // 1pxに対応する元画像の画素数
//   let k_max = width as f64 / HARF_SCALE as f64;
//   let k_max = (0..8).map(|x| {
//     (k_max + 0.125 * x as f64) as usize
//   }).collect::<Vec<usize>>();
//   // let k_max = vec![k_max as usize, (k_max + 0.5) as usize];
//   // 横方向の縮小
//   while i<height {
//     let mut j = 0;
//     let mut k = 0;
//     while j < HARF_SCALE {
//       let mut sum_r = 0.;
//       let mut sum_g = 0.;
//       let mut sum_b = 0.;
//       let mut s = 0.;
//       let end = k + k_max[j & 7] * 4;
//       // 対象範囲における色ごとの総和
//       while k< end && k < (width * 4) {
//         sum_r += img_data.data[i * width*4 + k] as f64;
//         sum_g += img_data.data[i * width*4 + k + 1] as f64;
//         sum_b += img_data.data[i * width*4 + k + 2] as f64;
//         k+=4;
//         s+=1.;
//       }
//       // 各色の平均
//       hrzn[i * HARF_SCALE*3 + j*3    ] = (sum_r / s) as u8;
//       hrzn[i * HARF_SCALE*3 + j*3 + 1] = (sum_g / s) as u8;
//       hrzn[i * HARF_SCALE*3 + j*3 + 2] = (sum_b / s) as u8;
//       j += 1;
//     }
//     i+=1;
//   }
//   // 縦方向に縮小した画像バッファ
//   let mut vrtcl = vec![255u8; scaled_height * HARF_SCALE * 4];
//   i = 0;
//   // 1pxに対応する画素数
//   let k_max = (0..8).map(|x| {
//     (1. / scale + 0.125 * x as f64)as usize
//   }).collect::<Vec<usize>>();
//   // let k_max = [(1. / scale + 0.5) as usize, (1. / scale) as usize];
//   // 高さ方向の縮小
//   let mut k = 0;
//   while i < scaled_height {
//     let end = k + k_max[i & 7];
//     let k_tmp = k;
//     let mut j:usize = 0;
//     while j < HARF_SCALE {
//       let mut sum_r = 0.;
//       let mut sum_g = 0.;
//       let mut sum_b = 0.;
//       let mut s = 0.;
//       k = k_tmp;
//       // 対象範囲における色ごとの総和
//       while k < end && k < height {
//         sum_r += hrzn[k * HARF_SCALE*3 + j*3    ] as f64;
//         sum_g += hrzn[k * HARF_SCALE*3 + j*3 + 1] as f64;
//         sum_b += hrzn[k * HARF_SCALE*3 + j*3 + 2] as f64;
//         s += 1.;
//         k += 1;
//       }
//       // 各色の平均を求める
//       vrtcl[i * HARF_SCALE*4 + j*4    ] = (sum_r / s) as u8;
//       vrtcl[i * HARF_SCALE*4 + j*4 + 1] = (sum_g / s) as u8;
//       vrtcl[i * HARF_SCALE*4 + j*4 + 2] = (sum_b / s) as u8;
//       j += 1;
//     }
//     i += 1;
//   }
//   // HSL変換
//   let hsl_data = color_cvt::rgba2hsla(vrtcl);
//   // 画像バッファ(高さ1/2)
//   let mut replaced_data = vec![255u8; WIDTH * scaled_height * 4];
//   i = 0;
//   let mut j:usize = 0;
//   // カラーパレット
//   const COLOR_PALLET:[[u8; 3]; 8] = [
//     [255, 0,   0],   // red
//     [255, 255, 0],   // yellow
//     [0,   255, 0],   // green
//     [0,   255, 255], // light blue
//     [0,   0,   255], // blue
//     [255, 0,   255], // purple
//     [0,   0,   0],   // black
//     [255, 255, 255]  // white
//   ];
//   // 画素データの決定
//   while i < hsl_data.len() {
//     let h = hsl_data[i as usize];
//     // 彩度を正規化
//     let s = hsl_data[(i + 1) as usize] / 100.;
//     // 彩度を0,1に変換
//     let s = (s.powf(gamma[0]) + 0.5) as u8;
//     // 明度を正規化
//     let l = hsl_data[(i + 2) as usize] / 100.;
//     // 明度を0,1,2,3,4or5に変換
//     let l_quartile = (l.powf(gamma[1]) * 5.) as u8;
//     // 色を2色決定
//     let (cm, cs) = calc_color(h);
//     // 明度に応じて出力する色を決定
//     let (main_color, sub_color) : (usize, usize) = match l_quartile {
//       0 => (6, 6),
//       1 => {
//           // 暗いほうの色を採用
//           if cm & 1 == 0 {
//             (cm, 6)
//           } else {
//             (cs, 6)
//           }
//         },
//       2 => {
//           if s == 0 {
//             (6, 7)
//           } else {
//             (cm, cs)
//           }
//         },
//       3 => {
//           // 明るいほうの色を採用
//           if cm & 1 == 1 {
//             (cm, 7)
//           } else {
//             (cs, 7)
//           }
//         },
//       _ => (7, 7)
//     };
//     // 色の配置場所を奇数、偶数行目で変える
//     let (main_color, sub_color):(usize, usize) = match (i / (WIDTH*4)) & 1 {
//       0 => (main_color, sub_color),
//       _ => (sub_color, main_color)
//     };
//     // 対応するRGBデータを2px分代入
//     [
//       replaced_data[j],
//       replaced_data[j+1],
//       replaced_data[j+2]
//     ] = COLOR_PALLET[main_color];
//     j += 4;
//     [
//       replaced_data[j],
//       replaced_data[j+1],
//       replaced_data[j+2]
//     ] = COLOR_PALLET[sub_color];
//     i += 4;
//     j += 4;
//   }
//   // 高さ方向を倍に拡大
//   let display_height = scaled_height * 2;
//   let mut dest = vec![255u8; display_height * WIDTH * 4];
//   i = 0;
//   while i<scaled_height {
//     let mut j = 0;
//     while j < 4*WIDTH {
//       dest[8*i * WIDTH + j    ] = replaced_data[i * 4*WIDTH + j];
//       dest[8*i * WIDTH + j + 1] = replaced_data[i * 4*WIDTH + j + 1];
//       dest[8*i * WIDTH + j + 2] = replaced_data[i * 4*WIDTH + j + 2];
//
//       dest[(2*i + 1) * 4*WIDTH + j    ] = replaced_data[i * 4*WIDTH + j];
//       dest[(2*i + 1) * 4*WIDTH + j + 1] = replaced_data[i * 4*WIDTH + j + 1];
//       dest[(2*i + 1) * 4*WIDTH + j + 2] = replaced_data[i * 4*WIDTH + j + 2];
//       j += 4;
//     }
//     i += 1;
//   }
//   ImageData {
//     height:display_height as u32,
//     width: WIDTH as u32,
//     format: 4,
//     data:dest
//   }
// }
// // 色の決定関数
// fn calc_color(hue:f32) -> (usize, usize) {
//   // 12色表現
//   let h_dt = (hue / 30.) as usize;
//   // 6色表現
//   let h = (hue / 60.) as usize;
//   let sub_color = h_dt - h;
//   // 色の決定
//   if sub_color > 5 {
//     (h, sub_color - 6)
//   } else {
//     (h, sub_color)
//   }
// }
