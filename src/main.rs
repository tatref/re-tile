use image::{GenericImage, GenericImageView, ImageBuffer, ImageReader};
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 8 {
        println!(
            "Usage: {} <src> <tile_size> <src_padding> <src_offset> <dst> <dst_padding> <dst_offset>",
            args[0]
        );
        exit(1);
    }

    let src = args[1].clone();
    let tile_size: u32 = args[2].parse().unwrap();
    let src_padding: u32 = args[3].parse().unwrap();
    let src_offset: u32 = args[4].parse().unwrap();

    let dst = args[5].clone();
    let dst_padding: u32 = args[6].parse().unwrap();
    let dst_offset: u32 = args[7].parse().unwrap();

    let src = ImageReader::open(&src).unwrap().decode().unwrap();
    let src_w = src.width();
    let src_h = src.height();
    dbg!(src_w, src_h);

    let columns = (src_w - 2 * src_offset) / (tile_size + src_padding);
    let rows = (src_h - 2 * src_offset) / (tile_size + src_padding);
    dbg!(columns, rows);

    let dst_w = dst_offset * 2 + columns * tile_size + (columns - 1) * dst_padding;
    let dst_h = dst_offset * 2 + rows * tile_size + (rows - 1) * dst_padding;

    let mut new_image = ImageBuffer::new(dst_w, dst_h);
    dbg!(dst_w, dst_h);

    for row in 0..rows {
        for column in 0..columns {
            dbg!(row, column);
            let src_x = src_offset + column * (src_padding + tile_size);
            let src_y = src_offset + row * (src_padding + tile_size);
            dbg!(src_x, src_y);
            let tile = src.view(src_x, src_y, tile_size, tile_size).to_image();

            let dst_x = dst_offset + column * (dst_padding + tile_size);
            let dst_y = dst_offset + row * (dst_padding + tile_size);
            dbg!(dst_x, dst_y);
            new_image.copy_from(&tile, dst_x, dst_y).unwrap();
        }
    }

    new_image.save(&dst).unwrap();
}
