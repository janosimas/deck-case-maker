use std::f32::consts::SQRT_2;

use printpdf::Mm;

pub struct DeckSize {
    pub width: Mm,
    pub heigh: Mm,
    pub depth: Mm,
}

impl DeckSize {
    pub fn lid_paper_size(&self) -> Mm {
        Mm((self.width.0 + self.heigh.0 + 4. * self.depth.0) / SQRT_2)
    }

    pub fn lid_vertical_folds(&self) -> [Mm; 8] {
        [
            Mm(self.width.0 / 2.),
            Mm(self.width.0 / 2.) + self.depth,
            Mm(self.width.0 / 2.) + self.depth + self.depth,
            Mm(self.width.0 / 2.) + self.depth + self.depth + Mm(self.heigh.0 / 2.),
            Mm(self.width.0 / 2.)
                + self.depth
                + self.depth
                + Mm(self.heigh.0 / 2.)
                + Mm(self.heigh.0 / 2.),
            Mm(self.width.0 / 2.)
                + self.depth
                + self.depth
                + Mm(self.heigh.0 / 2.)
                + Mm(self.heigh.0 / 2.)
                + self.depth,
            Mm(self.width.0 / 2.)
                + self.depth
                + self.depth
                + Mm(self.heigh.0 / 2.)
                + Mm(self.heigh.0 / 2.)
                + self.depth
                + self.depth,
            Mm(self.width.0 / 2.)
                + self.depth
                + self.depth
                + Mm(self.heigh.0 / 2.)
                + Mm(self.heigh.0 / 2.)
                + self.depth
                + self.depth
                + Mm(self.width.0 / 2.),
        ]
    }

    pub fn lid_horizontal_folds(&self) -> [Mm; 8] {
        [
            Mm(self.heigh.0 / 2.),
            Mm(self.heigh.0 / 2.) + self.depth,
            Mm(self.heigh.0 / 2.) + self.depth + self.depth,
            Mm(self.heigh.0 / 2.) + self.depth + self.depth + Mm(self.width.0 / 2.),
            Mm(self.heigh.0 / 2.)
                + self.depth
                + self.depth
                + Mm(self.width.0 / 2.)
                + Mm(self.width.0 / 2.),
            Mm(self.heigh.0 / 2.)
                + self.depth
                + self.depth
                + Mm(self.width.0 / 2.)
                + Mm(self.width.0 / 2.)
                + self.depth,
            Mm(self.heigh.0 / 2.)
                + self.depth
                + self.depth
                + Mm(self.width.0 / 2.)
                + Mm(self.width.0 / 2.)
                + self.depth
                + self.depth,
            Mm(self.heigh.0 / 2.)
                + self.depth
                + self.depth
                + Mm(self.width.0 / 2.)
                + Mm(self.width.0 / 2.)
                + self.depth
                + self.depth
                + Mm(self.heigh.0 / 2.),
        ]
    }
}
