import SwiftRs
import Tauri
import UIKit
import MapKit
import WebKit

class MapDisplayPlugin: Plugin {
    private var mapView: MKMapView?
    
    private func log(_ message: Any) {
        if let error = message as? Error {
            NSLog("MapDisplayPlugin Error: \(error.localizedDescription)")
        } else {
            NSLog("MapDisplayPlugin: \(message)")
        }
    }
    
    @objc public func showMap(_ invoke: Invoke) throws {
        log("showMap called")
        let args = try invoke.parseArgs(ShowMapRequest.self)
        
        DispatchQueue.main.async {
            self.log("Setting up map view")
            do {
                try self.setupMapView(with: args)
            } catch {
                self.log(error)
                invoke.reject(error.localizedDescription)
                return
            }
        }
        
        invoke.resolve(ShowMapResponse(success: true))
    }
    
    private func setupMapView(with args: ShowMapRequest) throws {
        guard let rootViewController = UIApplication.shared.windows.first?.rootViewController else {
            log("Failed to get root view controller")
            throw NSError(domain: "MapDisplayPlugin", code: 1, userInfo: [NSLocalizedDescriptionKey: "No root view controller found"])
        }
        
        if self.mapView == nil {
            self.mapView = MKMapView(frame: rootViewController.view.bounds)
            rootViewController.view.insertSubview(self.mapView!, at: 0)
            self.mapView?.autoresizingMask = [.flexibleWidth, .flexibleHeight]
            
            // Set WKWebView to be transparent
            if let webView = rootViewController.view.subviews.first(where: { $0 is WKWebView }) as? WKWebView {
                webView.isOpaque = false
                webView.backgroundColor = .clear
                webView.scrollView.backgroundColor = .clear
            }
        }
        
        guard let mapView = self.mapView else {
            log("MapView is nil after setup")
            throw NSError(domain: "MapDisplayPlugin", code: 2, userInfo: [NSLocalizedDescriptionKey: "Failed to create MapView"])
        }
        
        // Apply map style
        applyMapStyle(to: mapView, style: args.style)
        
        // Apply POI filter
        applyPOIFilter(to: mapView, filter: args.poiFilter)
        
        // Set building display
        mapView.showsBuildings = args.showBuildings ?? true
        
        // Set traffic display
        mapView.showsTraffic = args.showTraffic ?? false
        
        // Set map type
        applyMapType(to: mapView, type: args.mapType)
        
        // Set camera pitch
        if #available(iOS 16.0, *) {
            if let pitch = args.cameraPitch {
                let camera = mapView.camera
                camera.pitch = min(max(pitch, 0), 90)  // Clamp pitch between 0 and 90 degrees
                mapView.setCamera(camera, animated: true)
            }
        }

        
        // Set region
        let region = MKCoordinateRegion(
            center: CLLocationCoordinate2D(latitude: args.region.latitude, longitude: args.region.longitude),
            span: MKCoordinateSpan(latitudeDelta: args.region.latitudeDelta, longitudeDelta: args.region.longitudeDelta)
        )
        mapView.setRegion(region, animated: false)
        
        mapView.isHidden = false
        log("Map view setup complete with style: \(args.style?.rawValue ?? "unspecified")")
    }
    
    private func applyMapStyle(to mapView: MKMapView, style: MapStyle?) {
        if #available(iOS 13.0, *) {
            switch style {
            case .dark:
                mapView.overrideUserInterfaceStyle = .dark
                log("Applied dark style to map")
            case .light:
                mapView.overrideUserInterfaceStyle = .light
                log("Applied light style to map")
            case .default, .none:
                mapView.overrideUserInterfaceStyle = .unspecified
                log("Applied default (unspecified) style to map")
            }
        } else {
            log("Map style changes are not supported on this iOS version. Using default style.")
        }
        
        // Keep the map type standard for all styles
        mapView.mapType = .standard
    }
    
    private func applyPOIFilter(to mapView: MKMapView, filter: [POICategory]?) {
        guard #available(iOS 13.0, *) else {
            log("POI filtering not available on this iOS version")
            return
        }

        guard let filter = filter, !filter.isEmpty else {
            mapView.pointOfInterestFilter = .includingAll
            return
        }

        let categories = filter.compactMap { category -> MKPointOfInterestCategory? in
            let categoryString = category.rawValue
            let poiCategory = MKPointOfInterestCategory(rawValue: categoryString)
            
            // Check if the category is a valid MKPointOfInterestCategory
            if poiCategory != MKPointOfInterestCategory(rawValue: "invalid_category_name") {
                return poiCategory
            } else {
                log("POI category '\(categoryString)' is not available on this iOS version")
                return nil
            }
        }

        mapView.pointOfInterestFilter = MKPointOfInterestFilter(including: categories)
    }

    private func applyMapType(to mapView: MKMapView, type: MapType?) {
        switch type {
        case .some(.standard):
            mapView.mapType = .standard
        case .some(.satellite):
            mapView.mapType = .satellite
        case .some(.hybrid):
            mapView.mapType = .hybrid
        case .some(.mutedStandard):
            if #available(iOS 11.0, *) {
                mapView.mapType = .mutedStandard
            } else {
                mapView.mapType = .standard
            }
        case .none:
            mapView.mapType = .standard
        }
    }
    
    @objc public func hideMap(_ invoke: Invoke) throws {
        log("hideMap called")
        DispatchQueue.main.async {
            self.mapView?.isHidden = true
            self.log("Map hidden")
        }
        invoke.resolve(HideMapResponse(success: true))
    }
    
    @objc public func setRegion(_ invoke: Invoke) throws {
        log("setRegion called")
        let args = try invoke.parseArgs(SetRegionRequest.self)
        
        DispatchQueue.main.async {
            let region = MKCoordinateRegion(
                center: CLLocationCoordinate2D(latitude: args.region.latitude, longitude: args.region.longitude),
                span: MKCoordinateSpan(latitudeDelta: args.region.latitudeDelta, longitudeDelta: args.region.longitudeDelta)
            )
            self.mapView?.setRegion(region, animated: true)
            self.log("New region set")
        }
        
        invoke.resolve(SetRegionResponse(success: true))
    }
}

struct ShowMapRequest: Decodable {
    let region: MapRegion
    let style: MapStyle?
    let poiFilter: [POICategory]?
    let showBuildings: Bool?
    let showTraffic: Bool?
    let mapType: MapType?
    let cameraPitch: Double?
}

struct MapRegion: Decodable {
    let latitude: Double
    let longitude: Double
    let latitudeDelta: Double
    let longitudeDelta: Double
}

enum MapStyle: String, Decodable {
    case dark
    case light
    case `default`
}

enum POICategory: String, Decodable {
    // Arts and culture
    case museum
    case musicVenue
    case theater
    // Education
    case library
    case planetarium
    case school
    case university
    // Entertainment
    case movieTheater
    case nightlife
    // Health and safety
    case fireStation
    case hospital
    case pharmacy
    case police
    // Historical and cultural landmarks
    case castle
    case fortress
    case landmark
    case nationalMonument
    // Food and drink
    case bakery
    case brewery
    case cafe
    case distillery
    case foodMarket
    case restaurant
    case winery
    // Personal services
    case animalService
    case atm
    case automotiveRepair
    case bank
    case beauty
    case evCharger
    case fitnessCenter
    case laundry
    case mailbox
    case postOffice
    case restroom
    case spa
    case store
    // Parks and recreation
    case amusementPark
    case aquarium
    case beach
    case campground
    case fairground
    case marina
    case nationalPark
    case park
    case rvPark
    case zoo
    // Sports
    case baseball
    case basketball
    case bowling
    case goKart
    case golf
    case hiking
    case miniGolf
    case rockClimbing
    case skatePark
    case skating
    case skiing
    case soccer
    case stadium
    case tennis
    case volleyball
    // Travel
    case airport
    case carRental
    case conventionCenter
    case gasStation
    case hotel
    case parking
    case publicTransport
    // Water sports
    case fishing
    case kayaking
    case surfing
    case swimming
}

enum MapType: String, Decodable {
    case standard
    case satellite
    case hybrid
    case mutedStandard
}

struct ShowMapResponse: Encodable {
    let success: Bool
}

struct HideMapResponse: Encodable {
    let success: Bool
}

struct SetRegionRequest: Decodable {
    let region: MapRegion
}

struct SetRegionResponse: Encodable {
    let success: Bool
}

@_cdecl("init_plugin_map_display")
func initPlugin() -> Plugin {
    return MapDisplayPlugin()
}