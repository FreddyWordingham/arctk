//! Image detection functions.

use crate::Error;
use ndarray::Array2;

/// Calculate the edge map from the first hit index data.
#[inline]
pub fn find_edges<T: PartialEq>(img: &Array2<T>, rad: i32) -> Result<Array2<f64>, Error> {
    debug_assert!(rad > 0);

    let [width, height] = [img.shape()[0] as i32, img.shape()[1] as i32];
    let mut edges = Vec::with_capacity(img.len());

    let scan = scan_circle(rad);
    for xi in 0..width {
        for yi in 0..height {
            let mut max = 0.0;
            let mut diff = 0.0;
            let centre = &img[[xi as usize, yi as usize]];

            for ([dx, dy], w) in &scan {
                let px = xi + dx;
                let py = yi + dy;

                if (px >= 0) && (px < width) && (py >= 0) && (py < height) {
                    max += w;
                    if centre != &img[[px as usize, py as usize]] {
                        diff += w;
                    }
                }
            }

            edges.push(diff / max);
        }
    }

    println!("Edges len: {}", edges.len());

    Ok(Array2::from_shape_vec(
        [width as usize, height as usize],
        edges,
    )?)
}

/// Create a weighted scan circle.
#[inline]
#[must_use]
fn scan_circle(rad: i32) -> Vec<([i32; 2], f64)> {
    debug_assert!(rad > 0);

    let rad_sq = rad.pow(2);

    let mut scan = Vec::with_capacity(((rad * 2) + 1).pow(2) as usize);
    let mut total_weight = 0.0;
    for xi in -rad..=rad {
        for yi in -rad..=rad {
            // Not central pixel.
            if xi == 0 && yi == 0 {
                continue;
            }

            // Sample circle, not square.
            let r_sq = xi.pow(2) + yi.pow(2);
            if r_sq > rad_sq {
                continue;
            }

            let w = 1.0 - (f64::from(r_sq).sqrt() / f64::from(rad));
            total_weight += w;

            scan.push(([xi, yi], w));
        }
    }
    scan.shrink_to_fit();

    for (_, w) in &mut scan {
        *w /= total_weight;
    }

    scan
}
