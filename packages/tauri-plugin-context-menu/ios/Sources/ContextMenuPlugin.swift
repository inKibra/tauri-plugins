import SwiftRs
import Tauri
import UIKit
import WebKit

class ContextMenuArgs: Decodable {
  let items: [MenuItem]
  let x: CGFloat
  let y: CGFloat
}

struct MenuItem: Decodable {
  let title: String
  let id: String
}

class ContextMenuPlugin: Plugin {
  @objc public func showContextMenu(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(ContextMenuArgs.self)
    
    DispatchQueue.main.async {
      guard let window = UIApplication.shared.windows.first,
            let viewController = window.rootViewController else {
        invoke.reject("Unable to present context menu")
        return
      }
      
      let alertController = UIAlertController(title: nil, message: nil, preferredStyle: .actionSheet)
      
      for item in args.items {
        alertController.addAction(UIAlertAction(title: item.title, style: .default) { _ in
          invoke.resolve(["selectedId": item.id])
        })
      }
      
      alertController.addAction(UIAlertAction(title: "Cancel", style: .cancel) { _ in
        invoke.resolve(["selectedId": NSNull()])
      })
      
      if let popoverController = alertController.popoverPresentationController {
        popoverController.sourceView = viewController.view
        popoverController.sourceRect = CGRect(x: args.x, y: args.y, width: 0, height: 0)
        popoverController.permittedArrowDirections = []
      }
      
      viewController.present(alertController, animated: true, completion: nil)
    }
  }
}

@_cdecl("init_plugin_context_menu")
func initPlugin() -> Plugin {
  return ContextMenuPlugin()
}
