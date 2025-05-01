use galileo::layer::raster_tile_layer::RasterTileLayerBuilder;
use galileo::{Map, MapBuilder};
use galileo::layer::FeatureLayer;
use galileo::symbol::CirclePointSymbol;
use galileo::galileo_types::latlon;
use galileo_types::geo::Crs;
use galileo::Color;

mod app;
use app::MapApp;

fn main() {
    run()
}

// Creates the window and starts the egui event loop.
pub(crate) fn run() {
    galileo_egui::init_with_app(Box::new(|cc| Ok(Box::new(MapApp::new(create_map(),cc,[])))))
        .expect("Couldn't create window");
}

// build a map showing the Open Street Map raster data for Seoul, South Korea
fn create_map()->Map {
    MapBuilder::default()
        // Set the position of the center of the map view
        .with_latlon(37.566, 126.9784)
        // Set the size of the smallest visible feature (zoom level) for the view to start with.
        .with_z_level(8)
        // Add the Open Street Maps raster tile server as a layer
        .with_layer(RasterTileLayerBuilder::new_osm().build().unwrap())
        // Add a blue dot at the coordinates specified
        .with_layer(FeatureLayer::new(
            // the position of the marker we're going to add, in the WGS84 CRS
            vec![latlon!(37.566, 126.9784)], 
            // a blue circle with fixed size of 5.0 pixels
            CirclePointSymbol::new(Color::BLUE, 5.0), 
            // WGS84 is the coordinate reference system that's based on latitude/longitude pairs.
            Crs::WGS84, 
        ))
        .build()
}

