pub fn rgba2gray_scale(src:Vec<u8>) -> Vec<u8> {
  let len = src.len();
  let mut dest = vec![0u8;len>>2];
  let mut i = 0;
  let mut j = 0;
  while i < len {
    let r = src[i] as f64;
    let g = src[i+1] as f64;
    let b = src[i+2] as f64;
    dest[j] = (0.2126*r + 0.7152*g + 0.0722*b) as u8;
    i+=4;
    j+=1;
  }
  dest
}
