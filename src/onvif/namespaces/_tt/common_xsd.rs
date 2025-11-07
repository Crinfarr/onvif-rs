use serde::Deserialize;
/*
<!--===============================-->
<!--         Generic Types         -->
<!--===============================-->
*/
/** Unique identifier for a physical or logical resource.
 * Tokens should be assigned such that they are unique within a device. Tokens must be at least unique within its class.
 * Length up to 64 characters. Token may be extended by intermediate terminal with adding prefix to make it global unique.
 * The length should be within 36 characters for generating at local device. See "Remote Token" section in Resource Query specification.
 */

#[derive(Deserialize)]
#[allow(unused)]
pub struct ReferenceToken(String);
///Range of values greater equal Min value and less equal Max value.
#[allow(unused)]
#[derive(Deserialize)]
pub struct IntRange {
    #[serde(rename = "@Min")]
    pub min: isize,
    #[serde(rename = "@Max")]
    pub max: isize,
}
/*
<!--===============================-->
<!--    Start PTZ Related Types    -->
<!--===============================-->
*/
#[allow(unused)]
#[derive(Deserialize)]
pub struct Vector2D {
    #[serde(rename = "@x")]
    pub x: f32,
    #[serde(rename = "@y")]
    pub y: f32,
    /**
      Pan/tilt coordinate space selector. The following options are defined:
    * http://www.onvif.org/ver10/tptz/PanTiltSpaces/PositionGenericSpace
    * http://www.onvif.org/ver10/tptz/PanTiltSpaces/TranslationGenericSpace
    * http://www.onvif.org/ver10/tptz/PanTiltSpaces/VelocityGenericSpace
    * http://www.onvif.org/ver10/tptz/PanTiltSpaces/GenericSpeedSpace
    */
    #[serde(rename = "@space")]
    pub space: Option<String>,
}

#[allow(unused)]
#[derive(Deserialize)]
pub struct Vector1D {
    #[serde(rename = "@x")]
    pub x: f32,
    /**
    Zoom coordinate space selector. The following options are defined:
    * http://www.onvif.org/ver10/tptz/ZoomSpaces/PositionGenericSpace
    * http://www.onvif.org/ver10/tptz/ZoomSpaces/TranslationGenericSpace
    * http://www.onvif.org/ver10/tptz/ZoomSpaces/VelocityGenericSpace
    * http://www.onvif.org/ver10/tptz/ZoomSpaces/ZoomGenericSpeedSpace
    */
    #[serde(rename = "@space")]
    pub space: Option<String>,
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct PTZVector {
    ///Pan and tilt position. The x component corresponds to pan and the y component to tilt.
    #[serde(rename = "PanTilt")]
    pub pan_tilt: self::Vector2D,
    ///A zoom position.
    #[serde(rename = "Zoom")]
    pub zoom: self::Vector1D,
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct FieldOfView {
    ///Horizontal field-of-view in degrees.
    #[serde(rename = "@hfov")]
    pub h_fov: f32,
    ///Vertical field-of-view in degrees.
    #[serde(rename = "@vfov")]
    pub v_fov: f32,
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct PTZStatus {
    ///Specifies the absolute position of the PTZ unit together with the Space references. The default absolute spaces of the corresponding PTZ configuration MUST be referenced within the Position element.
    #[serde(rename = "Position")]
    pub position: PTZVector,
    ///Indicates if the Pan/Tilt/Zoom device unit is currently moving, idle or in an unknown state.
    #[serde(rename = "MoveStatus")]
    pub move_status: PTZMoveStatus,
    ///States a current PTZ error
    #[serde(rename = "Error")]
    pub error: String,
    ///Specifies the UTC time when this status was generated
    #[serde(rename = "UtcTime")]
    pub utc_time: String, //could deserialize to Chrono but i dont wanna add more deps. maybe feature
    ///States the current field of view of the video stream
    #[serde(rename = "FieldOfView")]
    pub field_of_view: FieldOfView,
    #[serde(rename = "#text")]
    content: String,
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct PTZMoveStatus {
    #[serde(rename = "PanTilt")]
    pub pan_tilt: MoveStatus,
    #[serde(rename = "Zoom")]
    pub zoom: MoveStatus,
}
#[allow(unused)]
#[derive(Deserialize)]
pub enum MoveStatus {
    IDLE,
    MOVING,
    UNKNOWN,
}
/*
<!--===============================-->
<!--  Event and Analytics Types    -->
<!--===============================-->
*/
#[allow(unused)]
#[derive(Deserialize)]
pub struct Vector {
    #[serde(rename = "@x")]
    pub x: f32,
    #[serde(rename = "@y")]
    pub y: f32,
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct Rectangle {
    #[serde(rename = "@bottom")]
    pub bottom: f32,
    #[serde(rename = "@top")]
    pub top: f32,
    #[serde(rename = "@right")]
    pub right: f32,
    #[serde(rename = "@left")]
    pub left: f32,
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct Polygon {
    #[serde(rename = "Point")]
    points: Vec<Vector>,
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct Color {
    #[serde(rename = "@X")]
    pub x: f32,
    #[serde(rename = "@Y")]
    pub y: f32,
    #[serde(rename = "@Z")]
    pub z: f32,
    #[serde(rename = "Colorspace")]
    /**
       Acceptable values:
       * http://www.onvif.org/ver10/colorspace/YCbCr - YCbCr
           - X attribute = Y value
           - Y attribute = Cb value
           - Z attribute = Cr value
       * http://www.onvif.org/ver10/colorspace/RGB - RGB
           - X attribute = R value
           - Y attribute = G value
           - Z attribute = B value
    */
    pub color_space: String,
    #[serde(rename = "Likelihood")]
    ///Likelihood that the color is correct.
    pub confidence: f32,
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct ColorCovariance {
    #[serde(rename = "@XX")]
    pub xx: f32,
    #[serde(rename = "@YY")]
    pub yy: f32,
    #[serde(rename = "@ZZ")]
    pub zz: f32,
    #[serde(rename = "@XY")]
    pub xy: Option<f32>,
    #[serde(rename = "@XZ")]
    pub xz: Option<f32>,
    #[serde(rename = "@YZ")]
    pub yz: Option<f32>,
    #[serde(rename = "Colorspace")]
    ///See [Color]/color_space
    pub color_space: String,
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct ColorDescriptor {
    #[serde(rename = "ColorCluster")]
    pub color_cluster: Vec<ColorClusterItem>,
    #[serde(rename = "Extension")]
    #[serde(skip)]
    _ext: Option<u8>, //idk if this is used or does anything but i skip it
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct ColorClusterItem {
    #[serde(rename = "Color")]
    pub color: Color,
    #[serde(rename = "Weight")]
    pub weight: Option<f32>,
    #[serde(rename = "Covariance")]
    pub covariance: ColorCovariance,
    #[serde(rename = "#text")]
    content: String,
}
/*
<!--===============================-->
<!--       Scene Description Types -->
<!--===============================-->
*/
#[allow(unused)]
#[derive(Deserialize)]
pub struct Transformation {
    #[serde(rename = "Translate")]
    pub translate:Option<Vector>,
    #[serde(rename = "Scale")]
    pub scale:Option<Vector>,
    #[serde(rename = "Extension")]
    pub extension:Option<TransformationExtension>
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct TransformationExtension {
    #[serde(rename = "#text")]
    content:String,
}
/*
<!--===============================-->
<!--  Location/Orientation Types   -->
<!--===============================-->
 */
#[allow(unused)]
#[derive(Deserialize)]
pub struct GeoLocation {
    #[serde(rename = "@lon")]
    ///East west location as angle.
    pub longitude:f64,
    #[serde(rename = "@lat")]
    ///North south location as angle.
    pub latitude:f64,
    #[serde(rename = "@elevation")]
    ///Hight in meters above sea level.
    pub elevation:f64,
    #[serde(rename = "#text")]
    content:String
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct GeoOrientation {
    #[serde(rename = "@roll")]
    pub roll:f32,
    #[serde(rename = "@pitch")]
    pub pitch:f32,
    #[serde(rename = "@yaw")]
    pub yaw:f32,
    #[serde(rename = "#text")]
    content:String,
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct LocalLocation {
    #[serde(rename = "@x")]
    pub x:f32,
    #[serde(rename = "@y")]
    pub y:f32,
    #[serde(rename = "@z")]
    pub z:f32,
    #[serde(rename = "#text")]
    content:String
}
#[allow(unused)]
#[derive(Deserialize)]
pub struct LocalOrientation {
    #[serde(rename = "@pan")]
    pub pan:f32,
    #[serde(rename = "@tilt")]
    pub tilt:f32,
    #[serde(rename = "@zoom")]
    pub zoom:f32,
    #[serde(rename = "#text")]
    content:String
}
//TODO
