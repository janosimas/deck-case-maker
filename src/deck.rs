use std::f32::consts::SQRT_2;

pub struct DeckSize {
    pub width: f32,
    pub heigh: f32,
    pub depth: f32,
}

impl DeckSize {
    pub fn box_paper_size(&self) -> f32 {
        (self.width + self.heigh + 4. * self.depth) / SQRT_2
    }

    pub fn box_vertical_folds(&self) -> [f32; 8] {
        [
            self.width / 2.,
            self.width / 2. + self.depth,
            self.width / 2. + self.depth + self.depth,
            self.width / 2. + self.depth + self.depth + self.heigh / 2.,
            self.width / 2. + self.depth + self.depth + self.heigh / 2. + self.heigh / 2.,
            self.width / 2.
                + self.depth
                + self.depth
                + self.heigh / 2.
                + self.heigh / 2.
                + self.depth,
            self.width / 2.
                + self.depth
                + self.depth
                + self.heigh / 2.
                + self.heigh / 2.
                + self.depth
                + self.depth,
            self.width / 2.
                + self.depth
                + self.depth
                + self.heigh / 2.
                + self.heigh / 2.
                + self.depth
                + self.depth
                + self.width / 2.,
        ]
    }

    pub fn box_horizontal_folds(&self) -> [f32; 8] {
        [
            self.heigh / 2.,
            self.heigh / 2. + self.depth,
            self.heigh / 2. + self.depth + self.depth,
            self.heigh / 2. + self.depth + self.depth + self.width / 2.,
            self.heigh / 2. + self.depth + self.depth + self.width / 2. + self.width / 2.,
            self.heigh / 2.
                + self.depth
                + self.depth
                + self.width / 2.
                + self.width / 2.
                + self.depth,
            self.heigh / 2.
                + self.depth
                + self.depth
                + self.width / 2.
                + self.width / 2.
                + self.depth
                + self.depth,
            self.heigh / 2.
                + self.depth
                + self.depth
                + self.width / 2.
                + self.width / 2.
                + self.depth
                + self.depth
                + self.heigh / 2.,
        ]
    }

    pub(crate) fn lid_paper_height(&self) -> f32 {
        self.depth * 4.
    }

    pub(crate) fn lid_paper_width(&self) -> f32 {
        self.depth + self.width + self.depth
    }

    pub(crate) fn lid_vertical_folds(&self) -> [f32; 3] {
        [
            self.depth,
            self.depth + self.width,
            self.depth + self.width + self.depth,
        ]
    }

    pub(crate) fn lid_horizontal_folds(&self) -> [f32; 4] {
        [
            self.depth,
            self.depth + self.depth,
            self.depth + self.depth + self.depth,
            self.depth + self.depth + self.depth + self.depth,
        ]
    }
}
