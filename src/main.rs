use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
    
    let current_layer = doc.get_page(page1).get_layer(layer1);

    doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();
}
