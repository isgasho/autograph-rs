use super::shader_interface::ShaderInterfaceDesc;

/// A trait representing a shader
pub trait Shader {}

/// A trait representing a vertex shader
pub trait VertexShader: Shader {}
pub trait FragmentShader: Shader {}
pub trait GeometryShader: Shader {}
pub trait TessControlShader: Shader {}
pub trait TessEvalShader: Shader {}
pub trait ComputeShader: Shader {}

/// A trait representing a collection of shaders (of the same type)
/// for the whole graphics pipeline.
///
/// Potential implementations: GlslShaderPipeline, GlslBinaryPipeline, SpirvBinaryPipeline, etc.
pub trait GraphicsShaderPipeline
{
    fn vertex_shader(&self) -> &VertexShader;
    fn fragment_shader(&self) -> &FragmentShader;
    fn geometry_shader(&self) -> Option<&GeometryShader>;
    fn tess_control_shader(&self) -> Option<&TessControlShader>;
    fn tess_eval_shader(&self) -> Option<&TessEvalShader>;
    fn is_compatible_with(&self, interface: &ShaderInterfaceDesc) -> bool;
}

pub trait ComputeShaderPipeline
{
    fn compute_shader(&self) -> &ComputeShader;
    fn is_compatible_with(&self, interface: &ShaderInterfaceDesc) -> bool;
}
