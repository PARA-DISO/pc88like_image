#[derive(Clone, Debug)]
pub enum Format {
  RGB,
  RGBA,
  HSLA,
  HSVA,
  GRAYSCALE,
}
#[derive(Clone, Debug)]
pub struct ImageData<T> {
  pub height:usize,
  pub width:usize,
  pub format:Format,
  pub data:Vec<T>
}

pub trait Image {
  fn to_gray_scale(&self) -> ImageData<u8>;
  fn to_rgb(&self) -> ImageData<u8>;
  fn to_rgba(&self) -> ImageData<u8>;
  fn to_hsva(&self) -> ImageData<f32>;
  fn to_hsla(&self) -> ImageData<f32>;
}
mod color_cvt;
pub mod file_io {
  use std::fs::File;
  use png;
  use std::io::BufReader;
  use mime_guess;
  use std::path::Path;
  use std::io::BufWriter;
  use super::Format;
  // 画像読み込み
  pub fn file_load(file_path :&str) -> super::ImageData<u8> {
    let file = File::open(&file_path).expect("faild to open file");
    let guess = mime_guess::from_path(file_path);
    let mime_type = guess.first();
    if mime_type == Some(mime_guess::mime::IMAGE_JPEG) {
      println!("jpeg");
      let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(file));
      let pixels = decoder.decode().expect("faild to decode image");
      let metadata = decoder.info().unwrap();
      let mut px_format = Format::GRAYSCALE;
      if metadata.pixel_format == jpeg_decoder::PixelFormat::RGB24 {
        px_format = Format::RGB;
      }
      return super::ImageData {
        height: metadata.height as usize,
        width: metadata.width as usize,
        format: px_format,
        data: pixels
      };
    } else if mime_type == Some(mime_guess::mime::IMAGE_PNG) {
      let decoder = png::Decoder::new(file);
      let mut reader = decoder.read_info().unwrap();
      let mut buf = vec![0; reader.output_buffer_size()];
      let info = reader.next_frame(&mut buf).unwrap();
      let bytes = &buf[..info.buffer_size()];
      let img_info = reader.info();
      let send_data = bytes.to_vec();
      let mut px_format = Format::GRAYSCALE;
      if img_info.color_type ==  png::ColorType::Rgba {
        px_format = Format::RGBA;
      } else if img_info.color_type ==  png::ColorType::Rgb {
        px_format = Format::RGB;
      }
      println!("png");
      return super::ImageData {
        height: img_info.height as usize,
        width: img_info.width as usize,
        format: px_format,
        data: send_data
      };
    } else {
      return super::ImageData {height:0, width:0, format: Format::RGB, data:Vec::new()};
    }
  }
  // 画像出力
  pub fn file_save(file_path :&str, img_data :&super::ImageData<u8>) {
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
      let mut encoder = png::Encoder::new(w, img_data.width as u32, img_data.height as u32);
      encoder.set_color(png::ColorType::Rgba);
      encoder.set_depth(png::BitDepth::Eight);
      let mut writer = encoder.write_header().unwrap();
      writer.write_image_data(&img_data.data).unwrap();
    }
  }
}
