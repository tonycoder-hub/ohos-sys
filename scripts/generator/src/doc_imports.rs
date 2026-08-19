//! `#[cfg(doc)]` imports so rustdoc can resolve intra-doc links to items
//! that live in other modules of the same crate.
//!
//! These imports are needed only while building documentation: the C
//! headers refer to types and functions by unqualified name, but the
//! bindings are split across files. See openharmony-rs/ohos-sys#71.

use bindgen::Builder;

pub(crate) fn apply_doc_imports(builder: Builder, output_rel: &str) -> Builder {
    let imports = doc_imports_for(output_rel);
    let mut builder = builder;
    for line in imports {
        builder = builder.raw_line(*line);
    }
    builder
}

fn doc_imports_for(output_rel: &str) -> &'static [&'static str] {
    match output_rel {
        "components/arkui/src/drag_and_drop/drag_and_drop_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::ui_input_event::ArkUI_ModifierKeyName;",
        ],
        "components/arkui/src/native_node/native_node_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::drag_and_drop::{ArkUI_DragEvent, ArkUI_PreDragStatus};",
            "#[cfg(doc)]",
            "use crate::drawable_descriptor::ArkUI_DrawableDescriptor;",
            "#[cfg(doc)]",
            "use crate::native_animate::ArkUI_TransitionEffect;",
            "#[cfg(doc)]",
            "use crate::styled_string::{ArkUI_StyledString, ArkUI_TextLayoutManager};",
            "#[cfg(doc)]",
            "use crate::ui_input_event::{ArkUI_CoastingAxisEvent, OH_ArkUI_UIInputEvent_GetCoastingAxisEvent};",
            "#[cfg(all(doc, feature = \"api-20\"))]",
            "use crate::native_render::ArkUI_RenderNodeClipOption;",
        ],
        "components/arkui/src/ui_input_event/ui_input_event_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::native_gesture::OH_ArkUI_GestureEvent_GetRawInputEvent;",
            "#[cfg(doc)]",
            "use crate::native_node::ArkUI_NodeEvent;",
            "#[cfg(all(doc, feature = \"api-14\"))]",
            "use crate::native_key_event::OH_ArkUI_KeyEvent_GetType;",
        ],
        "components/basic-services-kit/src/scan/scan_ffi.rs" => &[
            "#[cfg(all(doc, feature = \"print\"))]",
            "use crate::print::Print_ErrorCode;",
        ],
        "components/inputmethod/src/inputmethod_proxy/inputmethod_proxy_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::controller::OH_InputMethodController_Attach;",
        ],
        "components/inputmethod/src/text_editor_proxy/text_editor_proxy_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::controller::OH_InputMethodController_Attach;",
        ],
        "components/ipckit/src/cparcel/cparcel_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::error_code::OH_IPC_ErrorCode;",
        ],
        "components/ipckit/src/cremote_object/cremote_object_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::error_code::OH_IPC_ErrorCode;",
        ],
        "components/ipckit/src/cskeleton/cskeleton_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::error_code::OH_IPC_ErrorCode;",
        ],
        "components/multimedia/player_framework/src/avbuffer/avbuffer_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::avformat::OH_AVFormat_Destroy;",
        ],
        "components/multimedia/player_framework/src/avcodec_audiocodec/avcodec_audiocodec_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::avcodec_base::{OH_AVCodecOnNeedInputBuffer, OH_AVCodecOnNewOutputBuffer};",
            "#[cfg(doc)]",
            "use crate::avformat::OH_AVFormat_Destroy;",
        ],
        "components/multimedia/player_framework/src/avcodec_audiodecoder/avcodec_audiodecoder_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::avcodec_base::OH_AVCodecOnNeedInputData;",
        ],
        "components/multimedia/player_framework/src/avcodec_audioencoder/avcodec_audioencoder_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::avcodec_base::OH_AVCodecOnNeedInputData;",
        ],
        "components/multimedia/player_framework/src/avcodec_base/avcodec_base_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::avcodec_videodecoder::{",
            "    OH_VideoDecoder_Configure, OH_VideoDecoder_GetOutputDescription,",
            "};",
            "#[cfg(doc)]",
            "use crate::avcodec_videoencoder::{",
            "    OH_VideoEncodeBitrateMode, OH_VideoEncoder_GetInputDescription,",
            "    OH_VideoEncoder_OnNeedInputParameter,",
            "};",
            "#[cfg(doc)]",
            "use crate::avformat::{OH_AVFormat_GetIntValue, OH_AVPixelFormat};",
            "#[cfg(all(doc, feature = \"api-10\"))]",
            "use crate::avcapability::{",
            "    OH_AVCapability_GetFeatureProperties, OH_AVCapability_IsFeatureSupported,",
            "};",
            "#[cfg(all(doc, feature = \"api-11\"))]",
            "use crate::avbuffer::OH_AVBuffer_SetParameter;",
        ],
        "components/multimedia/player_framework/src/avcodec_videodecoder/avcodec_videodecoder_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::avcodec_base::{",
            "    OH_AVCodecOnNeedInputBuffer, OH_AVCodecOnNeedInputData, OH_AVCodecOnNewOutputBuffer,",
            "    OH_AVCodecOnNewOutputData,",
            "};",
        ],
        "components/multimedia/player_framework/src/avcodec_videoencoder/avcodec_videoencoder_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::avcodec_base::{",
            "    OH_AVCodecOnNeedInputBuffer, OH_AVCodecOnNeedInputData, OH_AVCodecOnNewOutputData,",
            "};",
        ],
        "components/multimedia/player_framework/src/averrors/averrors_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::avcodec_videodecoder::OH_VideoDecoder_GetOutputDescription;",
            "#[cfg(doc)]",
            "use crate::avcodec_videoencoder::OH_VideoEncoder_GetOutputDescription;",
            "#[cfg(all(doc, feature = \"api-11\"))]",
            "use crate::avcodec_audiocodec::OH_AudioCodec_GetOutputDescription;",
        ],
        "components/multimedia/player_framework/src/avmetadata_extractor/avmetadata_extractor_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::avformat::OH_AVFormat_Destroy;",
        ],
        "components/multimedia/player_framework/src/avmetadata_extractor_base/avmetadata_extractor_base_ffi.rs" => &[
            "#[cfg(all(doc, feature = \"api-18\"))]",
            "use crate::media_types::OH_Core_HdrType;",
        ],
        "components/multimedia/player_framework/src/avplayer_base/avplayer_base_ffi.rs" => &[
            "#[cfg(all(doc, feature = \"api-11\"))]",
            "use crate::avplayer::{OH_AVPlayer_GetMediaDescription, OH_AVPlayer_GetTrackDescription};",
        ],
        "components/multimedia/video_processing_engine/src/image_processing_types/image_processing_types_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::image_processing::{",
            "    OH_ImageProcessing_Create, OH_ImageProcessing_GetParameter,",
            "    OH_ImageProcessing_IsColorSpaceConversionSupported, OH_ImageProcessing_IsCompositionSupported,",
            "    OH_ImageProcessing_IsDecompositionSupported, OH_ImageProcessing_SetParameter,",
            "};",
            "#[cfg(doc)]",
            "use crate::video_processing::{OH_VideoProcessing_GetParameter, OH_VideoProcessing_SetParameter};",
        ],
        "components/multimedia/video_processing_engine/src/video_processing_types/video_processing_types_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::image_processing::OH_ImageProcessing_Create;",
            "#[cfg(doc)]",
            "use crate::video_processing::{",
            "    OH_VideoProcessingCallback_Create, OH_VideoProcessing_Create, OH_VideoProcessing_GetParameter,",
            "    OH_VideoProcessing_IsColorSpaceConversionSupported, OH_VideoProcessing_RegisterCallback,",
            "    OH_VideoProcessing_RenderOutputBuffer, OH_VideoProcessing_SetParameter,",
            "    OH_VideoProcessing_Start, OH_VideoProcessing_Stop,",
            "};",
        ],
        "components/ohaudio/src/audio_device_base/audio_device_base_ffi.rs" => &[
            "#[cfg(all(doc, feature = \"api-12\"))]",
            "use crate::audio_routing_manager::{",
            "    OH_AudioRoutingManager_GetDevices, OH_AudioRoutingManager_ReleaseDevices,",
            "};",
        ],
        "components/ohaudio/src/audio_resource_manager/audio_resource_manager_ffi.rs" => &[
            "#[cfg(all(doc, feature = \"api-12\"))]",
            "use crate::audio_routing_manager::OH_AudioManager_GetAudioRoutingManager;",
        ],
        "components/pasteboard/src/pasteboard/pasteboard_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::PASTEBOARD_ErrCode;",
        ],
        "components/rdb/src/data_asset/data_asset_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::relational_store_error_code::OH_Rdb_ErrCode;",
        ],
        "components/rdb/src/rdb_transaction/rdb_transaction_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::relational_store_error_code::OH_Rdb_ErrCode;",
        ],
        "components/rdb/src/relational_store/relational_store_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::rdb_types::OH_ColumnType;",
            "#[cfg(doc)]",
            "use crate::relational_store_error_code::OH_Rdb_ErrCode;",
        ],
        "components/udmf/src/udmf/udmf_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::Udmf_ErrCode;",
            "#[cfg(all(doc, feature = \"api-15\"))]",
            "use crate::Udmf_ListenerStatus;",
        ],
        "components/udmf/src/uds/uds_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::Udmf_ErrCode;",
        ],
        "components/xcomponent/src/xcomponent_arkui_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::xcomponent_result_ffi::{",
            "    OH_NATIVEXCOMPONENT_RESULT_BAD_PARAMETER, OH_NATIVEXCOMPONENT_RESULT_SUCCESS,",
            "};",
        ],
        "components/xcomponent/src/xcomponent_ffi.rs" => &[
            "#[cfg(all(doc, feature = \"arkui\"))]",
            "use arkui_sys::ui_input_event::ArkUI_ModifierKeyName;",
            "#[cfg(doc)]",
            "use crate::xcomponent_result_ffi::{",
            "    OH_NATIVEXCOMPONENT_RESULT_BAD_PARAMETER, OH_NATIVEXCOMPONENT_RESULT_SUCCESS,",
            "};",
        ],
        _ => &[],
    }
}
