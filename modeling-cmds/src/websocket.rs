//! Types for the websocket server.

use std::{borrow::Cow, collections::HashMap};

use parse_display_derive::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "slog")]
use slog::{Record, Serializer, KV};
use uuid::Uuid;

use crate::{
    id::ModelingCmdId,
    ok_response::OkModelingCmdResponse,
    shared::{EngineErrorCode, ExportFile},
    ModelingCmd,
};

/// The type of error sent by the KittyCAD API.
#[derive(Display, FromStr, Copy, Eq, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    /// Graphics engine failed to complete request, consider retrying
    InternalEngine,
    /// API failed to complete request, consider retrying
    InternalApi,
    /// User requested something geometrically or graphically impossible.
    /// Don't retry this request, as it's inherently impossible. Instead, read the error message
    /// and change your request.
    BadRequest,
    /// Auth token is missing from the request
    AuthTokenMissing,
    /// Auth token is invalid in some way (expired, incorrect format, etc)
    AuthTokenInvalid,
    /// Client sent invalid JSON.
    InvalidJson,
    /// Client sent invalid BSON.
    InvalidBson,
    /// Client sent a message which is not accepted over this protocol.
    WrongProtocol,
    /// Problem sending data between client and KittyCAD API.
    ConnectionProblem,
    /// Client sent a Websocket message type which the KittyCAD API does not handle.
    MessageTypeNotAccepted,
    /// Client sent a Websocket message intended for WebRTC but it was configured as a WebRTC
    /// connection.
    MessageTypeNotAcceptedForWebRTC,
}

/// Because [`EngineErrorCode`] is a subset of [`ErrorCode`], you can trivially map
/// each variant of the former to a variant of the latter.
impl From<EngineErrorCode> for ErrorCode {
    fn from(value: EngineErrorCode) -> Self {
        match value {
            EngineErrorCode::InternalEngine => Self::InternalEngine,
            EngineErrorCode::BadRequest => Self::BadRequest,
        }
    }
}

/// A graphics command submitted to the KittyCAD engine via the Modeling API.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "derive-jsonschema-on-enums", derive(schemars::JsonSchema))]
pub struct ModelingCmdReq {
    /// Which command to submit to the Kittycad engine.
    pub cmd: ModelingCmd,
    /// ID of command being submitted.
    pub cmd_id: ModelingCmdId,
}

/// The websocket messages the server receives.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "derive-jsonschema-on-enums", derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WebSocketRequest {
    /// The trickle ICE candidate request.
    // We box these to avoid a huge size difference between variants.
    TrickleIce {
        /// Information about the ICE candidate.
        candidate: Box<RtcIceCandidateInit>,
    },
    /// The SDP offer request.
    SdpOffer {
        /// The session description.
        offer: Box<RtcSessionDescription>,
    },
    /// The modeling command request.
    ModelingCmdReq(ModelingCmdReq),
    /// A sequence of modeling requests. If any request fails, following requests will not be tried.
    ModelingCmdBatchReq(ModelingBatch),
    /// The client-to-server Ping to ensure the WebSocket stays alive.
    Ping {},

    /// The response to a metrics collection request from the server.
    MetricsResponse {
        /// Collected metrics from the Client's end of the engine connection.
        metrics: Box<ClientMetrics>,
    },

    /// Return information about the connected instance
    Debug {},

    /// Authentication header request.
    Headers {
        /// The authentication header.
        headers: HashMap<String, String>,
    },
}

/// A sequence of modeling requests. If any request fails, following requests will not be tried.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "derive-jsonschema-on-enums", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub struct ModelingBatch {
    /// A sequence of modeling requests. If any request fails, following requests will not be tried.
    pub requests: Vec<ModelingCmdReq>,
    /// ID of batch being submitted.
    /// Each request has their own individual ModelingCmdId, but this is the
    /// ID of the overall batch.
    pub batch_id: ModelingCmdId,
    /// If false or omitted, responses to each batch command will just be Ok(()).
    /// If true, responses will be the actual response data for that modeling command.
    #[serde(default)]
    pub responses: bool,
}

impl std::default::Default for ModelingBatch {
    /// Creates a batch with 0 requests and a random ID.
    fn default() -> Self {
        Self {
            requests: Default::default(),
            batch_id: Uuid::new_v4().into(),
            responses: false,
        }
    }
}

impl ModelingBatch {
    /// Add a new modeling command to the end of this batch.
    pub fn push(&mut self, req: ModelingCmdReq) {
        self.requests.push(req);
    }

    /// Are there any requests in the batch?
    pub fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }
}

/// Representation of an ICE server used for STUN/TURN
/// Used to initiate WebRTC connections
/// based on <https://developer.mozilla.org/en-US/docs/Web/API/RTCIceServer>
#[derive(serde::Serialize, serde::Deserialize, Debug, JsonSchema, Clone)]
pub struct IceServer {
    /// URLs for a given STUN/TURN server.
    /// IceServer urls can either be a string or an array of strings
    /// But, we choose to always convert to an array of strings for consistency
    pub urls: Vec<String>,
    /// Credentials for a given TURN server.
    pub credential: Option<String>,
    /// Username for a given TURN server.
    pub username: Option<String>,
}

/// The websocket messages this server sends.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "derive-jsonschema-on-enums", derive(schemars::JsonSchema))]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OkWebSocketResponseData {
    /// Information about the ICE servers.
    IceServerInfo {
        /// Information about the ICE servers.
        ice_servers: Vec<IceServer>,
    },
    /// The trickle ICE candidate response.
    // We box these to avoid a huge size difference between variants.
    TrickleIce {
        /// Information about the ICE candidate.
        candidate: Box<RtcIceCandidateInit>,
    },
    /// The SDP answer response.
    SdpAnswer {
        /// The session description.
        answer: Box<RtcSessionDescription>,
    },
    /// The modeling command response.
    Modeling {
        /// The result of the command.
        modeling_response: OkModelingCmdResponse,
    },
    /// Response to a ModelingBatch.
    ModelingBatch {
        /// For each request in the batch,
        /// maps its ID to the request's outcome.
        responses: HashMap<ModelingCmdId, BatchResponse>,
    },
    /// The exported files.
    Export {
        /// The exported files
        files: Vec<RawFile>,
    },

    /// Request a collection of metrics, to include WebRTC.
    MetricsRequest {},

    /// Data about the Modeling Session (application-level).
    ModelingSessionData {
        /// Data about the Modeling Session (application-level).
        session: ModelingSessionData,
    },

    /// Pong response to a Ping message.
    Pong {},

    /// Information about the connected instance
    Debug {
        /// Instance name. This may or may not mean something.
        name: String,
    },
}

/// Successful Websocket response.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "derive-jsonschema-on-enums", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub struct SuccessWebSocketResponse {
    /// Always true
    pub success: bool,
    /// Which request this is a response to.
    /// If the request was a modeling command, this is the modeling command ID.
    /// If no request ID was sent, this will be null.
    pub request_id: Option<Uuid>,
    /// The data sent with a successful response.
    /// This will be flattened into a 'type' and 'data' field.
    pub resp: OkWebSocketResponseData,
}

/// Unsuccessful Websocket response.
#[derive(JsonSchema, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct FailureWebSocketResponse {
    /// Always false
    pub success: bool,
    /// Which request this is a response to.
    /// If the request was a modeling command, this is the modeling command ID.
    /// If no request ID was sent, this will be null.
    pub request_id: Option<Uuid>,
    /// The errors that occurred.
    pub errors: Vec<ApiError>,
}

/// Websocket responses can either be successful or unsuccessful.
/// Slightly different schemas in either case.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "derive-jsonschema-on-enums", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case", untagged)]
pub enum WebSocketResponse {
    /// Response sent when a request succeeded.
    Success(SuccessWebSocketResponse),
    /// Response sent when a request did not succeed.
    Failure(FailureWebSocketResponse),
}

/// Websocket responses can either be successful or unsuccessful.
/// Slightly different schemas in either case.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "derive-jsonschema-on-enums", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case", untagged)]
pub enum BatchResponse {
    /// Response sent when a request succeeded.
    Success {
        /// Response to the modeling command.
        response: OkModelingCmdResponse,
    },
    /// Response sent when a request did not succeed.
    Failure {
        /// Errors that occurred during the modeling command.
        errors: Vec<ApiError>,
    },
}

impl WebSocketResponse {
    /// Make a new success response.
    pub fn success(request_id: Option<Uuid>, resp: OkWebSocketResponseData) -> Self {
        Self::Success(SuccessWebSocketResponse {
            success: true,
            request_id,
            resp,
        })
    }

    /// Make a new failure response.
    pub fn failure(request_id: Option<Uuid>, errors: Vec<ApiError>) -> Self {
        Self::Failure(FailureWebSocketResponse {
            success: false,
            request_id,
            errors,
        })
    }

    /// Did the request succeed?
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(_))
    }

    /// Did the request fail?
    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Failure(_))
    }

    /// Get the ID of whichever request this response is for.
    pub fn request_id(&self) -> Option<Uuid> {
        match self {
            WebSocketResponse::Success(x) => x.request_id,
            WebSocketResponse::Failure(x) => x.request_id,
        }
    }
}

/// A raw file with unencoded contents to be passed over binary websockets.
/// When raw files come back for exports it is sent as binary/bson, not text/json.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct RawFile {
    /// The name of the file.
    pub name: String,
    /// The contents of the file.
    #[serde(
        serialize_with = "serde_bytes::serialize",
        deserialize_with = "serde_bytes::deserialize"
    )]
    pub contents: Vec<u8>,
}

impl From<ExportFile> for RawFile {
    fn from(f: ExportFile) -> Self {
        Self {
            name: f.name,
            contents: f.contents.0,
        }
    }
}

/// An error with an internal message for logging.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LoggableApiError {
    /// The error shown to users
    pub error: ApiError,
    /// The string logged internally
    pub msg_internal: Option<Cow<'static, str>>,
}

#[cfg(feature = "slog")]
impl KV for LoggableApiError {
    fn serialize(&self, _rec: &Record, serializer: &mut dyn Serializer) -> slog::Result {
        if let Some(ref msg_internal) = self.msg_internal {
            serializer.emit_str("msg_internal", msg_internal)?;
        }
        serializer.emit_str("msg_external", &self.error.message)?;
        serializer.emit_str("error_code", &self.error.error_code.to_string())
    }
}

/// An error.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Eq, PartialEq, Clone)]
pub struct ApiError {
    /// The error code.
    pub error_code: ErrorCode,
    /// The error message.
    pub message: String,
}

impl ApiError {
    /// Convert to a `LoggableApiError` with no internal message.
    pub fn no_internal_message(self) -> LoggableApiError {
        LoggableApiError {
            error: self,
            msg_internal: None,
        }
    }
    /// Add an internal log message to this error.
    pub fn with_message(self, msg_internal: Cow<'static, str>) -> LoggableApiError {
        LoggableApiError {
            error: self,
            msg_internal: Some(msg_internal),
        }
    }

    /// Should the internal error message be logged?
    pub fn should_log_internal_message(&self) -> bool {
        use ErrorCode as Code;
        match self.error_code {
            // Internal errors should always be logged, as they're problems with KittyCAD programming
            Code::InternalEngine | Code::InternalApi => true,
            // The user did something wrong, no need to log it, as there's nothing KittyCAD programmers can fix
            Code::MessageTypeNotAcceptedForWebRTC
            | Code::MessageTypeNotAccepted
            | Code::BadRequest
            | Code::WrongProtocol
            | Code::AuthTokenMissing
            | Code::AuthTokenInvalid
            | Code::InvalidBson
            | Code::InvalidJson => false,
            // In debug builds, log connection problems, otherwise don't.
            Code::ConnectionProblem => cfg!(debug_assertions),
        }
    }
}

/// Serde serializes Result into JSON as "Ok" and "Err", but we want "ok" and "err".
/// So, create a new enum that serializes as lowercase.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", rename = "SnakeCaseResult")]
pub enum SnakeCaseResult<T, E> {
    /// The result is Ok.
    Ok(T),
    /// The result is Err.
    Err(E),
}

impl<T, E> From<SnakeCaseResult<T, E>> for Result<T, E> {
    fn from(value: SnakeCaseResult<T, E>) -> Self {
        match value {
            SnakeCaseResult::Ok(x) => Self::Ok(x),
            SnakeCaseResult::Err(x) => Self::Err(x),
        }
    }
}

impl<T, E> From<Result<T, E>> for SnakeCaseResult<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(x) => Self::Ok(x),
            Err(x) => Self::Err(x),
        }
    }
}

/// ClientMetrics contains information regarding the state of the peer.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ClientMetrics {
    /// Counter of the number of WebRTC frames the client has dropped from the
    /// inbound video stream.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats-framesdropped
    pub rtc_frames_dropped: Option<u32>,

    /// Counter of the number of WebRTC frames that the client has decoded
    /// from the inbound video stream.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats-freezecount
    pub rtc_frames_decoded: Option<u64>,

    /// Counter of the number of WebRTC frames that the client has received
    /// from the inbound video stream.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats-freezecount
    pub rtc_frames_received: Option<u64>,

    /// Current number of frames being rendered in the last second. A good target
    /// is 60 frames per second, but it can fluctuate depending on network
    /// conditions.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats-freezecount
    pub rtc_frames_per_second: Option<u8>, // no way we're more than 255 fps :)

    /// Number of times the inbound video playback has frozen. This is usually due to
    /// network conditions.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats-freezecount
    pub rtc_freeze_count: Option<u32>,

    /// Amount of "jitter" in the inbound video stream. Network latency is the time
    /// it takes a packet to traverse the network. The amount that the latency
    /// varies is the jitter. Video latency is the time it takes to render
    /// a frame sent by the server (including network latency). A low jitter
    /// means the video latency can be reduced without impacting smooth
    /// playback. High jitter means clients will increase video latency to
    /// ensure smooth playback.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcreceivedrtpstreamstats-jitter
    pub rtc_jitter_sec: Option<f64>,

    /// Number of "key frames" decoded in the inbound h.264 stream. A
    /// key frame is an expensive (bandwidth-wise) "full image" of the video
    /// frame. Data after the keyframe become -- effectively -- "diff"
    /// operations on that key frame. The Engine will only send a keyframe if
    /// required, which is an indication that some of the "diffs" have been
    /// lost, usually an indication of poor network conditions. We like this
    /// metric to understand times when the connection has had to recover.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats-keyframesdecoded
    pub rtc_keyframes_decoded: Option<u32>,

    /// Number of seconds of frozen video the user has been subjected to.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats-totalfreezesduration
    pub rtc_total_freezes_duration_sec: Option<f32>,

    /// The height of the inbound video stream in pixels.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats-frameheight
    pub rtc_frame_height: Option<u32>,

    /// The width of the inbound video stream in pixels.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats-framewidth
    pub rtc_frame_width: Option<u32>,

    /// Amount of packets lost in the inbound video stream.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcreceivedrtpstreamstats-packetslost
    pub rtc_packets_lost: Option<u32>,

    ///  Count the total number of Picture Loss Indication (PLI) packets.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats-plicount
    pub rtc_pli_count: Option<u32>,

    /// Count of the total number of video pauses experienced by this receiver.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats-pausecount
    pub rtc_pause_count: Option<u32>,

    /// Count of the total number of video pauses experienced by this receiver.
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats-totalpausesduration
    pub rtc_total_pauses_duration_sec: Option<f32>,

    /// Total duration of pauses in seconds.
    ///
    /// This is the "ping" between the client and the STUN server. Not to be confused with the
    /// E2E RTT documented
    /// [here](https://www.w3.org/TR/webrtc-stats/#dom-rtcremoteinboundrtpstreamstats-roundtriptime)
    ///
    /// https://www.w3.org/TR/webrtc-stats/#dom-rtcicecandidatepairstats-currentroundtriptime
    pub rtc_stun_rtt_sec: Option<f32>,
}

/// ICECandidate represents a ice candidate
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RtcIceCandidate {
    /// The stats ID.
    pub stats_id: String,
    /// The foundation for the address.
    pub foundation: String,
    /// The priority of the candidate.
    pub priority: u32,
    /// The address of the candidate.
    pub address: String,
    /// The protocol used for the candidate.
    pub protocol: RtcIceProtocol,
    /// The port used for the candidate.
    pub port: u16,
    /// The type of the candidate.
    pub typ: RtcIceCandidateType,
    /// The component of the candidate.
    pub component: u16,
    /// The related address of the candidate.
    pub related_address: String,
    /// The related port of the candidate.
    pub related_port: u16,
    /// The TCP type of the candidate.
    pub tcp_type: String,
}

#[cfg(feature = "webrtc")]
impl From<webrtc::ice_transport::ice_candidate::RTCIceCandidate> for RtcIceCandidate {
    fn from(candidate: webrtc::ice_transport::ice_candidate::RTCIceCandidate) -> Self {
        Self {
            stats_id: candidate.stats_id,
            foundation: candidate.foundation,
            priority: candidate.priority,
            address: candidate.address,
            protocol: candidate.protocol.into(),
            port: candidate.port,
            typ: candidate.typ.into(),
            component: candidate.component,
            related_address: candidate.related_address,
            related_port: candidate.related_port,
            tcp_type: candidate.tcp_type,
        }
    }
}

#[cfg(feature = "webrtc")]
impl From<RtcIceCandidate> for webrtc::ice_transport::ice_candidate::RTCIceCandidate {
    fn from(candidate: RtcIceCandidate) -> Self {
        Self {
            stats_id: candidate.stats_id,
            foundation: candidate.foundation,
            priority: candidate.priority,
            address: candidate.address,
            protocol: candidate.protocol.into(),
            port: candidate.port,
            typ: candidate.typ.into(),
            component: candidate.component,
            related_address: candidate.related_address,
            related_port: candidate.related_port,
            tcp_type: candidate.tcp_type,
        }
    }
}

/// ICECandidateType represents the type of the ICE candidate used.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RtcIceCandidateType {
    /// Unspecified indicates that the candidate type is unspecified.
    #[default]
    Unspecified,

    /// ICECandidateTypeHost indicates that the candidate is of Host type as
    /// described in <https://tools.ietf.org/html/rfc8445#section-5.1.1.1>. A
    /// candidate obtained by binding to a specific port from an IP address on
    /// the host. This includes IP addresses on physical interfaces and logical
    /// ones, such as ones obtained through VPNs.
    Host,

    /// ICECandidateTypeSrflx indicates the the candidate is of Server
    /// Reflexive type as described
    /// <https://tools.ietf.org/html/rfc8445#section-5.1.1.2>. A candidate type
    /// whose IP address and port are a binding allocated by a NAT for an ICE
    /// agent after it sends a packet through the NAT to a server, such as a
    /// STUN server.
    Srflx,

    /// ICECandidateTypePrflx indicates that the candidate is of Peer
    /// Reflexive type. A candidate type whose IP address and port are a binding
    /// allocated by a NAT for an ICE agent after it sends a packet through the
    /// NAT to its peer.
    Prflx,

    /// ICECandidateTypeRelay indicates the the candidate is of Relay type as
    /// described in <https://tools.ietf.org/html/rfc8445#section-5.1.1.2>. A
    /// candidate type obtained from a relay server, such as a TURN server.
    Relay,
}

#[cfg(feature = "webrtc")]
impl From<webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType> for RtcIceCandidateType {
    fn from(candidate_type: webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType) -> Self {
        match candidate_type {
            webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType::Host => RtcIceCandidateType::Host,
            webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType::Srflx => RtcIceCandidateType::Srflx,
            webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType::Prflx => RtcIceCandidateType::Prflx,
            webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType::Relay => RtcIceCandidateType::Relay,
            webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType::Unspecified => {
                RtcIceCandidateType::Unspecified
            }
        }
    }
}

#[cfg(feature = "webrtc")]
impl From<RtcIceCandidateType> for webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType {
    fn from(candidate_type: RtcIceCandidateType) -> Self {
        match candidate_type {
            RtcIceCandidateType::Host => webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType::Host,
            RtcIceCandidateType::Srflx => webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType::Srflx,
            RtcIceCandidateType::Prflx => webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType::Prflx,
            RtcIceCandidateType::Relay => webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType::Relay,
            RtcIceCandidateType::Unspecified => {
                webrtc::ice_transport::ice_candidate_type::RTCIceCandidateType::Unspecified
            }
        }
    }
}

/// ICEProtocol indicates the transport protocol type that is used in the
/// ice.URL structure.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RtcIceProtocol {
    /// Unspecified indicates that the protocol is unspecified.
    #[default]
    Unspecified,

    /// UDP indicates the URL uses a UDP transport.
    Udp,

    /// TCP indicates the URL uses a TCP transport.
    Tcp,
}

#[cfg(feature = "webrtc")]
impl From<webrtc::ice_transport::ice_protocol::RTCIceProtocol> for RtcIceProtocol {
    fn from(protocol: webrtc::ice_transport::ice_protocol::RTCIceProtocol) -> Self {
        match protocol {
            webrtc::ice_transport::ice_protocol::RTCIceProtocol::Udp => RtcIceProtocol::Udp,
            webrtc::ice_transport::ice_protocol::RTCIceProtocol::Tcp => RtcIceProtocol::Tcp,
            webrtc::ice_transport::ice_protocol::RTCIceProtocol::Unspecified => RtcIceProtocol::Unspecified,
        }
    }
}

#[cfg(feature = "webrtc")]
impl From<RtcIceProtocol> for webrtc::ice_transport::ice_protocol::RTCIceProtocol {
    fn from(protocol: RtcIceProtocol) -> Self {
        match protocol {
            RtcIceProtocol::Udp => webrtc::ice_transport::ice_protocol::RTCIceProtocol::Udp,
            RtcIceProtocol::Tcp => webrtc::ice_transport::ice_protocol::RTCIceProtocol::Tcp,
            RtcIceProtocol::Unspecified => webrtc::ice_transport::ice_protocol::RTCIceProtocol::Unspecified,
        }
    }
}

/// ICECandidateInit is used to serialize ice candidates
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
// These HAVE to be camel case as per the RFC.
pub struct RtcIceCandidateInit {
    /// The candidate string associated with the object.
    pub candidate: String,
    /// The identifier of the "media stream identification" as defined in
    /// [RFC 8841](https://tools.ietf.org/html/rfc8841).
    pub sdp_mid: Option<String>,
    /// The index (starting at zero) of the m-line in the SDP this candidate is
    /// associated with.
    #[serde(rename = "sdpMLineIndex")]
    pub sdp_mline_index: Option<u16>,
    /// The username fragment (as defined in
    /// [RFC 8445](https://tools.ietf.org/html/rfc8445#section-5.2.1)) associated with the object.
    pub username_fragment: Option<String>,
}

#[cfg(feature = "webrtc")]
impl From<webrtc::ice_transport::ice_candidate::RTCIceCandidateInit> for RtcIceCandidateInit {
    fn from(candidate: webrtc::ice_transport::ice_candidate::RTCIceCandidateInit) -> Self {
        Self {
            candidate: candidate.candidate,
            sdp_mid: candidate.sdp_mid,
            sdp_mline_index: candidate.sdp_mline_index,
            username_fragment: candidate.username_fragment,
        }
    }
}

#[cfg(feature = "webrtc")]
impl From<RtcIceCandidateInit> for webrtc::ice_transport::ice_candidate::RTCIceCandidateInit {
    fn from(candidate: RtcIceCandidateInit) -> Self {
        Self {
            candidate: candidate.candidate,
            sdp_mid: candidate.sdp_mid,
            sdp_mline_index: candidate.sdp_mline_index,
            username_fragment: candidate.username_fragment,
        }
    }
}

/// SessionDescription is used to expose local and remote session descriptions.
#[derive(Default, Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RtcSessionDescription {
    /// SDP type.
    #[serde(rename = "type")]
    pub sdp_type: RtcSdpType,

    /// SDP string.
    pub sdp: String,
}

#[cfg(feature = "webrtc")]
impl From<webrtc::peer_connection::sdp::session_description::RTCSessionDescription> for RtcSessionDescription {
    fn from(desc: webrtc::peer_connection::sdp::session_description::RTCSessionDescription) -> Self {
        Self {
            sdp_type: desc.sdp_type.into(),
            sdp: desc.sdp,
        }
    }
}

#[cfg(feature = "webrtc")]
impl TryFrom<RtcSessionDescription> for webrtc::peer_connection::sdp::session_description::RTCSessionDescription {
    type Error = anyhow::Error;

    fn try_from(desc: RtcSessionDescription) -> Result<Self, Self::Error> {
        let result = match desc.sdp_type {
            RtcSdpType::Offer => {
                webrtc::peer_connection::sdp::session_description::RTCSessionDescription::offer(desc.sdp)?
            }
            RtcSdpType::Pranswer => {
                webrtc::peer_connection::sdp::session_description::RTCSessionDescription::pranswer(desc.sdp)?
            }
            RtcSdpType::Answer => {
                webrtc::peer_connection::sdp::session_description::RTCSessionDescription::answer(desc.sdp)?
            }
            RtcSdpType::Rollback => anyhow::bail!("Rollback is not supported"),
            RtcSdpType::Unspecified => anyhow::bail!("Unspecified is not supported"),
        };

        Ok(result)
    }
}

/// SDPType describes the type of an SessionDescription.
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RtcSdpType {
    /// Unspecified indicates that the type is unspecified.
    #[default]
    Unspecified = 0,

    /// indicates that a description MUST be treated as an SDP offer.
    Offer,

    /// indicates that a description MUST be treated as an
    /// SDP answer, but not a final answer. A description used as an SDP
    /// pranswer may be applied as a response to an SDP offer, or an update to
    /// a previously sent SDP pranswer.
    Pranswer,

    /// indicates that a description MUST be treated as an SDP
    /// final answer, and the offer-answer exchange MUST be considered complete.
    /// A description used as an SDP answer may be applied as a response to an
    /// SDP offer or as an update to a previously sent SDP pranswer.
    Answer,

    /// indicates that a description MUST be treated as
    /// canceling the current SDP negotiation and moving the SDP offer and
    /// answer back to what it was in the previous stable state. Note the
    /// local or remote SDP descriptions in the previous stable state could be
    /// null if there has not yet been a successful offer-answer negotiation.
    Rollback,
}

#[cfg(feature = "webrtc")]
impl From<webrtc::peer_connection::sdp::sdp_type::RTCSdpType> for RtcSdpType {
    fn from(sdp_type: webrtc::peer_connection::sdp::sdp_type::RTCSdpType) -> Self {
        match sdp_type {
            webrtc::peer_connection::sdp::sdp_type::RTCSdpType::Offer => Self::Offer,
            webrtc::peer_connection::sdp::sdp_type::RTCSdpType::Pranswer => Self::Pranswer,
            webrtc::peer_connection::sdp::sdp_type::RTCSdpType::Answer => Self::Answer,
            webrtc::peer_connection::sdp::sdp_type::RTCSdpType::Rollback => Self::Rollback,
            webrtc::peer_connection::sdp::sdp_type::RTCSdpType::Unspecified => Self::Unspecified,
        }
    }
}

#[cfg(feature = "webrtc")]
impl From<RtcSdpType> for webrtc::peer_connection::sdp::sdp_type::RTCSdpType {
    fn from(sdp_type: RtcSdpType) -> Self {
        match sdp_type {
            RtcSdpType::Offer => Self::Offer,
            RtcSdpType::Pranswer => Self::Pranswer,
            RtcSdpType::Answer => Self::Answer,
            RtcSdpType::Rollback => Self::Rollback,
            RtcSdpType::Unspecified => Self::Unspecified,
        }
    }
}
/// Successful Websocket response.
#[derive(JsonSchema, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ModelingSessionData {
    /// ID of the API call this modeling session is using.
    /// Useful for tracing and debugging.
    pub api_call_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output;

    const REQ_ID: Uuid = uuid::uuid!("cc30d5e2-482b-4498-b5d2-6131c30a50a4");

    #[test]
    fn serialize_websocket_modeling_ok() {
        let actual = WebSocketResponse::Success(SuccessWebSocketResponse {
            success: true,
            request_id: Some(REQ_ID),
            resp: OkWebSocketResponseData::Modeling {
                modeling_response: OkModelingCmdResponse::CurveGetControlPoints(output::CurveGetControlPoints {
                    control_points: vec![],
                }),
            },
        });
        let expected = serde_json::json!({
            "success": true,
            "request_id": "cc30d5e2-482b-4498-b5d2-6131c30a50a4",
            "resp": {
                "type": "modeling",
                "data": {
                    "modeling_response": {
                        "type": "curve_get_control_points",
                        "data": { "control_points": [] }
                    }
                }
            }
        });
        assert_json_eq(actual, expected);
    }

    #[test]
    fn serialize_websocket_webrtc_ok() {
        let actual = WebSocketResponse::Success(SuccessWebSocketResponse {
            success: true,
            request_id: Some(REQ_ID),
            resp: OkWebSocketResponseData::IceServerInfo { ice_servers: vec![] },
        });
        let expected = serde_json::json!({
            "success": true,
            "request_id": "cc30d5e2-482b-4498-b5d2-6131c30a50a4",
            "resp": {
                "type": "ice_server_info",
                "data": {
                    "ice_servers": []
                }
            }
        });
        assert_json_eq(actual, expected);
    }

    #[test]
    fn serialize_websocket_export_ok() {
        let actual = WebSocketResponse::Success(SuccessWebSocketResponse {
            success: true,
            request_id: Some(REQ_ID),
            resp: OkWebSocketResponseData::Export { files: vec![] },
        });
        let expected = serde_json::json!({
            "success": true,
            "request_id": "cc30d5e2-482b-4498-b5d2-6131c30a50a4",
            "resp": {
                "type": "export",
                "data": {"files": [] }
            }
        });
        assert_json_eq(actual, expected);
    }

    #[test]
    fn serialize_websocket_err() {
        let actual = WebSocketResponse::Failure(FailureWebSocketResponse {
            success: false,
            request_id: Some(REQ_ID),
            errors: vec![ApiError {
                error_code: ErrorCode::InternalApi,
                message: "you fucked up!".to_owned(),
            }],
        });
        let expected = serde_json::json!({
            "success": false,
            "request_id": "cc30d5e2-482b-4498-b5d2-6131c30a50a4",
            "errors": [
                {
                    "error_code": "internal_api",
                    "message": "you fucked up!"
                }
            ],
        });
        assert_json_eq(actual, expected);
    }

    #[test]
    fn serialize_websocket_metrics() {
        let actual = WebSocketRequest::MetricsResponse {
            metrics: Box::new(ClientMetrics {
                rtc_frames_dropped: Some(1),
                rtc_frames_decoded: Some(2),
                rtc_frames_per_second: Some(3),
                rtc_frames_received: Some(4),
                rtc_freeze_count: Some(5),
                rtc_jitter_sec: Some(6.7),
                rtc_keyframes_decoded: Some(8),
                rtc_total_freezes_duration_sec: Some(9.1),
                rtc_frame_height: Some(100),
                rtc_frame_width: Some(100),
                rtc_packets_lost: Some(0),
                rtc_pli_count: Some(0),
                rtc_pause_count: Some(0),
                rtc_total_pauses_duration_sec: Some(0.0),
                rtc_stun_rtt_sec: Some(0.005),
            }),
        };
        let expected = serde_json::json!({
            "type": "metrics_response",
            "metrics": {
                "rtc_frames_dropped": 1,
                "rtc_frames_decoded": 2,
                "rtc_frames_per_second": 3,
                "rtc_frames_received": 4,
                "rtc_freeze_count": 5,
                "rtc_jitter_sec": 6.7,
                "rtc_keyframes_decoded": 8,
                "rtc_total_freezes_duration_sec": 9.1,
                "rtc_frame_height": 100,
                "rtc_frame_width": 100,
                "rtc_packets_lost": 0,
                "rtc_pli_count": 0,
                "rtc_pause_count": 0,
                "rtc_total_pauses_duration_sec": 0.0,
                "rtc_stun_rtt_sec": 0.005,
            },
        });
        assert_json_eq(actual, expected);
    }

    fn assert_json_eq<T: Serialize>(actual: T, expected: serde_json::Value) {
        let json_str = serde_json::to_string(&actual).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(actual, expected, "got\n{actual:#}\n, expected\n{expected:#}\n");
    }
}
