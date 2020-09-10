#[derive(Deserialize, Serialize, Debug, Default)]
/// Attributes of a light
pub struct Light {
	pub uniqueid: String,
	#[serde(rename = "type")]
	pub light_type: String,
	pub name: String,
	pub modelid: String,
	pub manufacturername: String,
	pub productid: String,
	pub state: LightState,
	pub swversion: String,
	pub swconfigid: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
/// Current state of a light
pub struct LightState {
	pub on: bool,
	/// Brightness
	pub bri: u8,
	/// Color tone
	pub ct: u16,
	/// Alert mode
	pub alert: String,
	pub colormode: String,
	pub mode: String,
	pub reachable: bool,
}