use serde::Deserialize;

/*
<xs:simpleType name="ReferenceToken">
    <xs:annotation>
        <xs:documentation>Unique identifier for a physical or logical resource.
        Tokens should be assigned such that they are unique within a device. Tokens must be at least unique within its class.
        Length up to 64 characters. Token may be extended by intermediate terminal with adding prefix to make it global unique.
        The length should be within 36 characters for generating at local device. See "Remote Token" section in Resource Query specification.</xs:documentation>
    </xs:annotation>
    <xs:restriction base="xs:string">
        <xs:maxLength value="64"/>
    </xs:restriction>
</xs:simpleType>
 */
/** Unique identifier for a physical or logical resource.
 * Tokens should be assigned such that they are unique within a device. Tokens must be at least unique within its class.
 * Length up to 64 characters. Token may be extended by intermediate terminal with adding prefix to make it global unique.
 * The length should be within 36 characters for generating at local device. See "Remote Token" section in Resource Query specification.
 */

#[derive(Deserialize)]
#[allow(unused)]
pub struct ReferenceToken(String);
/*
<xs:complexType name="IntRange">
    <xs:annotation>
        <xs:documentation>Range of values greater equal Min value and less equal Max value.</xs:documentation>
    </xs:annotation>
    <xs:sequence>
        <xs:element name="Min" type="xs:int"/>
        <xs:element name="Max" type="xs:int"/>
    </xs:sequence>
</xs:complexType>
 */
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
<xs:complexType name="Vector2D">
    <xs:attribute name="x" type="xs:float" use="required"/>
    <xs:attribute name="y" type="xs:float" use="required"/>
    <xs:attribute name="space" type="xs:anyURI" use="optional">
        <xs:annotation>
            <xs:documentation>
            Pan/tilt coordinate space selector. The following options are defined:<ul>
                    <li> http://www.onvif.org/ver10/tptz/PanTiltSpaces/PositionGenericSpace</li>
                    <li> http://www.onvif.org/ver10/tptz/PanTiltSpaces/TranslationGenericSpace</li>
                    <li> http://www.onvif.org/ver10/tptz/PanTiltSpaces/VelocityGenericSpace</li>
                    <li> http://www.onvif.org/ver10/tptz/PanTiltSpaces/GenericSpeedSpace</li>
                </ul>
            </xs:documentation>
        </xs:annotation>
    </xs:attribute>
</xs:complexType>
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
/*
<xs:complexType name="Vector1D">
    <xs:attribute name="x" type="xs:float" use="required"/>
    <xs:attribute name="space" type="xs:anyURI" use="optional">
        <xs:annotation>
            <xs:documentation>
            Zoom coordinate space selector. The following options are defined:<ul style="">
                    <li> http://www.onvif.org/ver10/tptz/ZoomSpaces/PositionGenericSpace</li>
                    <li> http://www.onvif.org/ver10/tptz/ZoomSpaces/TranslationGenericSpace</li>
                    <li> http://www.onvif.org/ver10/tptz/ZoomSpaces/VelocityGenericSpace</li>
                    <li> http://www.onvif.org/ver10/tptz/ZoomSpaces/ZoomGenericSpeedSpace</li>
                </ul>
            </xs:documentation>
        </xs:annotation>
    </xs:attribute>
</xs:complexType>
 */
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
/*
<xs:complexType name="PTZVector">
    <xs:sequence>
        <xs:element name="PanTilt" type="tt:Vector2D" minOccurs="0">
            <xs:annotation>
                <xs:documentation>Pan and tilt position. The x component corresponds to pan and the y component to tilt.</xs:documentation>
            </xs:annotation>
        </xs:element>
        <xs:element name="Zoom" type="tt:Vector1D" minOccurs="0">
            <xs:annotation>
                <xs:documentation>
        A zoom position.
        </xs:documentation>
            </xs:annotation>
        </xs:element>
    </xs:sequence>
</xs:complexType>
 */
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
/*
<xs:complexType name="FieldOfView">
	<xs:attribute name="hfov" type="xs:float" use="required">
		<xs:annotation>
			<xs:documentation>Horizontal field-of-view in degrees.</xs:documentation>
		</xs:annotation>
	</xs:attribute>
	<xs:attribute name="vfov" type="xs:float" use="required">
		<xs:annotation>
			<xs:documentation>Vertical field-of-view in degrees.</xs:documentation>
		</xs:annotation>
	</xs:attribute>
</xs:complexType>
 */
#[allow(unused)]
#[derive(Deserialize)]
pub struct FieldOfView {
    ///Horizontal field-of-view in degrees.
    #[serde(rename = "@hfov")]
    pub h_fov:f32,
    ///Vertical field-of-view in degrees.
    #[serde(rename = "@vfov")]
    pub v_fov:f32
}
/*
<xs:complexType name="PTZStatus">
	<xs:sequence>
		<xs:element name="Position" type="tt:PTZVector" minOccurs="0">
			<xs:annotation>
				<xs:documentation>
        Specifies the absolute position of the PTZ unit together with the Space references. The default absolute spaces of the corresponding PTZ configuration MUST be referenced within the Position element.
        </xs:documentation>
			</xs:annotation>
		</xs:element>
		<xs:element name="MoveStatus" type="tt:PTZMoveStatus" minOccurs="0">
			<xs:annotation>
				<xs:documentation>
        Indicates if the Pan/Tilt/Zoom device unit is currently moving, idle or in an unknown state.
        </xs:documentation>
			</xs:annotation>
		</xs:element>
		<xs:element name="Error" type="xs:string" minOccurs="0">
			<xs:annotation>
				<xs:documentation>
        States a current PTZ error.
        </xs:documentation>
			</xs:annotation>
		</xs:element>
		<xs:element name="UtcTime" type="xs:dateTime">
			<xs:annotation>
				<xs:documentation>
        Specifies the UTC time when this status was generated.
        </xs:documentation>
			</xs:annotation>
		</xs:element>
		<xs:element name="FieldOfView" type="tt:FieldOfView" minOccurs="0">
			<xs:annotation>
				<xs:documentation>
					States the current field of view of the video stream.
				</xs:documentation>
			</xs:annotation>
		</xs:element>
		<xs:any namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded"/>	<!-- first Vendor then ONVIF -->
	</xs:sequence>
	<xs:anyAttribute processContents="lax"/>
</xs:complexType>
 */
#[allow(unused)]
#[derive(Deserialize)]
pub struct PTZStatus {
    ///Specifies the absolute position of the PTZ unit together with the Space references. The default absolute spaces of the corresponding PTZ configuration MUST be referenced within the Position element.
    #[serde(rename = "Position")]
    pub position:PTZVector,
    ///Indicates if the Pan/Tilt/Zoom device unit is currently moving, idle or in an unknown state.
    #[serde(rename = "MoveStatus")]
    pub move_status:PTZMoveStatus,
    ///States a current PTZ error
    #[serde(rename = "Error")]
    pub error:String,
    ///Specifies the UTC time when this status was generated
    #[serde(rename = "UtcTime")]
    pub utc_time:String,//could deserialize to Chrono but i dont wanna add more deps. maybe feature
    ///States the current field of view of the video stream
    #[serde(rename = "FieldOfView")]
    pub field_of_view:FieldOfView,
    #[serde(rename = "#text")]
    content:String,
}
