use super::ImageData;
use super::Image;
use super::Format;

// GRAYSCALE/RGB/RGBA用
impl Image for ImageData<u8> {
  fn to_gray_scale(&self) -> ImageData<u8> {
    let len = self.height*self.width;
    ImageData::<u8> {
      height: self.height,
      width: self.width,
      format: Format::GRAYSCALE,
      data: match self.format {
        Format::GRAYSCALE => self.data.clone(),
        Format::RGBA => rgba2gray_scale(&self.data),
        Format::RGB => {
          let mut dest = Vec::with_capacity(len);
          for i in 0..len {
            let idx = i+i+i;
            dest.push((
                0.2126*self.data[idx] as f64
                + 0.7152*self.data[idx+1] as f64
                + 0.0722*self.data[idx+2] as f64
            ) as u8)
          }
          dest
        },
        _ => unreachable!()
      }
    }
  }
  fn to_rgb(&self) -> ImageData<u8>{
    let len = self.height*self.width;
    ImageData::<u8> {
      height: self.height,
      width: self.width,
      format: Format::RGB,
      data: match self.format {
        Format::GRAYSCALE => {
          self.data.iter().map(|x|
            [*x,*x,*x]
          ).flatten().collect::<Vec<u8>>()
        },
        Format::RGB => self.data.clone(),
        Format::RGBA => {
          let mut dest = Vec::with_capacity(len+len+len);
          for i in 0..len {
            let idx = i<<2;
            dest.push(self.data[idx]);
            dest.push(self.data[idx+1]);
            dest.push(self.data[idx+2]);
          }
          dest
        },
        _ => unreachable!()
      }
    }
  }
  fn to_rgba(&self) -> ImageData<u8> {
    let len = self.height*self.width;
    ImageData::<u8> {
      height: self.height,
      width: self.width,
      format: Format::RGBA,
      data: match self.format {
        Format::GRAYSCALE => {
          self.data.iter().map(|x|
            [*x,*x,*x,255u8]
          ).flatten().collect::<Vec<u8>>()
        },
        Format::RGB => {
          let mut dest = Vec::with_capacity(len<<2);
          for i in 0..len {
            let idx = i+i+i;
            dest.push(self.data[idx]);
            dest.push(self.data[idx+1]);
            dest.push(self.data[idx+2]);
            dest.push(255u8);
          }
          dest
        },
        Format::RGBA => self.data.clone(),
        _ => unreachable!()
      }
    }
  }
  fn to_hsva(&self) ->ImageData<f32> {
    ImageData::<f32> {
      height: self.height,
      width: self.width,
      format: Format::HSVA,
      data: match self.format {
        Format::RGBA => rgba2hsva(&self.data),
        Format::GRAYSCALE => rgba2hsva(
          &self.data.iter().map(|x| [*x,*x,*x,255u8]).flatten().collect::<Vec<u8>>()
        ),
        Format::RGB => rgba2hsva(&self.to_rgba().data),
        _ => unreachable!(),
      }
    }
  }
  fn to_hsla(&self) -> ImageData<f32>{
    ImageData::<f32> {
      height: self.height,
      width: self.width,
      format: Format::HSLA,
      data: match self.format {
        Format::RGBA => hsva2hsla(rgba2hsva(&self.data)),
        Format::GRAYSCALE => hsva2hsla(rgba2hsva(
          &self.data.iter().map(|x| [*x,*x,*x,255u8]).flatten().collect::<Vec<u8>>()
        )),
        Format::RGB => hsva2hsla(rgba2hsva(&self.to_rgba().data)),
        _ => unreachable!(),
      }
    }
  }
}
impl Image for ImageData<f32> {
  fn to_gray_scale(&self) -> ImageData<u8> {
    ImageData::<u8>{
      height: self.height,
      width: self.width,
      format: Format::GRAYSCALE,
      data: match self.format {
        Format::HSVA => rgba2gray_scale(&hsva2rgba(&self.data)),
        Format::HSLA => rgba2gray_scale(&hsva2rgba(&hsla2hsva(self.data.clone()))),
        _ => unreachable!(),
      }
    }
  }
  fn to_rgb(&self) -> ImageData<u8> {
    ImageData::<u8>{
      height: self.height,
      width: self.width,
      format: Format::RGB,
      data: match self.format {
        Format::HSVA => (&hsva2rgba(&self.data)).iter().enumerate().filter(|x|{
          x.0 == 0 || x.0 & 3 != 0
        }).map(|x| *x.1).collect::<Vec<u8>>(),
        Format::HSLA => (&hsva2rgba(&hsla2hsva(self.data.clone()))).iter().enumerate().filter(|x|{
          x.0 == 0 || x.0 & 3 != 0
        }).map(|x| *x.1).collect::<Vec<u8>>(),
        _ => unreachable!(),
      }
    }
  }
  fn to_rgba(&self) -> ImageData<u8> {
    ImageData::<u8>{
      height: self.height,
      width: self.width,
      format: Format::RGBA,
      data: match self.format {
        Format::HSVA => hsva2rgba(&self.data),
        Format::HSLA => hsva2rgba(&hsla2hsva(self.data.clone())),
        _ => unreachable!(),
      }
    }
  }
 fn to_hsva(&self) -> ImageData<f32> {
   match self.format {
     Format::HSVA => {
      self.clone()
     },
     Format::HSLA|_ => {
       ImageData::<f32> {
         height: self.height,
         width: self.width,
         format: Format::HSVA,
         data: hsla2hsva(self.data.clone())
       }
     }
   }
 }
 fn to_hsla(&self) -> ImageData<f32> {
   match self.format {
     Format::HSLA => {
      self.clone()
     },
     Format::HSVA|_ => {
       ImageData::<f32> {
         height: self.height,
         width: self.width,
         format: Format::HSLA,
         data: hsva2hsla(self.data.clone())
       }
     }
   }
 }
}
// u8 <-> f32 用
fn rgba2hsva(src: &[u8]) -> Vec<f32> {
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
fn hsva2rgba (src: &[f32]) -> Vec<u8> {
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
// 相互変換
fn hsva2hsla (mut src:Vec<f32>) -> Vec<f32> {
  let len = src.len();
  let mut i = 0;
  while i < len {
    let h = src[i];
    let s = src[i+1] / 100.;
    let max = src[i+2] / 100.;
    let a = src[i+3];
    // let  = v;
    let min = max * (1. - s);
    src[i] = h;
    src[i+1] = (max - min) * 100.;
    src[i+2] = (max + min) * 50.;
    src[i+3] = a;
    i += 4;
  }
  return src;
}
fn hsla2hsva (mut src:Vec<f32>) -> Vec<f32> {
 let len = src.len();
 let mut i = 0;
 while i < len {
   let h = src[i];
   let s = src[i+1] / 100.;
   let l = src[i+2] / 100.;
   let a = src[i+3];
   let max = 0.5 * (s + l + l);
   let min = 0.5 * (l + l - s);
   src[i  ] = h;
   src[i+1] = (max - min) / max * 100.;
   src[i+2] = max * 100.;
   src[i+3] = a;
   i += 4;
 }
 return src;
}
fn rgba2gray_scale(src:&[u8]) -> Vec<u8> {
  let len = src.len() >> 2;
  let mut dest = Vec::with_capacity(len);
  for i in 0..len {
    let idx = i<<2;
    dest.push((
        0.2126*src[idx] as f64
        + 0.7152*src[idx+1] as f64
        + 0.0722*src[idx+2] as f64
    ) as u8)
  }
  dest
}
