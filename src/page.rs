use printpdf::{Mm, PdfDocument, PdfDocumentReference, PdfLayerIndex, PdfPageIndex};

pub enum PageSize {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
}

impl PageSize {
    pub fn width(&self) -> Mm {
        match self {
            PageSize::A0 => Mm(841.),
            PageSize::A1 => Mm(594.),
            PageSize::A2 => Mm(420.),
            PageSize::A3 => Mm(297.),
            PageSize::A4 => Mm(210.),
            PageSize::A5 => Mm(148.),
            PageSize::A6 => Mm(105.),
            PageSize::A7 => Mm(74.),
        }
    }

    pub fn heigh(&self) -> Mm {
        match self {
            PageSize::A0 => Mm(1189.),
            PageSize::A1 => Mm(841.),
            PageSize::A2 => Mm(594.),
            PageSize::A3 => Mm(420.),
            PageSize::A4 => Mm(297.),
            PageSize::A5 => Mm(210.),
            PageSize::A6 => Mm(148.),
            PageSize::A7 => Mm(105.),
        }
    }
}

pub trait NewWithPageSize {
    fn new_with_page_size(
        document_title: &str,
        page_size: PageSize,
        initial_layer_name: &str,
    ) -> (PdfDocumentReference, PdfPageIndex, PdfLayerIndex);
}

impl NewWithPageSize for PdfDocument {
    fn new_with_page_size(
        document_title: &str,
        page_size: PageSize,
        initial_layer_name: &str,
    ) -> (PdfDocumentReference, PdfPageIndex, PdfLayerIndex) {
        PdfDocument::new(
            document_title,
            page_size.width(),
            page_size.heigh(),
            initial_layer_name,
        )
    }
}

pub trait AddPageSize {
    fn add_page_with_size(
        &self,
        page_size: PageSize,
        inital_layer_name: &str,
    ) -> (printpdf::PdfPageIndex, printpdf::PdfLayerIndex);
}

impl AddPageSize for PdfDocumentReference {
    fn add_page_with_size(
        &self,
        page_size: PageSize,
        inital_layer_name: &str,
    ) -> (printpdf::PdfPageIndex, printpdf::PdfLayerIndex) {
        self.add_page(page_size.width(), page_size.heigh(), inital_layer_name)
    }
}
