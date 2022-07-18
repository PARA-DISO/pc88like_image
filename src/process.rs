use super::img_processor_core::ImageData;
use array_macro::array;
pub fn pc88_like_means(
  img_data: &ImageData,gamma:f64
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
  let gamma_collections = (0..256).map(|x| ((x as f64 / 255f64).powf(gamma) * 255f64) as i32).collect::<Vec<i32>>();

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
  // let k_max = [(1. / scale + 0.5) as usize, (1. / scale) as usize];
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
      // 各色の平均を求める
      vrtcl[i * HARF_SCALE + j    ] = [
        gamma_collections[(sum_r / s) as usize],
        gamma_collections[(sum_g / s) as usize],
        gamma_collections[(sum_b / s) as usize]
      ];
      j += 1;
    }
    i += 1;
  }
  // HSL変換
  // let hsl_data = color_cvt::rgba2hsla(vrtcl);
  // 画像バッファ(高さ1/2)
  let mut replaced_data = vec![255u8; WIDTH * scaled_height * 4];
  i = 0;
  let mut j:usize = 0;
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
      let r_lens = [
        r*r,(128-r)*(128-r),(255-r)*(255-r)
      ];
      let g_lens = [
        g*g,(128-g)*(128-g),(255-g)*(255-g)
      ];
      let b_lens = [
        b*b,(128-b)*(128-b),(255-b)*(255-b)
      ];
      let r = if r_lens[0] < r_lens[1] && r_lens[0] < r_lens[2] {
        0
      } else if r_lens[1] < r_lens[2] {
        1
      } else {
        2
      };
      let g = if g_lens[0] < g_lens[1] && g_lens[0] < g_lens[2] {
        0
      } else if g_lens[1] < g_lens[2] {
        1
      } else {
        2
      };
      let b = if b_lens[0] < b_lens[1] && b_lens[0] < b_lens[2] {
        0
      } else if b_lens[1] < b_lens[2] {
        1
      } else {
        2
      };
      let color_code = b+(g<<2)+(r<<4);
      let mut pallet_num = match color_code {
        // grayScale
        0b00_00_00 => (6,6),
        0b10_10_10 => (7,7),
        0b01_01_01 => (6,7),
        //
        0b10_00_00 => (0,0),
        0b10_10_00 => (1,1),
        0b00_10_00 => (2,2),
        0b00_10_10 => (3,3),
        0b00_00_10 => (4,4),
        0b10_00_10 => (5,5),
        // whitish
        0b10_01_01 => (0,7),
        0b10_10_01 => (1,7),
        0b01_10_01 => (2,7),
        0b01_10_10 => (3,7),
        0b01_01_10 => (4,7),
        0b10_01_10 => (5,7),
        // blackish
        0b01_00_00 => (0,6),
        0b01_01_00 => (1,6),
        0b00_01_00 => (2,6),
        0b00_01_01 => (3,6),
        0b00_00_01 => (4,6),
        0b01_00_01 => (5,6),
        //
        0b10_01_00 => (0,1),
        0b01_10_00 => (1,2),
        0b00_10_01 => (2,3),
        0b00_01_10 => (3,4),
        0b01_00_10 => (4,5),
        0b10_00_01 => (5,0),
        _ => {println!("{}", color_code);unreachable!()}
      };
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
