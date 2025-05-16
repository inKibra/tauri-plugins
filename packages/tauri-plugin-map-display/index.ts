import { invoke } from '@tauri-apps/api/core';

export type MapRegion = {
  latitude: number;
  longitude: number;
  latitudeDelta: number;
  longitudeDelta: number;
};

export type ShowMapRequest = {
  region: MapRegion;
  style?: 'dark' | 'light' | 'default';
  poiFilter?: POICategory[];
  showBuildings?: boolean;
  showTraffic?: boolean;
  mapType?: 'standard' | 'satellite' | 'hybrid' | 'mutedStandard';
  cameraPitch?: number;
};

export type POICategory =
  // Arts and culture
  | 'museum'
  | 'musicVenue'
  | 'theater'
  // Education
  | 'library'
  | 'planetarium'
  | 'school'
  | 'university'
  // Entertainment
  | 'movieTheater'
  | 'nightlife'
  // Health and safety
  | 'fireStation'
  | 'hospital'
  | 'pharmacy'
  | 'police'
  // Historical and cultural landmarks
  | 'castle'
  | 'fortress'
  | 'landmark'
  | 'nationalMonument'
  // Food and drink
  | 'bakery'
  | 'brewery'
  | 'cafe'
  | 'distillery'
  | 'foodMarket'
  | 'restaurant'
  | 'winery'
  // Personal services
  | 'animalService'
  | 'atm'
  | 'automotiveRepair'
  | 'bank'
  | 'beauty'
  | 'evCharger'
  | 'fitnessCenter'
  | 'laundry'
  | 'mailbox'
  | 'postOffice'
  | 'restroom'
  | 'spa'
  | 'store'
  // Parks and recreation
  | 'amusementPark'
  | 'aquarium'
  | 'beach'
  | 'campground'
  | 'fairground'
  | 'marina'
  | 'nationalPark'
  | 'park'
  | 'rvPark'
  | 'zoo'
  // Sports
  | 'baseball'
  | 'basketball'
  | 'bowling'
  | 'goKart'
  | 'golf'
  | 'hiking'
  | 'miniGolf'
  | 'rockClimbing'
  | 'skatePark'
  | 'skating'
  | 'skiing'
  | 'soccer'
  | 'stadium'
  | 'tennis'
  | 'volleyball'
  // Travel
  | 'airport'
  | 'carRental'
  | 'conventionCenter'
  | 'gasStation'
  | 'hotel'
  | 'parking'
  | 'publicTransport'
  // Water sports
  | 'fishing'
  | 'kayaking'
  | 'surfing'
  | 'swimming';


export type ShowMapResponse = {
  success: boolean;
};

export type HideMapResponse = {
  success: boolean;
};

export type SetRegionResponse = {
  success: boolean;
};

export async function showMap(request: ShowMapRequest): Promise<ShowMapResponse> {
  return await invoke<ShowMapResponse>('plugin:map-display|show_map', {
    payload: request,
  });
}

export async function hideMap(): Promise<HideMapResponse> {
  return await invoke<HideMapResponse>('plugin:map-display|hide_map');
}

export async function setRegion(region: MapRegion): Promise<SetRegionResponse> {
  return await invoke<SetRegionResponse>('plugin:map-display|set_region', {
    payload: {
      region,
    },
  });
}