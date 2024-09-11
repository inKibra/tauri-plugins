use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowMapRequest {
    pub region: MapRegion,
    pub style: Option<MapStyle>,
    pub poi_filter: Option<Vec<POICategory>>,
    pub show_buildings: Option<bool>,
    pub show_traffic: Option<bool>,
    pub map_type: Option<MapType>,
    pub camera_pitch: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapRegion {
    pub latitude: f64,
    pub longitude: f64,
    pub latitude_delta: f64,
    pub longitude_delta: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MapStyle {
    Dark,
    Light,
    Default,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum POICategory {
    // Arts and culture
    Museum,
    MusicVenue,
    Theater,
    // Education
    Library,
    Planetarium,
    School,
    University,
    // Entertainment
    MovieTheater,
    Nightlife,
    // Health and safety
    FireStation,
    Hospital,
    Pharmacy,
    Police,
    // Historical and cultural landmarks
    Castle,
    Fortress,
    Landmark,
    NationalMonument,
    // Food and drink
    Bakery,
    Brewery,
    Cafe,
    Distillery,
    FoodMarket,
    Restaurant,
    Winery,
    // Personal services
    AnimalService,
    Atm,
    AutomotiveRepair,
    Bank,
    Beauty,
    EvCharger,
    FitnessCenter,
    Laundry,
    Mailbox,
    PostOffice,
    Restroom,
    Spa,
    Store,
    // Parks and recreation
    AmusementPark,
    Aquarium,
    Beach,
    Campground,
    Fairground,
    Marina,
    NationalPark,
    Park,
    RvPark,
    Zoo,
    // Sports
    Baseball,
    Basketball,
    Bowling,
    GoKart,
    Golf,
    Hiking,
    MiniGolf,
    RockClimbing,
    SkatePark,
    Skating,
    Skiing,
    Soccer,
    Stadium,
    Tennis,
    Volleyball,
    // Travel
    Airport,
    CarRental,
    ConventionCenter,
    GasStation,
    Hotel,
    Parking,
    PublicTransport,
    // Water sports
    Fishing,
    Kayaking,
    Surfing,
    Swimming,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MapType {
    Standard,
    Satellite,
    Hybrid,
    MutedStandard,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowMapResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HideMapRequest {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HideMapResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRegionRequest {
    pub region: MapRegion,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRegionResponse {
    pub success: bool,
}