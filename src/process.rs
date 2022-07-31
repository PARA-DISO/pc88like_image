use super::image_processor_core::{ImageData,Image,Format};
use super::test_mod::{scaling_down,saturation_correction};
// use array_macro::array;
pub fn pc88_like(
  img_data: ImageData<u8>,
  gamma: f32
) -> ImageData<u8> {
  const WIDTH:usize = 640;
  // pc88 カラーパレット
  const COLOR_PALLET:[[u8; 3]; 8] = [
    [255, 0,   0],   // red
    [255, 255, 0],   // yellow
    [0,   255, 0],   // green
    [0,   255, 255], // light blue
    [0,   0,   255], // blue
    [255, 0,   255], // purple
    [0,   0,   0],   // black
    [255, 255, 255]  // white
  ];
  // 画像を縮小し、彩度に鮮やかになるよう補正
  let scaled_image = saturation_correction(scaling_down(img_data),gamma);
  let thresh = ootu_method(scaled_image.to_gray_scale());
  println!("{}", thresh);
  let scaled_height = scaled_image.height;
  // 画像バッファ(高さ1/2)
  let mut replaced_data = vec![255u8; WIDTH * scaled_height<<2];
  // 画素データの決定
  let scaled_image = scaled_image.data;
  for i in 0..scaled_height {
    let idx_src = i*320<<2;
    let idx_dst = i*WIDTH << 2;
    for j in 0..320 {
      let idx_src = idx_src + (j<<2);
      let r = scaled_image[idx_src] as i32;
      let g = scaled_image[idx_src+1] as i32;
      let b = scaled_image[idx_src+2] as i32;
      // 各データの距離を計算
      let r_lens = [
        r*r,(thresh-r)*(thresh-r),(255-r)*(255-r)
      ];
      let g_lens = [
        g*g,(thresh-g)*(thresh-g),(255-g)*(255-g)
      ];
      let b_lens = [
        b*b,(thresh-b)*(thresh-b),(255-b)*(255-b)
      ];

      // 27色との距離を計算
      let lens = [
        r_lens[0] + g_lens[0] + b_lens[0], // black     + black
        r_lens[0] + g_lens[0] + b_lens[1], // blue      + black
        r_lens[0] + g_lens[0] + b_lens[2], // blue      + blue
        r_lens[0] + g_lens[1] + b_lens[0], // green     + black
        r_lens[0] + g_lens[1] + b_lens[1], // lightblue + black
        r_lens[0] + g_lens[1] + b_lens[2], // lightblue + blue
        r_lens[0] + g_lens[2] + b_lens[0], // green     + green
        r_lens[0] + g_lens[2] + b_lens[1], // green     + lightblue
        r_lens[0] + g_lens[2] + b_lens[2], // lightblue + lightblue
        r_lens[1] + g_lens[0] + b_lens[0], // red       + black
        r_lens[1] + g_lens[0] + b_lens[1], // purple    + black
        r_lens[1] + g_lens[0] + b_lens[2], // purple    + blue
        r_lens[1] + g_lens[1] + b_lens[0], // yellow    + black
        r_lens[1] + g_lens[1] + b_lens[1], // black     + white
        r_lens[1] + g_lens[1] + b_lens[2], // blue      + white
        r_lens[1] + g_lens[2] + b_lens[0], // yellow    + green
        r_lens[1] + g_lens[2] + b_lens[1], // green     + white
        r_lens[1] + g_lens[2] + b_lens[2], // lightblue + white
        r_lens[2] + g_lens[0] + b_lens[0], // red       + red
        r_lens[2] + g_lens[0] + b_lens[1], // purple    + red
        r_lens[2] + g_lens[0] + b_lens[2], // purple    + purple
        r_lens[2] + g_lens[1] + b_lens[0], // red       + yellow
        r_lens[2] + g_lens[1] + b_lens[1], // red       + white
        r_lens[2] + g_lens[1] + b_lens[2], // purple    + white
        r_lens[2] + g_lens[2] + b_lens[0], // yellow    + yellow
        r_lens[2] + g_lens[2] + b_lens[1], // yellow    + white
        r_lens[2] + g_lens[2] + b_lens[2], // white     + white
      ];
      // 最も近い色の決定
      let mut min = lens[0];
      let mut idx = 0;
      for k in 1..27 {
        if min > lens[k] {
          idx = k;
          min = lens[k];
        }
      }
      let pallet_num = match idx {
        // grayScale
        0 => (6,6),
        1 => (4,6),
        2 => (4,4),

        3 => (2,6),
        4 => (3,6),
        5 => (3,4),

        6 => (2,2),
        7 => (2,3),
        8 => (3,3),

        9 => (0,6),
        10 => (5,6),
        11 => (5,4),

        12 => (1,6),
        13 => (6,7),
        14 => (4,7),

        15 => (1,2),
        16 => (2,7),
        17 => (3,7),

        18 => (0,0),
        19 => (5,0),
        20 => (5,5),

        21 => (0,1),
        22 => (0,7),
        23 => (5,7),

        24 => (1,1),
        25 => (1,7),
        26 => (7,7),
        _ => {println!("{}", idx);unreachable!()}
      };
      //
      let pallet_num = if i&1==1 {
          (pallet_num.1,pallet_num.0)
      } else {
        pallet_num
      };
      [
        replaced_data[idx_dst+j*8],
        replaced_data[idx_dst+j*8+1],
        replaced_data[idx_dst+j*8+2]
      ] = COLOR_PALLET[pallet_num.0];
      [
        replaced_data[idx_dst+(2*j+1)*4],
        replaced_data[idx_dst+(2*j+1)*4+1],
        replaced_data[idx_dst+(2*j+1)*4+2]
      ] = COLOR_PALLET[pallet_num.1];
    }
  }

  // 高さ方向を倍に拡大
  let display_height = scaled_height + scaled_height;
  let mut dest = vec![255u8; display_height * WIDTH <<2];
  let mut i = 0;
  while i<scaled_height {
    let mut j = 0;
    while j < 4*WIDTH {
      dest[8*i * WIDTH + j    ] = replaced_data[i * 4*WIDTH + j];
      dest[8*i * WIDTH + j + 1] = replaced_data[i * 4*WIDTH + j + 1];
      dest[8*i * WIDTH + j + 2] = replaced_data[i * 4*WIDTH + j + 2];

      dest[(2*i + 1) * 4*WIDTH + j    ] = replaced_data[i * 4*WIDTH + j];
      dest[(2*i + 1) * 4*WIDTH + j + 1] = replaced_data[i * 4*WIDTH + j + 1];
      dest[(2*i + 1) * 4*WIDTH + j + 2] = replaced_data[i * 4*WIDTH + j + 2];
      j += 4;
    }
    i += 1;
  }
  ImageData {
    height:display_height,
    width: WIDTH,
    format: Format::RGBA,
    data:dest
  }
}
fn ootu_method(src:ImageData<u8> /* gray scale */) -> i32 {
  let mut hist = vec![0usize;256];
  src.data.iter().for_each(|x| {
    hist[*x as usize]+=1;
  });
  // 大津の二値化法による閾値?計算
  // ヒストグラムからデータを生成
  // 画素数の積分データ
  let mut int_px_hist = vec![0usize;256];
  // 輝度値の重みづけ積分データ
  let mut int_weighting = vec![0usize;256];
  int_px_hist[0] = hist[0];
  int_weighting[0] = hist[0];
  for i in 1..256 {
    int_px_hist[i] = int_px_hist[i-1]+ hist[i];
    int_weighting[i] = int_weighting[i-1] + (i+1)*hist[i];
  }
  // 閾値計算
  const CALC_MAX:usize = 255usize;
  let mut thresh = 0i32;
  let mut max = 0f64;
  for i in 0..CALC_MAX {
    let w1 = int_px_hist[i];
    let w2 = int_px_hist[CALC_MAX-1] - w1;
    // キャストして再定義
    let w1 = w1 as f64;
    let w2 = w2 as f64;
    // クラスごとの輝度値の総和
    let sum1 = int_weighting[i];
    let sum2 = int_weighting[CALC_MAX-1] - sum1;
    // 平均値
    let m1 = if w1 != 0. {
      sum1 as f64 / w1
    } else {
      0f64
    };
    let m2 = if w2 != 0. {
      sum2 as f64 / w2
    } else {
      0f64
    };
    let temp = w1 * w2 * (m1-m2) * (m1-m2);
    // print!("{},", temp);
    if temp > max {
      max = temp;
      thresh = i as i32;
    }
  }
  return thresh
}
