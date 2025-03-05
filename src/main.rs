use deck::DeckSize;
use page::{AddPageSize, NewWithPageSize, PageSize};
use printpdf::path::{PaintMode, WindingOrder};
use printpdf::*;
use std::f32::consts::SQRT_2;
use std::fs::File;
use std::io::BufWriter;

mod deck;
mod page;

const FACTOR: f32 = 2. / SQRT_2;

fn main() {
    let deck = DeckSize {
        width: 60.,
        heigh: 93.,
        depth: 30.,
    };
    let offset = 5.;

    let (doc, box_page_index, box_layer_index) =
        PdfDocument::new_with_page_size("Deck builder case", PageSize::A4, "Box");
    let box_layer = doc.get_page(box_page_index).get_layer(box_layer_index);
    add_box_to_page(&deck, &box_layer, offset);

    let (lid_page_index, lid_layer_index) = doc.add_page_with_size(PageSize::A4, "lids");
    let lid_layer = doc.get_page(lid_page_index).get_layer(lid_layer_index);

    add_lid_to_page(&deck, &lid_layer, offset);

    doc.save(&mut BufWriter::new(
        File::create("test_working.pdf").unwrap(),
    ))
    .unwrap();
}

fn add_lid_to_page(deck: &DeckSize, lid_layer: &PdfLayerReference, offset: f32) {
    let page_size = deck.lid_paper_size();
    let lid_border_points = vec![
        (Point::new(Mm(offset), Mm(offset)), false),
        (Point::new(Mm(offset), Mm(offset + page_size)), false),
        (
            Point::new(Mm(offset + page_size), Mm(offset + page_size)),
            false,
        ),
        (Point::new(Mm(offset + page_size), Mm(offset)), false),
    ];

    lid_layer.add_polygon(Polygon {
        rings: vec![lid_border_points.clone()],
        mode: PaintMode::Stroke,
        winding_order: WindingOrder::NonZero,
    });

    lid_layer.add_polygon(Polygon {
        rings: vec![lid_border_points],
        mode: PaintMode::Clip,
        winding_order: WindingOrder::NonZero,
    });

    // TODO: Set line style
    // let dash_pattern = LineDashPattern {
    //     dash_1: Some(20),
    //     ..Default::default()
    // };
    // lid_layer.set_line_dash_pattern(dash_pattern);
    {
        let lid_vertical_folds = {
            let mut folds = vec![];
            for d in deck.lid_vertical_folds() {
                let fold = vec![
                    (Point::new(Mm(offset + d), Mm(offset)), false),
                    (Point::new(Mm(offset + d), Mm(offset + page_size)), false),
                ];
                folds.push(fold);
            }

            Polygon {
                rings: folds,
                mode: PaintMode::Stroke,
                winding_order: WindingOrder::NonZero,
            }
        };
        lid_layer.add_polygon(lid_vertical_folds);
    }
    {
        let lid_horizontal_folds = {
            let mut folds = vec![];
            for d in deck.lid_horizontal_folds() {
                let fold = vec![
                    (Point::new(Mm(offset), Mm(offset + d)), false),
                    (Point::new(Mm(offset + page_size), Mm(offset + d)), false),
                ];
                folds.push(fold);
            }

            Polygon {
                rings: folds,
                mode: PaintMode::Stroke,
                winding_order: WindingOrder::NonZero,
            }
        };
        lid_layer.add_polygon(lid_horizontal_folds);
    }
}

fn add_box_to_page(deck: &DeckSize, box_layer: &PdfLayerReference, offset: f32) {
    let page_size = deck.box_paper_size();
    let box_border_points = vec![
        (Point::new(Mm(offset), Mm(offset)), false),
        (Point::new(Mm(offset), Mm(offset + page_size)), false),
        (
            Point::new(Mm(offset + page_size), Mm(offset + page_size)),
            false,
        ),
        (Point::new(Mm(offset + page_size), Mm(offset)), false),
    ];

    box_layer.add_polygon(Polygon {
        rings: vec![box_border_points.clone()],
        mode: PaintMode::Stroke,
        winding_order: WindingOrder::NonZero,
    });

    box_layer.add_polygon(Polygon {
        rings: vec![box_border_points],
        mode: PaintMode::Clip,
        winding_order: WindingOrder::NonZero,
    });

    // TODO: Set line style
    // let dash_pattern = LineDashPattern {
    //     dash_1: Some(20),
    //     ..Default::default()
    // };
    // lid_layer.set_line_dash_pattern(dash_pattern);
    {
        let box_vertical_folds = {
            let mut folds = vec![];
            for d in deck.box_vertical_folds() {
                let fold = vec![
                    (Point::new(Mm(offset), Mm(offset + d * FACTOR)), false),
                    (Point::new(Mm(offset + d * FACTOR), Mm(offset)), false),
                ];
                folds.push(fold);
            }

            Polygon {
                rings: folds,
                mode: PaintMode::Stroke,
                winding_order: WindingOrder::NonZero,
            }
        };
        box_layer.add_polygon(box_vertical_folds);
    }
    {
        let box_horizontal_folds = {
            let mut folds = vec![];
            for d in deck.box_horizontal_folds() {
                let fold = vec![
                    (
                        Point::new(Mm(offset), Mm(offset + page_size - d * FACTOR)),
                        false,
                    ),
                    (
                        Point::new(Mm(offset + d * FACTOR), Mm(offset + page_size)),
                        false,
                    ),
                ];
                folds.push(fold);
            }

            Polygon {
                rings: folds,
                mode: PaintMode::Stroke,
                winding_order: WindingOrder::NonZero,
            }
        };
        box_layer.add_polygon(box_horizontal_folds);
    }
}
