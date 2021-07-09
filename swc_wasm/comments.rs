use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use swc_common::comments::{Comment, CommentKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentInner(Comment);

impl From<&Comment> for CommentInner {
    fn from(comment: &Comment) -> Self {
        CommentInner(comment.clone())
    }
}

impl From<Comment> for CommentInner {
    fn from(comment: Comment) -> Self {
        CommentInner(comment)
    }
}

impl Serialize for CommentInner {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Comment", 3)?;
        let x = match &self.0.kind {
            CommentKind::Line => "Line",
            CommentKind::Block => "Block",
        };
        state.serialize_field("kind", &x)?;
        state.serialize_field("span", &self.0.span)?;
        state.serialize_field("text", &self.0.text)?;
        state.end()
    }
}
