import SwiftRs
import Tauri
import UIKit

struct CustomPattern: Decodable {
    let durations: [Double]
    let intensities: [Double]
}

struct VibratePayload: Decodable {
    let pattern: String
    let customPattern: CustomPattern?
}

struct ImpactFeedbackPayload: Decodable {
    let style: String
}

struct HapticResponse: Encodable {
    let success: Bool
}

class HapticFeedbackPlugin: Plugin {
    private func log(_ message: Any) {
        if let error = message as? Error {
            NSLog("HapticFeedbackPlugin Error: \(error.localizedDescription)")
        } else {
            NSLog("HapticFeedbackPlugin: \(message)")
        }
    }
    
    @objc public func vibrate(_ invoke: Invoke) throws {
        log("vibrate called")
        let payload = try invoke.parseArgs(VibratePayload.self)
        
        DispatchQueue.main.async {
            do {
                try self.performVibration(with: payload)
                invoke.resolve(HapticResponse(success: true))
            } catch {
                self.log(error)
                invoke.reject(error.localizedDescription)
            }
        }
    }
    
    private func performVibration(with payload: VibratePayload) throws {
        switch payload.pattern {
        case "short":
            UIImpactFeedbackGenerator(style: .light).impactOccurred()
        case "medium":
            UIImpactFeedbackGenerator(style: .medium).impactOccurred()
        case "long":
            UIImpactFeedbackGenerator(style: .heavy).impactOccurred()
        case "custom":
            if let customPattern = payload.customPattern {
                let generator = UISelectionFeedbackGenerator()
                generator.prepare()
                for (duration, intensity) in zip(customPattern.durations, customPattern.intensities) {
                    generator.selectionChanged()
                    Thread.sleep(forTimeInterval: duration / 1000.0)
                }
            }
        default:
            throw NSError(domain: "HapticFeedbackPlugin", code: 1, userInfo: [NSLocalizedDescriptionKey: "Invalid pattern"])
        }
    }
    
    @objc public func impactFeedback(_ invoke: Invoke) throws {
        log("impactFeedback called")
        let payload = try invoke.parseArgs(ImpactFeedbackPayload.self)
        
        DispatchQueue.main.async {
            do {
                try self.performImpactFeedback(with: payload.style)
                invoke.resolve(HapticResponse(success: true))
            } catch {
                self.log(error)
                invoke.reject(error.localizedDescription)
            }
        }
    }
    
    private func performImpactFeedback(with style: String) throws {
        let feedbackStyle: UIImpactFeedbackGenerator.FeedbackStyle
        switch style {
        case "light":
            feedbackStyle = .light
        case "medium":
            feedbackStyle = .medium
        case "heavy":
            feedbackStyle = .heavy
        default:
            throw NSError(domain: "HapticFeedbackPlugin", code: 2, userInfo: [NSLocalizedDescriptionKey: "Invalid impact style"])
        }
        
        UIImpactFeedbackGenerator(style: feedbackStyle).impactOccurred()
    }
    
    @objc public func selectionFeedback(_ invoke: Invoke) throws {
        log("selectionFeedback called")
        DispatchQueue.main.async {
            UISelectionFeedbackGenerator().selectionChanged()
            invoke.resolve(HapticResponse(success: true))
        }
    }
}

@_cdecl("init_plugin_haptic_feedback")
func initPlugin() -> Plugin {
    return HapticFeedbackPlugin()
}