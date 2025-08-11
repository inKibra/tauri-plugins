use std::sync::{Mutex};
use std::path::{Path, PathBuf};
use std::fs;
use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime, AppHandle,
};

pub use models::*;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

/// Extensions to [`tauri::AppHandle`] to access the OTA APIs.
pub trait OTAExt<R: Runtime> {
  fn ota_manager(&self) -> &UpdateManager<R>;
}

impl<R: Runtime, T: Manager<R>> crate::OTAExt<R> for T {
  fn ota_manager(&self) -> &UpdateManager<R> {
    self.state::<UpdateManager<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("ota")
    .invoke_handler(tauri::generate_handler![
        commands::prepare
    ])
    .setup(|app, _api| {
      app.manage(UpdateManager::new(app.clone()));
      Ok(())
    })
    .build()
}

/// Update manager that handles the OTA update process
pub struct UpdateManager<R: Runtime> {
    app_handle: AppHandle<R>,
    update_state: Mutex<UpdateState>,
}

struct UpdateState {
    current_version: Option<String>,
    last_error: Option<String>,
    update_in_progress: bool,
    latest_manifest: Option<UpdateManifest>,
}

impl<R: Runtime> UpdateManager<R> {
    pub fn new(app_handle: AppHandle<R>) -> Self {
        // Initialize update state
        let update_state = Mutex::new(UpdateState {
            current_version: None,
            last_error: None,
            update_in_progress: false,
            latest_manifest: None,
        });

        // Create the update manager
        let manager = Self {
            app_handle,
            update_state,
        };

        // Try to load the current version
        if let Ok(version) = manager.load_current_version() {
            let mut state = manager.update_state.lock().unwrap();
            state.current_version = Some(version.clone());
            
            // Try to load the manifest
            let _ = manager.load_manifest(&version);
        }

        manager
    }

    // Get the cache directory where updates are stored
    fn get_storage_dir(&self) -> PathBuf {
        self.app_handle.path().app_cache_dir().unwrap()
    }

    // Get the path to store version info
    fn get_version_file_path(&self) -> PathBuf {
        self.get_storage_dir().join("current-version.txt")
    }

    // Get the path for storing manifest
    fn get_manifest_path(&self, version: &str) -> PathBuf {
        self.get_storage_dir().join(format!("manifest-{}.json", version))
    }

    // Get the path for storing the update content
    fn get_update_content_path(&self, version: &str) -> PathBuf {
        self.get_storage_dir().join(format!("update-content-{}.js", version))
    }

    // Load the current version from storage
    fn load_current_version(&self) -> Result<String> {
        let version_path = self.get_version_file_path();
        if !version_path.exists() {
            return Err(Error::NoUpdateAvailable);
        }
        
        let version = fs::read_to_string(version_path)?;
        Ok(version.trim().to_string())
    }

    // Load the manifest for a specific version
    fn load_manifest(&self, version: &str) -> Result<UpdateManifest> {
        let manifest_path = self.get_manifest_path(version);
        if !manifest_path.exists() {
            return Err(Error::MissingConfig("Manifest not found".into()));
        }
        
        let manifest_json = fs::read_to_string(manifest_path)?;
        let manifest: UpdateManifest = serde_json::from_str(&manifest_json)?;
        Ok(manifest)
    }

    // Load the update content for a specific version
    fn load_update_content(&self, version: &str) -> Result<String> {
        let content_path = self.get_update_content_path(version);
        if !content_path.exists() {
            return Err(Error::MissingConfig("Update content not found".into()));
        }
        
        let content = fs::read_to_string(content_path)?;
        Ok(content)
    }

    // Save current version to storage
    fn save_current_version(&self, version: &str) -> Result<()> {
        let version_path = self.get_version_file_path();
        
        // Ensure directory exists
        if let Some(parent) = version_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(version_path, version)?;
        Ok(())
    }

    // Save manifest for a specific version
    fn save_manifest(&self, version: &str, manifest: &UpdateManifest) -> Result<()> {
        let manifest_path = self.get_manifest_path(version);
        
        // Ensure directory exists
        if let Some(parent) = manifest_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let manifest_json = serde_json::to_string_pretty(manifest)?;
        fs::write(manifest_path, manifest_json)?;
        Ok(())
    }

    // Save update content for a specific version
    fn save_update_content(&self, version: &str, content: &str) -> Result<()> {
        let content_path = self.get_update_content_path(version);
        
        // Ensure directory exists
        if let Some(parent) = content_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(content_path, content)?;
        Ok(())
    }

    // Prepare - check for updates, download and return content if available
    pub async fn prepare(&self, manifest_url: String) -> Result<UpdateInfo> {
        // Check if update already in progress
        {
            let in_progress = {
                let state = self.update_state.lock().unwrap();
                state.update_in_progress
            };
            
            if in_progress {
                return Err(Error::UpdateInProgress);
            }
        }
        
        // Set update in progress
        {
            let mut state = self.update_state.lock().unwrap();
            state.update_in_progress = true;
        }
        
        // Fetch manifest
        let manifest_result = self.fetch_manifest(&manifest_url).await;
        let manifest = match manifest_result {
            Ok(m) => m,
            Err(e) => {
                // Update state with error
                {
                    let mut state = self.update_state.lock().unwrap();
                    state.last_error = Some(e.to_string());
                    state.update_in_progress = false;
                }
                
                // Try to load existing manifest and content if available
                let current_version = self.load_current_version().ok();
                if let Some(version) = current_version {
                    let manifest_result = self.load_manifest(&version);
                    let content_result = self.load_update_content(&version);
                    
                    if let (Ok(manifest), Ok(content)) = (manifest_result, content_result) {
                        return Ok(UpdateInfo {
                            update: Some(content),
                            manifest: Some(manifest),
                            error: None,
                        });
                    }
                }
                
                return Ok(UpdateInfo {
                    update: None,
                    manifest: None,
                    error: Some(e.to_string()),
                });
            }
        };
        
        // Compare versions
        let current_version = self.load_current_version().ok();
        let needs_update = match &current_version {
            Some(current) => manifest.version != *current,
            None => true,
        };
        
        // If no update needed
        if !needs_update {
            // Update state
            {
                let mut state = self.update_state.lock().unwrap();
                state.update_in_progress = false;
                state.current_version = current_version.clone();
                state.latest_manifest = Some(manifest.clone());
            }
            
            // Try to load the existing update content
            if let Some(version) = &current_version {
                if let Ok(content) = self.load_update_content(version) {
                    return Ok(UpdateInfo {
                        update: Some(content),
                        manifest: Some(manifest),
                        error: None,
                    });
                }
            }
            
            // If we couldn't load the content, download it again
            let content_result = self.download_update_content(&manifest).await;
            match content_result {
                Ok(content) => {
                    // Save the content
                    if let Err(e) = self.save_update_content(&manifest.version, &content) {
                        println!("Failed to save update content: {}", e);
                    }
                    
                    return Ok(UpdateInfo {
                        update: Some(content),
                        manifest: Some(manifest),
                        error: None,
                    });
                },
                Err(e) => {
                    return Ok(UpdateInfo {
                        update: None,
                        manifest: Some(manifest),
                        error: Some(format!("Failed to download content: {}", e)),
                    });
                }
            }
        }
        
        // Store manifest for later use
        {
            let mut state = self.update_state.lock().unwrap();
            state.latest_manifest = Some(manifest.clone());
        }
        
        // Download update content
        let content_result = self.download_update_content(&manifest).await;
        match content_result {
            Ok(content) => {
                // Update was successful
                let _ = self.save_current_version(&manifest.version);
                let _ = self.save_manifest(&manifest.version, &manifest);
                let _ = self.save_update_content(&manifest.version, &content);
                
                // Update state
                {
                    let mut state = self.update_state.lock().unwrap();
                    state.update_in_progress = false;
                    state.current_version = Some(manifest.version.clone());
                }
                
                Ok(UpdateInfo {
                    update: Some(content),
                    manifest: Some(manifest),
                    error: None,
                })
            },
            Err(e) => {
                // Download failed
                {
                    let mut state = self.update_state.lock().unwrap();
                    state.last_error = Some(e.to_string());
                    state.update_in_progress = false;
                }
                
                Ok(UpdateInfo {
                    update: None,
                    manifest: None,
                    error: Some(e.to_string()),
                })
            }
        }
    }

    // Fetch manifest from server
    async fn fetch_manifest(&self, url: &str) -> Result<UpdateManifest> {
        // Create HTTP client
        let client = reqwest::Client::new();
        
        // Fetch manifest
        let response = client.get(url).send().await?;
        let manifest: UpdateManifest = response.json().await?;
        
        Ok(manifest)
    }

    // Download update content and return it
    async fn download_update_content(&self, manifest: &UpdateManifest) -> Result<String> {
        // Create HTTP client
        let client = reqwest::Client::new();
        
        // Download file directly
        let response = client.get(&manifest.url).send().await?;
        let content = response.text().await?;
        
        // Verify hash
        self.verify_content_hash(&content, &manifest.hash)?;
        
        Ok(content)
    }

    // Verify content hash
    fn verify_content_hash(&self, content: &str, expected_hash: &str) -> Result<()> {
        use sha2::{Sha256, Digest};
        
        // Calculate hash
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let hash = hex::encode(hasher.finalize());
        
        // Compare hashes
        if hash != expected_hash {
            return Err(Error::HashVerificationFailed);
        }
        
        Ok(())
    }
}
