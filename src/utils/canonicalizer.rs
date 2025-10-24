use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::io::Write;

pub struct Canonicalizer<W: Write> {
    writer: Writer<W>,
}

impl<W: Write> Canonicalizer<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer: Writer::new(writer),
        }
    }

    pub fn canonicalize_events<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
    ) -> quick_xml::Result<()> {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(e) => self.handle_start_event(&e)?,
                Event::End(e) => {
                    self.writer.write_event(Event::End(e.clone()))?;
                }
                Event::Empty(e) => {
                    self.handle_start_event(&e)?;
                    self.writer.write_event(Event::End(e.to_end()))?;
                }
                Event::Text(e) => {
                    if e.trim_ascii().is_empty() {
                        continue;
                    }
                    self.writer.write_event(Event::Text(e))?;
                }
                Event::CData(e) => {
                    self.writer.write_event(Event::Text(e.escape()?))?;
                }
                Event::GeneralRef(e) => {
                    let content = e.xml_content()?;
                    let bytes = BytesText::new(&content);
                    self.writer.write_event(Event::Text(bytes))?;
                }
                Event::Comment(_) | Event::Decl(_) | Event::PI(_) | Event::DocType(_) => (),
                Event::Eof => break,
            }
        }
        Ok(())
    }

    fn handle_start_event<'a>(&mut self, e: &BytesStart<'a>) -> quick_xml::Result<()> {
        let mut attrs: Vec<_> = e.attributes().collect::<Result<_, _>>()?;

        attrs.sort_unstable_by(|a, b| {
            let a_is_ns = a.key.as_ref().starts_with(b"xmlns");
            let b_is_ns = b.key.as_ref().starts_with(b"xmlns");

            (!a_is_ns, a.key.as_ref().to_ascii_lowercase())
                .cmp(&(!b_is_ns, b.key.as_ref().to_ascii_lowercase()))
        });

        let name = e.name();
        let mut sorted_element = BytesStart::new(std::str::from_utf8(name.as_ref()).unwrap());
        sorted_element.extend_attributes(attrs);

        self.writer.write_event(Event::Start(sorted_element))?;
        Ok(())
    }
}
