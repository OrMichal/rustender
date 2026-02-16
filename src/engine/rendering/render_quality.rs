///enum representing quality of rendering process
///number value represents maximum threads that can be used in a rendering process
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum RenderQuality {
    Low = 1,
    Medium = 2,
    High = 4
}
