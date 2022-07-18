use super::img_processor_core::ImageData;
use array_macro::array;
pub fn pc88_like_means(
  img_data: &ImageData
) -> ImageData {
  // 固定サイズ(横)
  const WIDTH:usize = 640;
  // 実縮小サイズ(横)
  const HARF_SCALE:usize = 320;
  // 画像データ
  let width = img_data.width as usize;
  let height = img_data.height as usize;
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
        sum_r += img_data.data[i * width*4 + k] as f64;
        sum_g += img_data.data[i * width*4 + k + 1] as f64;
        sum_b += img_data.data[i * width*4 + k + 2] as f64;
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
  let mut vrtcl = vec![[0i32,0i32,0i32]; scaled_height * HARF_SCALE];
  i = 0;
  // 1pxに対応する画素数
  let k_max = (0..8).map(|x| {
    (1. / scale + 0.125 * x as f64)as usize
  }).collect::<Vec<usize>>();
  let mut hist = [0usize;256];
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
      let px = [(sum_r / s) as i32,(sum_g / s) as i32,(sum_b / s) as i32];
      // 各色の平均を求める
      vrtcl[i * HARF_SCALE + j] = px.clone();
      // ヒストグラム(輝度)の作成
      let gray =( 0.2125 * px[0] as f64 +  0.7154*px[1] as f64 + 0.0721*px[2] as f64) as usize;
      hist[gray]+=1;
      j += 1;
    }
    i += 1;
  }
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
  println!("{}", thresh);

  // 画像バッファ(高さ1/2)
  let mut replaced_data = vec![255u8; WIDTH * scaled_height * 4];
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
  // 画素データの決定
  for i in 0..scaled_height {
    let idx_src = i*320;
    let idx_dst = i*640*4;
    for j in 0..320 {
      let [r,g,b] = vrtcl[idx_src+j];
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
  let display_height = scaled_height * 2;
  let mut dest = vec![255u8; display_height * WIDTH * 4];
  i = 0;
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
    height:display_height as u32,
    width: WIDTH as u32,
    format: 4,
    data:dest
  }
}
