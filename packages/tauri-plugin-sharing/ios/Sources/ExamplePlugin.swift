import SwiftRs
import Tauri
import UIKit
import WebKit

class PingArgs: Decodable {
  let value: String?
}

class ShareArgs: Decodable {
    let text: String?
    let url: String?
}

class ExamplePlugin: Plugin {
  @objc public func ping(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(PingArgs.self)
    invoke.resolve(["value": args.value ?? ""])
  }
  
  @objc public func share(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(ShareArgs.self)
    
    var itemsToShare: [Any] = []
    
    if let text = args.text {
        itemsToShare.append(text)
    }
    
    if let urlString = args.url, let url = URL(string: urlString) {
        itemsToShare.append(url)
    }
    
    guard !itemsToShare.isEmpty else {
        invoke.reject("No items to share")
        return
    }
    
    DispatchQueue.main.async {
        let activityViewController = UIActivityViewController(activityItems: itemsToShare, applicationActivities: nil)
        
        if let window = UIApplication.shared.windows.first,
           let viewController = window.rootViewController {
            activityViewController.popoverPresentationController?.sourceView = viewController.view
            viewController.present(activityViewController, animated: true) {
                invoke.resolve(["success": true])
            }
        } else {
            invoke.reject("Unable to present share sheet")
        }
    }
  }
}

@_cdecl("init_plugin_sharing")
func initPlugin() -> Plugin {
  return ExamplePlugin()
}
