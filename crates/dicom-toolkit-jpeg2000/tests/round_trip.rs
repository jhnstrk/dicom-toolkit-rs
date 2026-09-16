// Round trip encode / decode test for JPEG 2000 images

use dicom_toolkit_jpeg2000::{encode, encode_htj2k, DecodeSettings, EncodeOptions, Image};

use std::iter::zip;

struct ImageFixture {
    width: u32,
    height: u32,
    num_components: u8,
    bit_depth: u8,
    signed: bool,
    pixels: Vec<u8>,
}

fn make_fixture(width: u32, height: u32, num_components: u8) -> ImageFixture {
    let mut pixels = Vec::new();
    pixels.reserve(height as usize * width as usize * num_components as usize);
    for y in 0..height {
        for x in 0..width {
            for c in 0..num_components as u32 {
                pixels.push(((x * 20 + y * 7 + c * 33) & 0xff) as u8);
            }
        }
    }
    let bit_depth = 8;
    let signed = false;
    ImageFixture {
        width,
        height,
        num_components,
        bit_depth,
        signed,
        pixels,
    }
}

type EncodeFn = fn(&[u8], u32, u32, u8, u8, bool, &EncodeOptions) -> Result<Vec<u8>, &'static str>;

fn roundtrip(encode_fn: EncodeFn, num_components: u8) {
    let im = make_fixture(8, 8, num_components);

    let opts = EncodeOptions::default(); // reversible = true (lossless)
    let cs = 
        encode_fn(&im.pixels, im.width, im.height, im.num_components, im.bit_depth, im.signed, &opts).unwrap();
    let enc_image = Image::new(&cs, &DecodeSettings::default()).unwrap();
    let rt_pixels = enc_image.decode().unwrap();
    
    assert_eq!(im.width, enc_image.width());
    assert_eq!(im.height, enc_image.height());
    assert_eq!(im.num_components, enc_image.color_space().num_channels());

    let maxdiff = zip(&im.pixels, &rt_pixels).map(|(a, b)| (*a as i32 - *b as i32).abs()).max().unwrap();

    assert_eq!(maxdiff, 0);
    return;
}

macro_rules! make_rt_testcase {
    ($encode_fn:expr, $num_components:expr, $testname:ident) => {
        #[test]
        fn $testname() {
            let encode_fn = $encode_fn;
            roundtrip(encode_fn, $num_components);
        }
    }
}

make_rt_testcase!(encode, 3, roundtrip_j2k_rgb);
make_rt_testcase!(encode_htj2k, 3, roundtrip_htj2k_rgb);
make_rt_testcase!(encode, 1, roundtrip_j2k_gray);
make_rt_testcase!(encode_htj2k, 1, roundtrip_htj2k_gray);
