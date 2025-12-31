use super::ast::{
    Alias, Comment, Definition, Element, Import, NamespaceDeclaration, Package, SysMLFile, Usage,
};

pub trait AstVisitor {
    fn visit_file(&mut self, _file: &SysMLFile) {}
    fn visit_namespace(&mut self, _namespace: &NamespaceDeclaration) {}
    fn visit_element(&mut self, _element: &Element) {}
    fn visit_package(&mut self, _package: &Package) {}
    fn visit_definition(&mut self, _definition: &Definition) {}
    fn visit_usage(&mut self, _usage: &Usage) {}
    fn visit_comment(&mut self, _comment: &Comment) {}
    fn visit_import(&mut self, _import: &Import) {}
    fn visit_alias(&mut self, _alias: &Alias) {}
}

pub trait Visitable {
    fn accept<V: AstVisitor>(&self, visitor: &mut V);
}

macro_rules! impl_visitable {
    ($type:ty, $visit_method:ident) => {
        impl Visitable for $type {
            fn accept<V: AstVisitor>(&self, visitor: &mut V) {
                visitor.$visit_method(self);
            }
        }
    };
    ($type:ty, $visit_method:ident, |$self:ident, $visitor:ident| $walk:block) => {
        impl Visitable for $type {
            fn accept<V: AstVisitor>(&$self, $visitor: &mut V) {
                $visitor.$visit_method(&$self);
                $walk
            }
        }
    };
}

impl_visitable!(SysMLFile, visit_file, |self, visitor| {
    if let Some(ref ns) = self.namespace {
        ns.accept(visitor);
    }
    for element in &self.elements {
        element.accept(visitor);
    }
});

impl_visitable!(NamespaceDeclaration, visit_namespace);

impl Visitable for Element {
    fn accept<V: AstVisitor>(&self, visitor: &mut V) {
        visitor.visit_element(self);
        match self {
            Element::Package(p) => p.accept(visitor),
            Element::Definition(d) => d.accept(visitor),
            Element::Usage(u) => u.accept(visitor),
            Element::Comment(c) => c.accept(visitor),
            Element::Import(i) => i.accept(visitor),
            Element::Alias(a) => a.accept(visitor),
        }
    }
}

impl_visitable!(Package, visit_package, |self, visitor| {
    for element in &self.elements {
        element.accept(visitor);
    }
});

impl_visitable!(Definition, visit_definition);
impl_visitable!(Usage, visit_usage);
impl_visitable!(Comment, visit_comment);
impl_visitable!(Import, visit_import);
impl_visitable!(Alias, visit_alias);

#[cfg(test)]
mod tests;
