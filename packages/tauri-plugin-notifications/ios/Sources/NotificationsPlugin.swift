import SwiftRs
import Tauri
import UIKit
import UserNotifications
import WebKit

// MARK: - Supporting Structures

struct NotificationPermissionStatus: Encodable {
    let status: String
}

struct NotificationRegistrationStatus: Encodable {
    let isRegistered: Bool
    let token: String?
}

struct NotificationRegistrationResult: Encodable {
    let success: Bool
    let token: String?
    let error: String?
}

enum NotificationEventType: String, Encodable {
    case backgroundTap = "BACKGROUND_TAP"
    case foregroundTap = "FOREGROUND_TAP"
    case foregroundDelivery = "FOREGROUND_DELIVERY"
    case backgroundDelivery = "BACKGROUND_DELIVERY"
}

struct NotificationEvent: Encodable {
    let type: NotificationEventType
    let payload: [AnyHashable: Any]
    
    private enum CodingKeys: String, CodingKey {
        case type, payload
    }
    
    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(type, forKey: .type)
        
        // Convert payload to [String: String]
        let stringPayload = payload.reduce(into: [String: String]()) { result, pair in
            if let key = pair.key as? String {
                result[key] = String(describing: pair.value)
            }
        }
        try container.encode(stringPayload, forKey: .payload)
    }
}

struct WatchNotificationsArgs: Decodable {
    let channel: Channel
}

struct WatchNotificationResult: Encodable {
    let success: Bool
}

class NotificationsPlugin: Plugin, UNUserNotificationCenterDelegate {
    private var registrationInvoke: Invoke?
    private var notificationChannels: [Channel] = []
    private var originalDelegate: UIApplicationDelegate?
    
    override init() {
        super.init()
        UNUserNotificationCenter.current().delegate = self
    }
    
    @objc override public func load(webview: WKWebView) {
        super.load(webview: webview)
        Logger.debug("NotificationsPlugin: Loading plugin")
        
        // Register as the application delegate
        if let app = UIApplication.value(forKey: "sharedApplication") as? UIApplication {
            Logger.debug("NotificationsPlugin: Got shared application, setting delegate")
            NotificationCenter.default.addObserver(
                self,
                selector: #selector(handleBackgroundNotification(_:)),
                name: NSNotification.Name("UIApplicationDidReceiveRemoteNotification"),
                object: nil
            )

            self.originalDelegate = app.delegate
            app.delegate = self
        } else {
            Logger.error("NotificationsPlugin: Failed to get shared application")
        }
    }

    @objc private func handleBackgroundNotification(_ notification: Notification) {
        guard let userInfo = notification.userInfo as? [AnyHashable: Any] else { return }
        
        let stringKeyedUserInfo = userInfo.reduce(into: [String: Any]()) { result, pair in
            if let key = pair.key as? String {
                result[key] = pair.value
            }
        }
        
        emitNotificationEvent(NotificationEvent(
            type: .backgroundDelivery,
            payload: stringKeyedUserInfo
        ))
    }
    
    @objc private func handleRemoteNotificationRegistration(_ notification: Notification) {
        Logger.info("NotificationsPlugin: Received registration notification")
        if let deviceToken = notification.object as? Data {
            self.application(UIApplication.shared, didRegisterForRemoteNotificationsWithDeviceToken: deviceToken)
        }
    }
    
    @objc private func handleRemoteNotificationRegistrationError(_ notification: Notification) {
        Logger.info("NotificationsPlugin: Received registration error notification")
        if let error = notification.object as? Error {
            self.application(UIApplication.shared, didFailToRegisterForRemoteNotificationsWithError: error)
        }
    }
    
    @objc override public func checkPermissions(_ invoke: Invoke) {
        UNUserNotificationCenter.current().getNotificationSettings { settings in
            DispatchQueue.main.async {
                let status: String
                switch settings.authorizationStatus {
                case .notDetermined:
                    status = "prompt"
                case .denied:
                    status = "denied"
                case .authorized, .provisional, .ephemeral:
                    status = "granted"
                @unknown default:
                    status = "prompt"
                }
                invoke.resolve(NotificationPermissionStatus(status: status))
            }
        }
    }
    
    @objc override public func requestPermissions(_ invoke: Invoke) {
        UNUserNotificationCenter.current().requestAuthorization(options: [.alert, .sound, .badge]) { granted, error in
            DispatchQueue.main.async {
                let status = granted ? "granted" : "denied"
                invoke.resolve(NotificationPermissionStatus(status: status))
            }
        }
    }
    
    @objc public func checkRegistrationStatus(_ invoke: Invoke) {
        DispatchQueue.main.async {
            let isRegistered = UIApplication.shared.isRegisteredForRemoteNotifications
            let currentToken = UserDefaults.standard.string(forKey: "deviceToken")
            invoke.resolve(NotificationRegistrationStatus(
                isRegistered: isRegistered,
                token: currentToken
            ))
        }
    }
    
    @objc public func registerForRemoteNotifications(_ invoke: Invoke) {
        self.registrationInvoke = invoke
        
        Logger.info("Attempting to register for remote notifications")
        
        // First check if we already have permission
        let center = UNUserNotificationCenter.current()
        center.getNotificationSettings { settings in
            Logger.debug("Current notification settings: \(settings.authorizationStatus.rawValue)")
            
            DispatchQueue.main.async {
                switch settings.authorizationStatus {
                case .notDetermined:
                    // Need to request permission first
                    self.requestNotificationPermissionAndRegister(invoke)
                case .denied:
                    Logger.info("Notifications are denied")
                    let result = NotificationRegistrationResult(
                        success: false,
                        token: nil,
                        error: "Push notifications are not authorized. Please enable them in Settings."
                    )
                    invoke.resolve(result)
                case .authorized, .provisional, .ephemeral:
                    // Already have permission, proceed with registration
                    self.proceedWithRegistration(invoke)
                @unknown default:
                    Logger.error("Unknown authorization status")
                    let result = NotificationRegistrationResult(
                        success: false,
                        token: nil,
                        error: "Unknown authorization status"
                    )
                    invoke.resolve(result)
                }
            }
        }
    }
    
    private func requestNotificationPermissionAndRegister(_ invoke: Invoke) {
        Logger.info("Requesting notification permission")
        let center = UNUserNotificationCenter.current()
        center.requestAuthorization(options: [.alert, .sound, .badge]) { granted, error in
            if let error = error {
                Logger.error("Error requesting authorization: \(error.localizedDescription)")
                DispatchQueue.main.async {
                    let result = NotificationRegistrationResult(
                        success: false,
                        token: nil,
                        error: "Failed to request notification permission: \(error.localizedDescription)"
                    )
                    invoke.resolve(result)
                }
                return
            }
            
            DispatchQueue.main.async {
                if granted {
                    Logger.info("Permission granted, proceeding with registration")
                    self.proceedWithRegistration(invoke)
                } else {
                    Logger.info("Permission denied by user")
                    let result = NotificationRegistrationResult(
                        success: false,
                        token: nil,
                        error: "User denied notification permission"
                    )
                    invoke.resolve(result)
                }
            }
        }
    }
    
    private func proceedWithRegistration(_ invoke: Invoke) {
        // Ensure we're on the main thread
        if !Thread.isMainThread {
            DispatchQueue.main.async {
                self.proceedWithRegistration(invoke)
            }
            return
        }

        Logger.info("Proceeding with remote notification registration")
        Logger.debug("NotificationsPlugin: Application delegate is: \(String(describing: UIApplication.shared.delegate))")
        
        // Store the invoke for use in delegate callbacks
        self.registrationInvoke = invoke
        
        // Check if already registered with a token
        if let existingToken = UserDefaults.standard.string(forKey: "deviceToken") {
            Logger.info("Found existing device token: \(existingToken)")
            let result = NotificationRegistrationResult(
                success: true,
                token: existingToken,
                error: nil
            )
            invoke.resolve(result)
            return
        }
        
        // Register for remote notifications
        Logger.info("Calling registerForRemoteNotifications()")
        UIApplication.shared.registerForRemoteNotifications()
        
        // Set a timeout
        DispatchQueue.main.asyncAfter(deadline: .now() + 10.0) { [weak self] in
            guard let self = self, self.registrationInvoke != nil else { return }
            Logger.error("Registration timed out")
            Logger.debug("NotificationsPlugin: Application delegate at timeout is: \(String(describing: UIApplication.shared.delegate))")
            
            let result = NotificationRegistrationResult(
                success: false,
                token: nil,
                error: "Registration timed out. Please check your internet connection and provisioning profile."
            )
            self.registrationInvoke?.resolve(result)
            self.registrationInvoke = nil
        }
    }
    
    // Called when a notification arrives while app is in foreground
    public func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        willPresent notification: UNNotification,
        withCompletionHandler completionHandler: @escaping (UNNotificationPresentationOptions) -> Void
    ) {
        Logger.debug("NotificationsPlugin: Received notification while app in foreground")
        
        let userInfo = notification.request.content.userInfo as? [String: Any] ?? [:]
        emitNotificationEvent(NotificationEvent(
            type: .foregroundDelivery,
            payload: userInfo
        ))
        
        if #available(iOS 14.0, *) {
            completionHandler([.banner, .sound, .badge, .list])
        } else {
            completionHandler([.alert, .sound, .badge])
        }
    }
    
    // Called when user taps on a notification
    public func userNotificationCenter(
        _ center: UNUserNotificationCenter,
        didReceive response: UNNotificationResponse,
        withCompletionHandler completionHandler: @escaping () -> Void
    ) {
        Logger.debug("NotificationsPlugin: User responded to notification")
        
        let userInfo = response.notification.request.content.userInfo as? [String: Any] ?? [:]
        let isBackground = UIApplication.shared.applicationState != .active
        
        emitNotificationEvent(NotificationEvent(
            type: isBackground ? .backgroundTap : .foregroundTap,
            payload: userInfo
        ))
        
        completionHandler()
    }
    
    // Add method to handle background notifications
    public func application(
        _ application: UIApplication,
        didReceiveRemoteNotification userInfo: [AnyHashable: Any],
        fetchCompletionHandler completionHandler: @escaping (UIBackgroundFetchResult) -> Void
    ) {
        if application.applicationState != .active {
            let stringKeyedUserInfo = userInfo.reduce(into: [String: Any]()) { result, pair in
                if let key = pair.key as? String {
                    result[key] = pair.value
                }
            }
            emitNotificationEvent(NotificationEvent(
                type: .backgroundDelivery,
                payload: stringKeyedUserInfo
            ))
        }
        completionHandler(.newData)
    }
    
    // Add method to watch for notifications
    @objc public func watchNotifications(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(WatchNotificationsArgs.self)
        notificationChannels.append(args.channel)
        invoke.resolve(WatchNotificationResult(success: true))
    }
    
    // Helper method to emit events
    private func emitNotificationEvent(_ event: NotificationEvent) {
        notificationChannels.forEach { channel in
            try? channel.send(event)
        }
    }
}

// MARK: - UIApplicationDelegate
extension NotificationsPlugin: UIApplicationDelegate {
    public func application(_ application: UIApplication, didRegisterForRemoteNotificationsWithDeviceToken deviceToken: Data) {
        Logger.info("NotificationsPlugin: SUCCESS - Received device token")
        let tokenParts = deviceToken.map { data in String(format: "%02.2hhx", data) }
        let token = tokenParts.joined()
        
        Logger.info("NotificationsPlugin: Device token: \(token)")
        
        // Store token for later retrieval
        UserDefaults.standard.set(token, forKey: "deviceToken")
        
        let result = NotificationRegistrationResult(
            success: true,
            token: token,
            error: nil
        )
        
        DispatchQueue.main.async {
            self.registrationInvoke?.resolve(result)
            self.registrationInvoke = nil
        }
    }
    
    public func application(_ application: UIApplication, didFailToRegisterForRemoteNotificationsWithError error: Error) {
        Logger.error("NotificationsPlugin: FAILURE - Registration failed with error: \(error.localizedDescription)")
        Logger.error("NotificationsPlugin: Error domain: \(error._domain)")
        Logger.error("NotificationsPlugin: Error code: \(error._code)")
        
        let result = NotificationRegistrationResult(
            success: false,
            token: nil,
            error: error.localizedDescription
        )
        
        DispatchQueue.main.async {
            self.registrationInvoke?.resolve(result)
            self.registrationInvoke = nil
        }
    }

    /*
    Proxy all application delegate methods to the original delegate:
      sel!(application:didFinishLaunchingWithOptions:),
      sel!(application:openURL:options:),
      sel!(application:continue:restorationHandler:),
      sel!(applicationDidBecomeActive:),
      sel!(applicationWillResignActive:),
      sel!(applicationWillEnterForeground:),
      sel!(applicationDidEnterBackground:),
      sel!(applicationWillTerminate:),
    */

    public func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        self.originalDelegate?.application?(application, didFinishLaunchingWithOptions: launchOptions) ?? false
    }

    public func application(_ application: UIApplication, open url: URL, options: [UIApplication.OpenURLOptionsKey: Any] = [:]) -> Bool {
        self.originalDelegate?.application?(application, open: url, options: options) ?? false
    }

    public func application(_ application: UIApplication, continue continueUserActivity: NSUserActivity, restorationHandler: @escaping ([UIUserActivityRestoring]?) -> Void) -> Bool {
        self.originalDelegate?.application?(application, continue: continueUserActivity, restorationHandler: restorationHandler) ?? false
    }

    public func applicationDidBecomeActive(_ application: UIApplication) {
        self.originalDelegate?.applicationDidBecomeActive?(application)
    }

    public func applicationWillResignActive(_ application: UIApplication) {
        self.originalDelegate?.applicationWillResignActive?(application)
    }

    public func applicationWillEnterForeground(_ application: UIApplication) {
        self.originalDelegate?.applicationWillEnterForeground?(application)
    }

    public func applicationDidEnterBackground(_ application: UIApplication) {
        self.originalDelegate?.applicationDidEnterBackground?(application)
    }

    public func applicationWillTerminate(_ application: UIApplication) {
        self.originalDelegate?.applicationWillTerminate?(application)
    }
}

@_cdecl("init_plugin_notifications")
func initPlugin() -> Plugin {
    return NotificationsPlugin()
}