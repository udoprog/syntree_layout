//! The module with the **Public API that is highly encouraged to be used**.

use syntree::{index::Index, pointer::Width, Tree};

use crate::{Drawer, Embedding, LayouterError, Result, SvgDrawer, Visualize, VisualizeEmbedder};

///
/// The Layouter type provides a simple builder mechanism with a fluent API.
///
pub struct Layouter<'t, 'd, 'p, T, I, W>
where
    I: Index,
    W: Width,
{
    tree: &'t Tree<T, I, W>,
    drawer: Option<&'d dyn Drawer>,
    file_name: Option<&'p std::path::Path>,
    embedding: Embedding,
}

impl<'t, 'd, 'p, T, I, W> Layouter<'t, 'd, 'p, T, I, W>
where
    I: Index,
    W: Width,
{
    ///
    /// Creates a new Layouter with the required tree.
    ///
    /// ```
    /// use syntree_layout::{Layouter, Visualize};
    /// use syntree::{Tree, Builder};
    ///
    /// struct MyNodeData(i32);
    ///
    /// impl Visualize for MyNodeData {
    ///     fn visualize(&self) -> std::string::String { self.0.to_string() }
    ///     fn emphasize(&self) -> bool { false }
    /// }
    ///
    ///
    /// let tree: Tree<MyNodeData, _, _> = Builder::new().build().unwrap();
    /// let layouter = Layouter::new(&tree);
    /// ```
    ///
    pub fn new(tree: &'t Tree<T, I, W>) -> Self {
        Self {
            tree,
            drawer: None,
            file_name: None,
            embedding: Vec::default(),
        }
    }

    ///
    /// Sets the path of the output file on the layouter.
    ///
    /// ```
    /// use syntree_layout::{Layouter, Visualize};
    /// use syntree::{Tree, Builder};
    /// use std::path::Path;
    ///
    /// struct MyNodeData(i32);
    ///
    /// impl Visualize for MyNodeData {
    ///     fn visualize(&self) -> std::string::String { self.0.to_string() }
    ///     fn emphasize(&self) -> bool { false }
    /// }
    ///
    ///
    /// let tree: Tree<MyNodeData, _, _> = Builder::new().build().unwrap();
    /// let layouter = Layouter::new(&tree)
    ///     .with_file_path(Path::new("test.svg"));
    /// ```
    ///
    pub fn with_file_path(self, path: &'p std::path::Path) -> Self {
        Self {
            tree: self.tree,
            file_name: Some(path),
            drawer: self.drawer,
            embedding: self.embedding,
        }
    }

    ///
    /// Sets a different drawer when you don't want to use the default svg-drawer.
    /// If this method is not called the crate's own svg-drawer is used.
    ///
    /// ```
    /// use syntree_layout::{Drawer, Layouter, EmbeddedNode, Result, Visualize};
    /// use syntree::{Tree, Builder};
    /// use std::path::Path;
    ///
    /// struct NilDrawer;
    /// impl Drawer for NilDrawer {
    ///     fn draw(&self, _file_name: &Path, _embedding: &[EmbeddedNode]) -> Result<()> {
    ///         Ok(())
    ///     }
    /// }
    ///
    /// struct MyNodeData(i32);
    ///
    /// impl Visualize for MyNodeData {
    ///     fn visualize(&self) -> std::string::String { self.0.to_string() }
    ///     fn emphasize(&self) -> bool { false }
    /// }
    ///
    ///
    /// let tree: Tree<MyNodeData, _, _> = Builder::new().build().unwrap();
    /// let drawer = NilDrawer;
    /// let layouter = Layouter::new(&tree)
    ///     .with_drawer(&drawer)
    ///     .with_file_path(Path::new("test.svg"));
    /// ```
    ///
    pub fn with_drawer(self, drawer: &'d dyn Drawer) -> Self {
        Self {
            tree: self.tree,
            file_name: self.file_name,
            drawer: Some(drawer),
            embedding: self.embedding,
        }
    }

    ///
    /// When the layouter instance is fully configured this method invokes the necessary embedding
    /// functionality and uses the drawer which writes the result to the output file in its own
    /// output format.
    ///
    /// ```
    /// use syntree_layout::{Layouter, Visualize, Result};
    /// use syntree::{Tree, Builder};
    /// use std::path::Path;
    ///
    /// struct MyNodeData(i32);
    ///
    /// impl Visualize for MyNodeData {
    ///     fn visualize(&self) -> std::string::String { self.0.to_string() }
    ///     fn emphasize(&self) -> bool { false }
    /// }
    ///
    /// fn test() -> Result<()> {
    ///     let tree: Tree<MyNodeData, _, _> = Builder::new().build().unwrap();
    ///     Ok(Layouter::new(&tree)
    ///         .with_file_path(Path::new("test.svg"))
    ///         .embed_with_visualize()?
    ///         .write().expect("Failed writing layout"))
    /// }
    ///
    /// test().expect("Embedding should work");
    /// ```
    ///
    pub fn write(&self) -> Result<()> {
        if self.file_name.is_none() {
            Err(LayouterError::from_description(
                "No output file name given - use Layouter::with_file_path.".to_string(),
            ))
        } else {
            let default_drawer = SvgDrawer::new();
            let drawer = self.drawer.unwrap_or(&default_drawer);
            drawer.draw(self.file_name.unwrap(), &self.embedding)
        }
    }
}

impl<'t, 'd, 'p, T, I, W> Layouter<'t, 'd, 'p, T, I, W>
where
    T: Visualize,
    I: Index,
    W: Width,
{
    pub fn embed_with_visualize(self) -> Result<Self> {
        let embedding = VisualizeEmbedder::embed(&self.tree)?;
        Ok(Self {
            tree: self.tree,
            file_name: self.file_name,
            drawer: self.drawer,
            embedding,
        })
    }
}