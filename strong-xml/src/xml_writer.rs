use std::io::Result;
use std::io::Write;

use crate::xml_escape::xml_escape;

pub struct XmlWriter<W: Write> {
    pub inner: W,
}

impl<W: Write> XmlWriter<W> {
    pub fn new(inner: W) -> Self {
        XmlWriter { inner }
    }

    pub fn into_inner(self) -> W {
        self.inner
    }

    pub fn write_element_start(&mut self, tag: &str) -> Result<()> {
        write!(self.inner, "<{}", tag)
    }

    pub fn write_attribute(&mut self, key: &str, value: &str) -> Result<()> {
        write!(self.inner, r#" {}="{}""#, key, xml_escape(value))
    }

    pub fn write_text(&mut self, content: &str) -> Result<()> {
        write!(self.inner, "{}", xml_escape(content))
    }

    pub fn write_element_end_open(&mut self) -> Result<()> {
        write!(self.inner, ">")
    }

    pub fn write_element_end_close(&mut self, tag: &str) -> Result<()> {
        write!(self.inner, "</{}>", tag)
    }

    pub fn write_element_end_empty(&mut self) -> Result<()> {
        write!(self.inner, "/>")?;
        Ok(())
    }
}
