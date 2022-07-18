// 画像のデータ管理用構造体
pub struct ImageData {
  pub height:u32,
  pub width:u32,
  pub format:u8,
  pub data:Vec<u8>
}
// ファイル入出力
pub mod file_io {
  use std::fs::File;
  use png;
  use std::io::BufReader;
  use mime_guess;
  use std::path::Path;
  use std::io::BufWriter;
  // 画像読み込み
  pub fn file_load(file_path : &str) -> super::ImageData {
    let file = File::open(&file_path).expect("faild to open file");
    let guess = mime_guess::from_path(file_path);
    let mime_type = guess.first();
    if mime_type == Some(mime_guess::mime::IMAGE_JPEG) {
      println!("jpeg");
      let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(file));
      let pixels = decoder.decode().expect("faild to decode image");
      let metadata = decoder.info().unwrap();
      let mut px_format = 1;
      if metadata.pixel_format == jpeg_decoder::PixelFormat::RGB24 {
        px_format = 3;
      }
      return super::ImageData {
        height: metadata.height as u32,
        width: metadata.width as u32,
        format: px_format,
        data:pixels
      };
    } else if mime_type == Some(mime_guess::mime::IMAGE_PNG) {
      let decoder = png::Decoder::new(file);
      let mut reader = decoder.read_info().unwrap();
      let mut buf = vec![0; reader.output_buffer_size()];
      let info = reader.next_frame(&mut buf).unwrap();
      let bytes = &buf[..info.buffer_size()];
      let img_info = reader.info();
      let send_data = bytes.to_vec();
      let mut px_format = 1;
      if img_info.color_type ==  png::ColorType::Rgba {
        px_format = 4;
      } else if img_info.color_type ==  png::ColorType::Rgb {
        px_format = 3;
      }
      println!("png");
      return super::ImageData {
        height: img_info.height,
        width: img_info.width,
        format: px_format,
        data: send_data
      };
    } else {
      return super::ImageData {height:0, width:0, format:0, data:Vec::new()};
    }
  }
  // 画像出力
  pub fn file_save(file_path : &str, img_data :&super::ImageData) {
    let guess = mime_guess::from_path(&file_path);
    let mime_type = guess.first();
    if mime_type == Some(mime_guess::mime::IMAGE_JPEG) {
      let mut data = vec![0 as u8;(img_data.width * img_data.height * 3) as usize];
      let len = img_data.data.len();
      let mut i = 0;
      let mut j = 0;
      while i < len {
        data[j]     = img_data.data[i];
        data[j + 1] = img_data.data[i + 1];
        data[j + 2] = img_data.data[i + 2];
        i += 4;
        j += 3;
      }
      let encoder = jpeg_encoder::Encoder::new_file(&file_path, 100).unwrap();
      encoder.encode(&data, img_data.width as u16, img_data.height as u16, jpeg_encoder::ColorType::Rgb).unwrap();
    } else if  mime_type == Some(mime_guess::mime::IMAGE_PNG) {
      let path = Path::new(&file_path);
      let file = File::create(path).unwrap();
      let ref mut w = BufWriter::new(file);
      let mut encoder = png::Encoder::new(w, img_data.width, img_data.height);
      encoder.set_color(png::ColorType::Rgba);
      encoder.set_depth(png::BitDepth::Eight);
      let mut writer = encoder.write_header().unwrap();
      writer.write_image_data(&img_data.data).unwrap();
    }
  }

}
// 色空間、フォーマット変換
pub mod color_cvt {
  pub fn rgba2hsva(src: Vec<u8>) -> Vec<f32> {
    let len = src.len();
    let mut dest:Vec<f32> = vec![0.0; len];
    let mut i = 0;
    while i < len {
      let r:f32 = src[i]   as f32;
      let g:f32 = src[i+1] as f32;
      let b:f32 = src[i+2] as f32;
      let a:f32 = src[i+3] as f32;
      // 最小値
      let min:f32 = if r < g { r } else { g };
      let min = if min < b { min } else { b };
      // 最大値
      let max:f32 = if r > g { r } else { g };
      let max = if max > b { max } else { b };
      let sat = (max - min) / max;
      let sub_max_min = max - min;
      let hue = if min == max {
        0.0
      } else if min == b {
        60.0 * (g - r) / sub_max_min + 60.0
      } else if min == r {
        60.0 * (b - g) / sub_max_min + 180.0
      } else if min == g {
        60.0 * (r - b) / sub_max_min + 300.0
      } else {
        0.0
      };
      dest[i] = if hue < 0. {
        hue + 360.
      } else if hue > 360. {
        hue - 360.
      } else {
        hue
      };
      dest[i+1] = 100. * sat;
      dest[i+2] = 100. *  max / 255.;
      dest[i+3] = a;
      i+=4;
    }
    return dest;
  }
  pub fn hsva2rgba (src: Vec<f32>) -> Vec<u8> {
    let len = src.len();
    let mut dest:Vec<u8> = vec![0u8;len];
    let mut i = 0;
    while i < len {
      let h:f32 = src[i] / 60.0;
      let s:f32 = src[i+1] / 100.;
      let v:f32 = src[i+2] / 100.;
      let a:u8  = src[i+3] as u8;
      let c = v*s;
      let constant_num = v-c;
      let mut r = constant_num;
      let mut g = constant_num;
      let mut b = constant_num;
      let hp = h as u32;
      let hmod = h - 2. * ((h * 0.5) as i32) as f32;
      let hsub1 = hmod - 1.;
      let hsub1 = if hsub1 < 0. {
        -1. * hsub1
      } else {
        hsub1
      };
      let x:f32 = c * (1. - hsub1);
      if s > 0. {
        match hp {
          0 => {
            r += c;
            g += x;
          },
          1 => {
            r += x;
            g += c;
          },
          2 => {
            g += c;
            b += x;
          },
          3 => {
            g += x;
            b += c;
          },
          4 => {
            r += x;
            b += c;
          },
          5|6 => {
            r += c;
            b += x;
          },
          _ => {},
        }
      }
      dest[i]   = (r*255.0) as u8;
      dest[i+1] = (g*255.0) as u8;
      dest[i+2] = (b*255.0) as u8;
      dest[i+3] = a;
      i+=4;
    }
    return dest;
  }

  pub fn hsva2hsla (mut src:Vec<f32>) -> Vec<f32> {
    let len = src.len();
    let mut i = 0;
    while i < len {
      let h = src[i];
      let s = src[i+1] / 100.;
      let max = src[i+2] / 100.;
      let a = src[i+3];
      let l = max * (2. - s) / 2.;
      src[i] = h;
      src[i+1] = s * max * 100.;
      src[i+2] = l * 100.;
      src[i+3] = a;
      i += 4;
    }
    return src;
  }
  pub fn hsla2hsva (mut src:Vec<f32>) -> Vec<f32> {
    let len = src.len();
    let mut i = 0;
    while i < len {
      let h = src[i];
      let s = src[i+1] / 100.;
      let l = src[i+2] / 100.;
      let a = src[i+3];
      let v = (s + 2. * l) * 50.;
      src[i] = h;
      src[i+1] = s / v * 100.;
      src[i+2] = v;
      src[i+3] = a;
      i += 4;
    }
    return src;
  }

  pub fn rgba2rgb(src:Vec<u8>) -> Vec<u8> {
    let mut dest = vec![0u8; (src.len() >> 2) * 3];
    let mut i = 0;
    let mut j = 0;
    while i < src.len() {
      dest[j] = src[i];
      dest[j+1] = src[i+1];
      dest[j+2] = src[i+2];
      i += 4;
      j += 3;
    }
    return dest;
  }

  pub fn rgb2rgba(src:Vec<u8>) -> Vec<u8> {
    let mut dest = vec![255u8; (src.len() / 3) << 2];
    let mut i = 0;
    let mut j = 0;
    while j < src.len() {
      dest[i] = src[j];
      dest[i+1] = src[j+1];
      dest[i+2] = src[j+2];
      i += 4;
      j += 3;
    }
    return dest;
  }

  pub fn rgba2hsla(src:Vec<u8>) -> Vec<f32>{
    hsva2hsla(rgba2hsva(src))
  }

  pub fn hsla2rgba(src:Vec<f32>) -> Vec<u8>{
    hsva2rgba(hsla2hsva(src))
  }
}
