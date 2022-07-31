use super::image_processor_core::ImageData;
use super::image_processor_core::Image;
pub fn saturation_correction(src:ImageData<u8>,t:f32) -> ImageData<u8> {
  let mut hsla_data = src.to_hsla();
  let pixels = hsla_data.data.iter().enumerate();
  hsla_data.data = pixels.map(|x| {
      if x.0 & 3 == 1 {
        correction_function(*x.1,t)
        // *x.1
      } else {
        *x.1
      }
  }).collect::<Vec<f32>>();
  hsla_data.to_rgba()
}
// Note: ガンマ補正のがいいかも 普通の使い道的にはいい感じだけどね
fn correction_function(x:f32,t:f32) ->f32 {
  // let tmp = 1f32 - t;
  // 4f32*tmp*x*x*x -6f32*tmp*x*x+(3f32-2f32*t)*x
  let x = x / 100.;
  let x = if x<0.25 {
    t*x
  } else if x < 0.5 {
    (0.5-0.25*t)/0.25 * (x - 0.25) + t*0.25
  } else {
    x
  };
  x*100.
  // (x/100f32).powf(t) * 100f32
}

pub fn scaling_down(src:ImageData<u8>) -> ImageData<u8> {
  // 固定サイズ(横)
  const WIDTH:usize = 640;
  // 実縮小サイズ(横)
  const HARF_SCALE:usize = 320;
  // 画像データ
  let width = src.width as usize;
  let height = src.height as usize;
  // 画像縮小サイズの決定
  let scale = WIDTH as f64 / (2. * (width as f64));
  let scaled_height = (scale * (height as f64)) as usize;
  let scaled_height:usize = if (scaled_height & 1) == 1 {
    scaled_height - 1
  } else {
    scaled_height
  };
  // 横方向の縮小画像バッファ
  let mut hrzn = vec![0u8; HARF_SCALE * 3 * height];
  let mut i:usize = 0;
  // 1pxに対応する元画像の画素数
  let k_max = width as f64 / HARF_SCALE as f64;
  let k_max = (0..8).map(|x| {
    (k_max + 0.125 * x as f64) as usize
  }).collect::<Vec<usize>>();

  // 横方向の縮小
  while i<height {
    let mut j = 0;
    let mut k = 0;
    while j < HARF_SCALE {
      let mut sum_r = 0.;
      let mut sum_g = 0.;
      let mut sum_b = 0.;
      let mut s = 0.;
      let end = k + k_max[j & 7] * 4;
      // 対象範囲における色ごとの総和
      while k< end && k < (width * 4) {
        sum_r += src.data[i * width*4 + k] as f64;
        sum_g += src.data[i * width*4 + k + 1] as f64;
        sum_b += src.data[i * width*4 + k + 2] as f64;
        k+=4;
        s+=1.;
      }
      // 各色の平均
      hrzn[i * HARF_SCALE*3 + j*3    ] = (sum_r / s) as u8;
      hrzn[i * HARF_SCALE*3 + j*3 + 1] = (sum_g / s) as u8;
      hrzn[i * HARF_SCALE*3 + j*3 + 2] = (sum_b / s) as u8;
      j += 1;
    }
    i+=1;
  }
  // 縦方向に縮小した画像バッファ
  let mut vrtcl = Vec::<u8>::with_capacity((scaled_height * HARF_SCALE) << 2);
  i = 0;
  // 1pxに対応する画素数
  let k_max = (0..8).map(|x| {
    (1. / scale + 0.125 * x as f64)as usize
  }).collect::<Vec<usize>>();
  // 高さ方向の縮小
  let mut k = 0;
  while i < scaled_height {
    let end = k + k_max[i & 7];
    let k_tmp = k;
    let mut j:usize = 0;
    while j < HARF_SCALE {
      let mut sum_r = 0.;
      let mut sum_g = 0.;
      let mut sum_b = 0.;
      let mut s = 0.;
      k = k_tmp;
      // 対象範囲における色ごとの総和
      while k < end && k < height {
        sum_r += hrzn[k * HARF_SCALE*3 + j*3    ] as f64;
        sum_g += hrzn[k * HARF_SCALE*3 + j*3 + 1] as f64;
        sum_b += hrzn[k * HARF_SCALE*3 + j*3 + 2] as f64;
        s += 1.;
        k += 1;
      }
      vrtcl.push((sum_r / s) as u8);
      vrtcl.push((sum_g / s) as u8);
      vrtcl.push((sum_b / s) as u8);
      vrtcl.push(255u8);
      j += 1;
    }
    i += 1;
  }
  ImageData{
    width: HARF_SCALE,
    height: scaled_height,
    data: vrtcl,
    format: super::image_processor_core::Format::RGBA
  }
}
