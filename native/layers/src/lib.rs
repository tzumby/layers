use image::{DynamicImage, ImageFormat};
use image::imageops::overlay;
use rustler::{ResourceArc, Atom, Binary, Env, Error, Term, NifStruct, OwnedBinary};
use std::io::Cursor;
use std::io::Write as _;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        png,
        jpg,
        gif,
        unsupported_image_format
    }
}

#[derive(NifStruct)]
#[module = "Layer"]
struct Layer {
    byte_size: usize,
    format: Atom,
    data: ResourceArc<Image>,
}

struct Image {
    image: DynamicImage,
    format: ImageFormat
}

#[rustler::nif(schedule = "DirtyCpu")]
fn from_bytes(binary: Binary) -> Result<(Atom, Layer), Error> {
    match image::load_from_memory(binary.as_slice()) {
        Ok(image) => {
            if let Ok(format) = image::guess_format(&binary.as_slice()) {
                let layer = Layer {
                    byte_size: binary.len(),
                    format: image_format(format),
                    data: ResourceArc::new( Image { image, format })
                };

                return Ok((atoms::ok(), layer ));
            }
            return Err(Error::Atom("unsupported_image_format"));
        }
        Err(_) => Err(Error::BadArg),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn layer_images(
    env: Env, 
    background: ResourceArc<Image>, 
    foreground: ResourceArc<Image>,
    x: i64,
    y: i64 
    ) -> Result<(Atom, Binary, Layer), Error> {
    let mut background_image = background.image.clone();
    let foreground_image = foreground.image.clone();

    overlay(&mut background_image, &foreground_image, x, y);

    let mut output = Vec::new();
    let mut binary = OwnedBinary::new(background_image.as_bytes().len()).unwrap();

    match background_image.write_to(&mut Cursor::new(&mut output), background.format) {
        Ok(_) => {
            binary
                .as_mut_slice()
                .write_all(&output)
                .map_err(|_| Error::Atom("io_error"))?;
            let format = image_format(background.format);
            let bytes = binary.release(env);
            let byte_size = bytes.as_slice().len();

            let layer = Layer {
                byte_size,
                format,
                data: background,
            };

            Ok((atoms::ok(), bytes, layer))
        }
        Err(_) => Err(Error::BadArg),
    }
}

fn image_format(format: ImageFormat) -> Atom {
    match format {
        ImageFormat::Png => atoms::png(),
        ImageFormat::Jpeg => atoms::jpg(),
        ImageFormat::Gif => atoms::gif(),
        _ => atoms::unsupported_image_format(),
    }
}


fn load(env: Env, _info: Term) -> bool {
    rustler::resource!(Image, env);
    true
}

rustler::init!("Elixir.Layers", [from_bytes, layer_images], load=load);
