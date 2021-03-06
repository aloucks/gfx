use ash::vk;

use hal::range::RangeArg;
use hal::{buffer, command, format, image, pass, pso, query};
use hal::{CompositeAlpha, Features, IndexType, PresentMode, Primitive};

use native as n;
use std::borrow::Borrow;
use std::mem;
use std::ptr;

pub fn map_format(format: format::Format) -> vk::Format {
    vk::Format::from_raw(format as i32)
}

pub fn map_vk_format(vk_format: vk::Format) -> Option<format::Format> {
    if (vk_format.as_raw() as usize) < format::NUM_FORMATS && vk_format != vk::Format::UNDEFINED {
        Some(unsafe { mem::transmute(vk_format) })
    } else {
        None
    }
}

pub fn map_tiling(tiling: image::Tiling) -> vk::ImageTiling {
    vk::ImageTiling::from_raw(tiling as i32)
}

pub fn map_component(component: format::Component) -> vk::ComponentSwizzle {
    use hal::format::Component::*;
    match component {
        Zero => vk::ComponentSwizzle::ZERO,
        One => vk::ComponentSwizzle::ONE,
        R => vk::ComponentSwizzle::R,
        G => vk::ComponentSwizzle::G,
        B => vk::ComponentSwizzle::B,
        A => vk::ComponentSwizzle::A,
    }
}

pub fn map_swizzle(swizzle: format::Swizzle) -> vk::ComponentMapping {
    vk::ComponentMapping {
        r: map_component(swizzle.0),
        g: map_component(swizzle.1),
        b: map_component(swizzle.2),
        a: map_component(swizzle.3),
    }
}

pub fn map_index_type(index_type: IndexType) -> vk::IndexType {
    match index_type {
        IndexType::U16 => vk::IndexType::UINT16,
        IndexType::U32 => vk::IndexType::UINT32,
    }
}

pub fn map_image_layout(layout: image::Layout) -> vk::ImageLayout {
    use hal::image::Layout as Il;
    match layout {
        Il::General => vk::ImageLayout::GENERAL,
        Il::ColorAttachmentOptimal => vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        Il::DepthStencilAttachmentOptimal => vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        Il::DepthStencilReadOnlyOptimal => vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
        Il::ShaderReadOnlyOptimal => vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        Il::TransferSrcOptimal => vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
        Il::TransferDstOptimal => vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        Il::Undefined => vk::ImageLayout::UNDEFINED,
        Il::Preinitialized => vk::ImageLayout::PREINITIALIZED,
        Il::Present => vk::ImageLayout::PRESENT_SRC_KHR,
    }
}

pub fn map_image_aspects(aspects: format::Aspects) -> vk::ImageAspectFlags {
    vk::ImageAspectFlags::from_raw(aspects.bits() as u32)
}

pub fn map_clear_color(value: command::ClearColor) -> vk::ClearColorValue {
    match value {
        command::ClearColor::Float(v) => vk::ClearColorValue { float32: v },
        command::ClearColor::Int(v) => vk::ClearColorValue { int32: v },
        command::ClearColor::Uint(v) => vk::ClearColorValue { uint32: v },
    }
}

pub fn map_offset(offset: image::Offset) -> vk::Offset3D {
    vk::Offset3D {
        x: offset.x,
        y: offset.y,
        z: offset.z,
    }
}

pub fn map_extent(offset: image::Extent) -> vk::Extent3D {
    vk::Extent3D {
        width: offset.width,
        height: offset.height,
        depth: offset.depth,
    }
}

pub fn map_subresource(sub: &image::Subresource) -> vk::ImageSubresource {
    vk::ImageSubresource {
        aspect_mask: map_image_aspects(sub.aspects),
        mip_level: sub.level as _,
        array_layer: sub.layer as _,
    }
}

pub fn map_subresource_layers(sub: &image::SubresourceLayers) -> vk::ImageSubresourceLayers {
    vk::ImageSubresourceLayers {
        aspect_mask: map_image_aspects(sub.aspects),
        mip_level: sub.level as _,
        base_array_layer: sub.layers.start as _,
        layer_count: (sub.layers.end - sub.layers.start) as _,
    }
}

pub fn map_subresource_range(range: &image::SubresourceRange) -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange {
        aspect_mask: map_image_aspects(range.aspects),
        base_mip_level: range.levels.start as _,
        level_count: (range.levels.end - range.levels.start) as _,
        base_array_layer: range.layers.start as _,
        layer_count: (range.layers.end - range.layers.start) as _,
    }
}

pub fn map_attachment_load_op(op: pass::AttachmentLoadOp) -> vk::AttachmentLoadOp {
    use hal::pass::AttachmentLoadOp as Alo;
    match op {
        Alo::Load => vk::AttachmentLoadOp::LOAD,
        Alo::Clear => vk::AttachmentLoadOp::CLEAR,
        Alo::DontCare => vk::AttachmentLoadOp::DONT_CARE,
    }
}

pub fn map_attachment_store_op(op: pass::AttachmentStoreOp) -> vk::AttachmentStoreOp {
    use hal::pass::AttachmentStoreOp as Aso;
    match op {
        Aso::Store => vk::AttachmentStoreOp::STORE,
        Aso::DontCare => vk::AttachmentStoreOp::DONT_CARE,
    }
}

pub fn map_buffer_access(access: buffer::Access) -> vk::AccessFlags {
    vk::AccessFlags::from_raw(access.bits())
}

pub fn map_image_access(access: image::Access) -> vk::AccessFlags {
    vk::AccessFlags::from_raw(access.bits())
}

pub fn map_pipeline_stage(stage: pso::PipelineStage) -> vk::PipelineStageFlags {
    vk::PipelineStageFlags::from_raw(stage.bits())
}

pub fn map_buffer_usage(usage: buffer::Usage) -> vk::BufferUsageFlags {
    vk::BufferUsageFlags::from_raw(usage.bits())
}

pub fn map_image_usage(usage: image::Usage) -> vk::ImageUsageFlags {
    vk::ImageUsageFlags::from_raw(usage.bits())
}

pub fn map_vk_image_usage(usage: vk::ImageUsageFlags) -> image::Usage {
    image::Usage::from_bits_truncate(usage.as_raw())
}

pub fn map_descriptor_type(ty: pso::DescriptorType) -> vk::DescriptorType {
    vk::DescriptorType::from_raw(ty as i32)
}

pub fn map_stage_flags(stages: pso::ShaderStageFlags) -> vk::ShaderStageFlags {
    vk::ShaderStageFlags::from_raw(stages.bits())
}

pub fn map_filter(filter: image::Filter) -> vk::Filter {
    vk::Filter::from_raw(filter as i32)
}

pub fn map_mip_filter(filter: image::Filter) -> vk::SamplerMipmapMode {
    vk::SamplerMipmapMode::from_raw(filter as i32)
}

pub fn map_wrap(wrap: image::WrapMode) -> vk::SamplerAddressMode {
    use hal::image::WrapMode as Wm;
    match wrap {
        Wm::Tile => vk::SamplerAddressMode::REPEAT,
        Wm::Mirror => vk::SamplerAddressMode::MIRRORED_REPEAT,
        Wm::Clamp => vk::SamplerAddressMode::CLAMP_TO_EDGE,
        Wm::Border => vk::SamplerAddressMode::CLAMP_TO_BORDER,
    }
}

pub fn map_border_color(col: image::PackedColor) -> Option<vk::BorderColor> {
    match col.0 {
        0x00000000 => Some(vk::BorderColor::FLOAT_TRANSPARENT_BLACK),
        0xFF000000 => Some(vk::BorderColor::FLOAT_OPAQUE_BLACK),
        0xFFFFFFFF => Some(vk::BorderColor::FLOAT_OPAQUE_WHITE),
        _ => None,
    }
}

pub fn map_topology(prim: Primitive) -> vk::PrimitiveTopology {
    match prim {
        Primitive::PointList => vk::PrimitiveTopology::POINT_LIST,
        Primitive::LineList => vk::PrimitiveTopology::LINE_LIST,
        Primitive::LineListAdjacency => vk::PrimitiveTopology::LINE_LIST_WITH_ADJACENCY,
        Primitive::LineStrip => vk::PrimitiveTopology::LINE_STRIP,
        Primitive::LineStripAdjacency => vk::PrimitiveTopology::LINE_STRIP_WITH_ADJACENCY,
        Primitive::TriangleList => vk::PrimitiveTopology::TRIANGLE_LIST,
        Primitive::TriangleListAdjacency => vk::PrimitiveTopology::TRIANGLE_LIST_WITH_ADJACENCY,
        Primitive::TriangleStrip => vk::PrimitiveTopology::TRIANGLE_STRIP,
        Primitive::TriangleStripAdjacency => vk::PrimitiveTopology::TRIANGLE_STRIP_WITH_ADJACENCY,
        Primitive::PatchList(_) => vk::PrimitiveTopology::PATCH_LIST,
    }
}

pub fn map_polygon_mode(rm: pso::PolygonMode) -> (vk::PolygonMode, f32) {
    match rm {
        pso::PolygonMode::Point => (vk::PolygonMode::POINT, 1.0),
        pso::PolygonMode::Line(w) => (vk::PolygonMode::LINE, w),
        pso::PolygonMode::Fill => (vk::PolygonMode::FILL, 1.0),
    }
}

pub fn map_cull_face(cf: pso::Face) -> vk::CullModeFlags {
    match cf {
        pso::Face::NONE => vk::CullModeFlags::NONE,
        pso::Face::FRONT => vk::CullModeFlags::FRONT,
        pso::Face::BACK => vk::CullModeFlags::BACK,
        _ => vk::CullModeFlags::FRONT_AND_BACK,
    }
}

pub fn map_front_face(ff: pso::FrontFace) -> vk::FrontFace {
    match ff {
        pso::FrontFace::Clockwise => vk::FrontFace::CLOCKWISE,
        pso::FrontFace::CounterClockwise => vk::FrontFace::COUNTER_CLOCKWISE,
    }
}

pub fn map_comparison(fun: pso::Comparison) -> vk::CompareOp {
    use hal::pso::Comparison::*;
    match fun {
        Never => vk::CompareOp::NEVER,
        Less => vk::CompareOp::LESS,
        LessEqual => vk::CompareOp::LESS_OR_EQUAL,
        Equal => vk::CompareOp::EQUAL,
        GreaterEqual => vk::CompareOp::GREATER_OR_EQUAL,
        Greater => vk::CompareOp::GREATER,
        NotEqual => vk::CompareOp::NOT_EQUAL,
        Always => vk::CompareOp::ALWAYS,
    }
}

pub fn map_stencil_op(op: pso::StencilOp) -> vk::StencilOp {
    use hal::pso::StencilOp::*;
    match op {
        Keep => vk::StencilOp::KEEP,
        Zero => vk::StencilOp::ZERO,
        Replace => vk::StencilOp::REPLACE,
        IncrementClamp => vk::StencilOp::INCREMENT_AND_CLAMP,
        IncrementWrap => vk::StencilOp::INCREMENT_AND_WRAP,
        DecrementClamp => vk::StencilOp::DECREMENT_AND_CLAMP,
        DecrementWrap => vk::StencilOp::DECREMENT_AND_WRAP,
        Invert => vk::StencilOp::INVERT,
    }
}

pub fn map_stencil_side(side: &pso::StencilFace) -> vk::StencilOpState {
    vk::StencilOpState {
        fail_op: map_stencil_op(side.op_fail),
        pass_op: map_stencil_op(side.op_pass),
        depth_fail_op: map_stencil_op(side.op_depth_fail),
        compare_op: map_comparison(side.fun),
        compare_mask: match side.mask_read {
            pso::State::Static(mr) => mr,
            pso::State::Dynamic => !0,
        },
        write_mask: match side.mask_write {
            pso::State::Static(mw) => mw,
            pso::State::Dynamic => !0,
        },
        reference: match side.reference {
            pso::State::Static(r) => r,
            pso::State::Dynamic => 0,
        },
    }
}

pub fn map_blend_factor(factor: pso::Factor) -> vk::BlendFactor {
    use hal::pso::Factor::*;
    match factor {
        Zero => vk::BlendFactor::ZERO,
        One => vk::BlendFactor::ONE,
        SrcColor => vk::BlendFactor::SRC_COLOR,
        OneMinusSrcColor => vk::BlendFactor::ONE_MINUS_SRC_COLOR,
        DstColor => vk::BlendFactor::DST_COLOR,
        OneMinusDstColor => vk::BlendFactor::ONE_MINUS_DST_COLOR,
        SrcAlpha => vk::BlendFactor::SRC_ALPHA,
        OneMinusSrcAlpha => vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        DstAlpha => vk::BlendFactor::DST_ALPHA,
        OneMinusDstAlpha => vk::BlendFactor::ONE_MINUS_DST_ALPHA,
        ConstColor => vk::BlendFactor::CONSTANT_COLOR,
        OneMinusConstColor => vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR,
        ConstAlpha => vk::BlendFactor::CONSTANT_ALPHA,
        OneMinusConstAlpha => vk::BlendFactor::ONE_MINUS_CONSTANT_ALPHA,
        SrcAlphaSaturate => vk::BlendFactor::SRC_ALPHA_SATURATE,
        Src1Color => vk::BlendFactor::SRC1_COLOR,
        OneMinusSrc1Color => vk::BlendFactor::ONE_MINUS_SRC1_COLOR,
        Src1Alpha => vk::BlendFactor::SRC1_ALPHA,
        OneMinusSrc1Alpha => vk::BlendFactor::ONE_MINUS_SRC1_ALPHA,
    }
}

pub fn map_blend_op(operation: pso::BlendOp) -> (vk::BlendOp, vk::BlendFactor, vk::BlendFactor) {
    use hal::pso::BlendOp::*;
    match operation {
        Add { src, dst } => (
            vk::BlendOp::ADD,
            map_blend_factor(src),
            map_blend_factor(dst),
        ),
        Sub { src, dst } => (
            vk::BlendOp::SUBTRACT,
            map_blend_factor(src),
            map_blend_factor(dst),
        ),
        RevSub { src, dst } => (
            vk::BlendOp::REVERSE_SUBTRACT,
            map_blend_factor(src),
            map_blend_factor(dst),
        ),
        Min => (
            vk::BlendOp::MIN,
            vk::BlendFactor::ZERO,
            vk::BlendFactor::ZERO,
        ),
        Max => (
            vk::BlendOp::MAX,
            vk::BlendFactor::ZERO,
            vk::BlendFactor::ZERO,
        ),
    }
}

pub fn map_pipeline_statistics(
    statistics: query::PipelineStatistic,
) -> vk::QueryPipelineStatisticFlags {
    vk::QueryPipelineStatisticFlags::from_raw(statistics.bits())
}

pub fn map_query_control_flags(flags: query::ControlFlags) -> vk::QueryControlFlags {
    // Safe due to equivalence of HAL values and Vulkan values
    vk::QueryControlFlags::from_raw(flags.bits() & vk::QueryControlFlags::all().as_raw())
}

pub fn map_query_result_flags(flags: query::ResultFlags) -> vk::QueryResultFlags {
    vk::QueryResultFlags::from_raw(flags.bits() & vk::QueryResultFlags::all().as_raw())
}

pub fn map_image_features(features: vk::FormatFeatureFlags) -> format::ImageFeature {
    format::ImageFeature::from_bits_truncate(features.as_raw())
}

pub fn map_buffer_features(features: vk::FormatFeatureFlags) -> format::BufferFeature {
    format::BufferFeature::from_bits_truncate(features.as_raw())
}

pub fn map_device_features(features: Features) -> vk::PhysicalDeviceFeatures {
    // vk::PhysicalDeviceFeatures is a struct composed of Bool32's while
    // Features is a bitfield so we need to map everything manually
    vk::PhysicalDeviceFeatures::builder()
        .robust_buffer_access(features.contains(Features::ROBUST_BUFFER_ACCESS))
        .full_draw_index_uint32(features.contains(Features::FULL_DRAW_INDEX_U32))
        .image_cube_array(features.contains(Features::IMAGE_CUBE_ARRAY))
        .independent_blend(features.contains(Features::INDEPENDENT_BLENDING))
        .geometry_shader(features.contains(Features::GEOMETRY_SHADER))
        .tessellation_shader(features.contains(Features::TESSELLATION_SHADER))
        .sample_rate_shading(features.contains(Features::SAMPLE_RATE_SHADING))
        .dual_src_blend(features.contains(Features::DUAL_SRC_BLENDING))
        .logic_op(features.contains(Features::LOGIC_OP))
        .multi_draw_indirect(features.contains(Features::MULTI_DRAW_INDIRECT))
        .draw_indirect_first_instance(features.contains(Features::DRAW_INDIRECT_FIRST_INSTANCE))
        .depth_clamp(features.contains(Features::DEPTH_CLAMP))
        .depth_bias_clamp(features.contains(Features::DEPTH_BIAS_CLAMP))
        .fill_mode_non_solid(features.contains(Features::NON_FILL_POLYGON_MODE))
        .depth_bounds(features.contains(Features::DEPTH_BOUNDS))
        .wide_lines(features.contains(Features::LINE_WIDTH))
        .large_points(features.contains(Features::POINT_SIZE))
        .alpha_to_one(features.contains(Features::ALPHA_TO_ONE))
        .multi_viewport(features.contains(Features::MULTI_VIEWPORTS))
        .sampler_anisotropy(features.contains(Features::SAMPLER_ANISOTROPY))
        .texture_compression_etc2(features.contains(Features::FORMAT_ETC2))
        .texture_compression_astc_ldr(features.contains(Features::FORMAT_ASTC_LDR))
        .texture_compression_bc(features.contains(Features::FORMAT_BC))
        .occlusion_query_precise(features.contains(Features::PRECISE_OCCLUSION_QUERY))
        .pipeline_statistics_query(features.contains(Features::PIPELINE_STATISTICS_QUERY))
        .vertex_pipeline_stores_and_atomics(features.contains(Features::VERTEX_STORES_AND_ATOMICS))
        .fragment_stores_and_atomics(features.contains(Features::FRAGMENT_STORES_AND_ATOMICS))
        .shader_tessellation_and_geometry_point_size(
            features.contains(Features::SHADER_TESSELLATION_AND_GEOMETRY_POINT_SIZE),
        )
        .shader_image_gather_extended(features.contains(Features::SHADER_IMAGE_GATHER_EXTENDED))
        .shader_storage_image_extended_formats(
            features.contains(Features::SHADER_STORAGE_IMAGE_EXTENDED_FORMATS),
        )
        .shader_storage_image_multisample(
            features.contains(Features::SHADER_STORAGE_IMAGE_MULTISAMPLE),
        )
        .shader_storage_image_read_without_format(
            features.contains(Features::SHADER_STORAGE_IMAGE_READ_WITHOUT_FORMAT),
        )
        .shader_storage_image_write_without_format(
            features.contains(Features::SHADER_STORAGE_IMAGE_WRITE_WITHOUT_FORMAT),
        )
        .shader_uniform_buffer_array_dynamic_indexing(
            features.contains(Features::SHADER_UNIFORM_BUFFER_ARRAY_DYNAMIC_INDEXING),
        )
        .shader_sampled_image_array_dynamic_indexing(
            features.contains(Features::SHADER_SAMPLED_IMAGE_ARRAY_DYNAMIC_INDEXING),
        )
        .shader_storage_buffer_array_dynamic_indexing(
            features.contains(Features::SHADER_STORAGE_BUFFER_ARRAY_DYNAMIC_INDEXING),
        )
        .shader_storage_image_array_dynamic_indexing(
            features.contains(Features::SHADER_STORAGE_IMAGE_ARRAY_DYNAMIC_INDEXING),
        )
        .shader_clip_distance(features.contains(Features::SHADER_CLIP_DISTANCE))
        .shader_cull_distance(features.contains(Features::SHADER_CULL_DISTANCE))
        .shader_float64(features.contains(Features::SHADER_FLOAT64))
        .shader_int64(features.contains(Features::SHADER_INT64))
        .shader_int16(features.contains(Features::SHADER_INT16))
        .shader_resource_residency(features.contains(Features::SHADER_RESOURCE_RESIDENCY))
        .shader_resource_min_lod(features.contains(Features::SHADER_RESOURCE_MIN_LOD))
        .sparse_binding(features.contains(Features::SPARSE_BINDING))
        .sparse_residency_buffer(features.contains(Features::SPARSE_RESIDENCY_BUFFER))
        .sparse_residency_image2_d(features.contains(Features::SPARSE_RESIDENCY_IMAGE_2D))
        .sparse_residency_image3_d(features.contains(Features::SPARSE_RESIDENCY_IMAGE_3D))
        .sparse_residency2_samples(features.contains(Features::SPARSE_RESIDENCY_2_SAMPLES))
        .sparse_residency4_samples(features.contains(Features::SPARSE_RESIDENCY_4_SAMPLES))
        .sparse_residency8_samples(features.contains(Features::SPARSE_RESIDENCY_8_SAMPLES))
        .sparse_residency16_samples(features.contains(Features::SPARSE_RESIDENCY_16_SAMPLES))
        .sparse_residency_aliased(features.contains(Features::SPARSE_RESIDENCY_ALIASED))
        .variable_multisample_rate(features.contains(Features::VARIABLE_MULTISAMPLE_RATE))
        .inherited_queries(features.contains(Features::INHERITED_QUERIES))
        .build()
}

pub fn map_memory_ranges<'a, I, R>(ranges: I) -> Vec<vk::MappedMemoryRange>
where
    I: IntoIterator,
    I::Item: Borrow<(&'a n::Memory, R)>,
    R: RangeArg<u64>,
{
    ranges
        .into_iter()
        .map(|range| {
            let &(ref memory, ref range) = range.borrow();
            let (offset, size) = map_range_arg(range);
            vk::MappedMemoryRange {
                s_type: vk::StructureType::MAPPED_MEMORY_RANGE,
                p_next: ptr::null(),
                memory: memory.raw,
                offset,
                size,
            }
        })
        .collect()
}

/// Returns (offset, size) of the range.
///
/// Unbound start indices will be mapped to 0.
/// Unbound end indices will be mapped to VK_WHOLE_SIZE.
pub fn map_range_arg<R>(range: &R) -> (u64, u64)
where
    R: RangeArg<u64>,
{
    let offset = *range.start().unwrap_or(&0);
    let size = match range.end() {
        Some(end) => end - offset,
        None => vk::WHOLE_SIZE,
    };

    (offset, size)
}

pub fn map_command_buffer_flags(flags: command::CommandBufferFlags) -> vk::CommandBufferUsageFlags {
    // Safe due to equivalence of HAL values and Vulkan values
    vk::CommandBufferUsageFlags::from_raw(flags.bits())
}

pub fn map_command_buffer_level(level: command::RawLevel) -> vk::CommandBufferLevel {
    match level {
        command::RawLevel::Primary => vk::CommandBufferLevel::PRIMARY,
        command::RawLevel::Secondary => vk::CommandBufferLevel::SECONDARY,
    }
}

pub fn map_view_kind(
    kind: image::ViewKind,
    ty: vk::ImageType,
    is_cube: bool,
) -> Option<vk::ImageViewType> {
    use image::ViewKind::*;
    use vk::ImageType;

    Some(match (ty, kind) {
        (ImageType::TYPE_1D, D1) => vk::ImageViewType::TYPE_1D,
        (ImageType::TYPE_1D, D1Array) => vk::ImageViewType::TYPE_1D_ARRAY,
        (ImageType::TYPE_2D, D2) => vk::ImageViewType::TYPE_2D,
        (ImageType::TYPE_2D, D2Array) => vk::ImageViewType::TYPE_2D_ARRAY,
        (ImageType::TYPE_3D, D3) => vk::ImageViewType::TYPE_3D,
        (ImageType::TYPE_2D, Cube) if is_cube => vk::ImageViewType::CUBE,
        (ImageType::TYPE_2D, CubeArray) if is_cube => vk::ImageViewType::CUBE_ARRAY,
        (ImageType::TYPE_3D, Cube) if is_cube => vk::ImageViewType::CUBE,
        (ImageType::TYPE_3D, CubeArray) if is_cube => vk::ImageViewType::CUBE_ARRAY,
        _ => return None,
    })
}

pub fn map_rect(rect: &pso::Rect) -> vk::Rect2D {
    vk::Rect2D {
        offset: vk::Offset2D {
            x: rect.x as _,
            y: rect.y as _,
        },
        extent: vk::Extent2D {
            width: rect.w as _,
            height: rect.h as _,
        },
    }
}

pub fn map_clear_rect(rect: &pso::ClearRect) -> vk::ClearRect {
    vk::ClearRect {
        base_array_layer: rect.layers.start as _,
        layer_count: (rect.layers.end - rect.layers.start) as _,
        rect: map_rect(&rect.rect),
    }
}

pub fn map_viewport(vp: &pso::Viewport) -> vk::Viewport {
    vk::Viewport {
        x: vp.rect.x as _,
        y: vp.rect.y as _,
        width: vp.rect.w as _,
        height: vp.rect.h as _,
        min_depth: vp.depth.start,
        max_depth: vp.depth.end,
    }
}

pub fn map_view_capabilities(caps: image::ViewCapabilities) -> vk::ImageCreateFlags {
    vk::ImageCreateFlags::from_raw(caps.bits())
}

pub fn map_present_mode(mode: PresentMode) -> vk::PresentModeKHR {
    vk::PresentModeKHR::from_raw(mode as i32)
}

pub fn map_vk_present_mode(mode: vk::PresentModeKHR) -> PresentMode {
    unsafe { mem::transmute(mode) }
}

pub fn map_composite_alpha(composite_alpha: CompositeAlpha) -> vk::CompositeAlphaFlagsKHR {
    vk::CompositeAlphaFlagsKHR::from_raw(composite_alpha.bits())
}

pub fn map_vk_composite_alpha(composite_alpha: vk::CompositeAlphaFlagsKHR) -> CompositeAlpha {
    CompositeAlpha::from_bits_truncate(composite_alpha.as_raw())
}

pub fn map_descriptor_pool_create_flags(
    flags: pso::DescriptorPoolCreateFlags,
) -> vk::DescriptorPoolCreateFlags {
    vk::DescriptorPoolCreateFlags::from_raw(flags.bits())
}
