//! Main Document part
//!
//! The corresponding ZIP item is `/word/document.xml`.

mod body;
mod bookmark;
mod r#break;
mod hyperlink;
mod paragraph;
mod run;
mod text;

pub use self::{body::*, bookmark::*, hyperlink::*, paragraph::*, r#break::*, run::*, text::*};

use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::io::Write;

use crate::{
    error::{Error, Result},
    schema::SCHEMA_MAIN,
};

/// The root element of the main document part.
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[xml(tag = "w:document")]
#[xml(extend_attrs = "document_extend_attrs")]
pub struct Document<'a> {
    /// Specifies the body of the docment.
    #[xml(child = "w:body")]
    pub body: Body<'a>,
}

impl<'a> Document<'a> {
    pub fn push<T: Into<BodyContent<'a>>>(&mut self, content: T) -> &mut Self {
        self.body.push(content);
        self
    }
}

#[inline]
fn document_extend_attrs<W: Write>(_: &Document, mut w: W) -> Result<()> {
    write!(w, " xmlns:w=\"{}\"", SCHEMA_MAIN)?;
    Ok(())
}
