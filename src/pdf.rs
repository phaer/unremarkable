// temporarily in-sourced as upstream only renders the first page
use std::io;
use pdf_canvas::graphicsstate::{self, CapStyle, JoinStyle, Matrix};
use pdf_canvas::Pdf;
use lines_are_rusty::Page;

const BASE_LINE_WIDTH: f32 = 4.;

pub fn render(path: &str, pages: Vec<Page>) -> io::Result<()> {
    let mut document = Pdf::create(path)?;

    for page in pages {

        document.render_page(1404.0, 1872.0, |c| {
            // Inverse Y coordinate system.
            c.concat(Matrix::scale(1., -1.))?;
            c.concat(Matrix::translate(0., -1872.))?;

            c.set_stroke_color(graphicsstate::Color::gray(0))?;

            for layer in &page.layers {
                for line in &layer.lines {
                    if line.points.is_empty() {
                        continue;
                    }
                    let first_point = &line.points[0];
                    c.move_to(first_point.x, first_point.y)?;
                    for point in &line.points {
                        c.set_line_width(point.pressure * BASE_LINE_WIDTH)?;
                        c.set_line_cap_style(CapStyle::Round)?;
                        c.set_line_join_style(JoinStyle::Round)?;
                        c.line_to(point.x, point.y)?;
                    }
                    c.stroke()?;
                }
            }

            Ok(())
        })?;
    }
        document.finish()?;
        Ok(())
    }
