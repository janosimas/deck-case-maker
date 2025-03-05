use deck::DeckSize;
use page::{AddPageSize, NewWithPageSize, PageSize};
use printpdf::path::{PaintMode, WindingOrder};
use printpdf::*;
use std::cmp;
use std::f32::consts::SQRT_2;
use std::fs::File;
use std::io::BufWriter;
use std::iter::FromIterator;

mod deck;
mod page;

const FACTOR: f32 = 2. / SQRT_2;

fn main() {
    let deck = DeckSize {
        width: Mm(70.),
        heigh: Mm(93.),
        depth: Mm(65.),
    };
    let margin = Mm(25.);

    let (doc, main_box_page_index, main_box_layer_index) =
        PdfDocument::new_with_page_size("Deck builder case", PageSize::A2, "main box");
    let main_box_layer = doc
        .get_page(main_box_page_index)
        .get_layer(main_box_layer_index);

    let (lid_page_index, lib_layer_index) = doc.add_page_with_size(PageSize::A2, "lid");
    let lid_layer = doc.get_page(lid_page_index).get_layer(lib_layer_index);
    let page_size = margin + deck.lid_paper_size();

    let lid_border_points = vec![
        (Point::new(margin, margin), false),
        (Point::new(margin, page_size), false),
        (Point::new(page_size, page_size), false),
        (Point::new(page_size, margin), false),
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
                    (Point::new(margin, margin + d * FACTOR), false),
                    (Point::new(margin + d * FACTOR, margin), false),
                ];
                folds.push(fold);
            }

            Polygon {
                rings: folds,
                mode: PaintMode::Stroke,
                winding_order: WindingOrder::NonZero,
            }
        };
        // TODO: use lines instead of polygon
        lid_layer.add_polygon(lid_vertical_folds);
    }
    {
        let lid_horizontal_folds = {
            let mut folds = vec![];
            for d in dbg!(deck.lid_horizontal_folds()) {
                let fold = vec![
                    (Point::new(margin, page_size - d * FACTOR), false),
                    (Point::new(margin + d * FACTOR, page_size), false),
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

    // Quadratic shape. The "false" determines if the next (following)
    // point is a bezier handle (for curves)
    //
    // If you want holes, use WindingOrder::EvenOdd
    let points1 = vec![
        (Point::new(Mm(100.0), Mm(100.0)), false),
        (Point::new(Mm(100.0), Mm(200.0)), false),
        (Point::new(Mm(300.0), Mm(200.0)), false),
        (Point::new(Mm(300.0), Mm(100.0)), false),
    ];

    let line1 = Polygon {
        rings: vec![points1],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };

    let fill_color = Color::Cmyk(Cmyk::new(0.0, 0.23, 0.0, 0.0, None));
    let outline_color = Color::Rgb(Rgb::new(0.75, 1.0, 0.64, None));
    let mut dash_pattern = LineDashPattern::default();
    dash_pattern.dash_1 = Some(20);

    main_box_layer.set_fill_color(fill_color);
    main_box_layer.set_outline_color(outline_color);
    main_box_layer.set_outline_thickness(10.0);

    // Draw first line
    main_box_layer.add_polygon(line1);

    let fill_color_2 = Color::Cmyk(Cmyk::new(0.0, 0.0, 0.0, 0.0, None));
    let outline_color_2 = Color::Greyscale(Greyscale::new(0.45, None));

    // More advanced graphical options
    main_box_layer.set_overprint_stroke(true);
    main_box_layer.set_blend_mode(BlendMode::Seperable(SeperableBlendMode::Multiply));
    main_box_layer.set_line_dash_pattern(dash_pattern);
    main_box_layer.set_line_cap_style(LineCapStyle::Round);

    main_box_layer.set_fill_color(fill_color_2);
    main_box_layer.set_outline_color(outline_color_2);
    main_box_layer.set_outline_thickness(15.0);

    // Triangle shape
    let mut line2 = Line::from_iter(vec![
        (Point::new(Mm(150.0), Mm(150.0)), false),
        (Point::new(Mm(150.0), Mm(250.0)), false),
        (Point::new(Mm(350.0), Mm(250.0)), false),
    ]);

    // draw second line
    main_box_layer.add_line(line2);

    doc.save(&mut BufWriter::new(
        File::create("test_working.pdf").unwrap(),
    ))
    .unwrap();
}
