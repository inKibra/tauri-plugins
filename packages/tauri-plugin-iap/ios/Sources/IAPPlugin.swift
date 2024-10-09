import SwiftRs
import Tauri
import StoreKit

// MARK: - Supporting Structures

// Structure for decoding fetch products arguments
struct FetchProductsArgs: Decodable {
    let productIds: [String]
}

// Structure for decoding purchase product arguments
struct PurchaseProductArgs: Decodable {
    let productId: String
}

// Structure for product information
struct ProductInfo: Encodable {
    let id: String
    let title: String
    let description: String
    let price: String
    let priceLocale: String
}

// Structure for purchased/restored product information
struct PurchasedProduct: Encodable {
    let productId: String
    let transactionId: String?
    let originalTransactionId: String?
}

// Structure for purchase result
struct PurchaseResult: Encodable {
    let success: Bool
    let product: PurchasedProduct?
    let error: String?
}

// Structure for restore purchases result
struct RestoreResult: Encodable {
    let success: Bool
    let restoredProducts: [PurchasedProduct]
    let error: String?
}

// MARK: - IAPPlugin

class IAPPlugin: Plugin {
    // Store available products
    private var products: [SKProduct] = []
    private var productRequest: SKProductsRequest?
    
    // Store invokes for asynchronous resolution
    private var fetchProductsInvoke: Invoke?
    private var purchaseInvokes: [String: Invoke] = [:]
    private var restorePurchasesInvoke: Invoke?
    
    // Initialize the plugin and set up as payment queue observer
    override init() {
        super.init()
        SKPaymentQueue.default().add(self)
    }
    
    // Clean up when the plugin is deinitialized
    deinit {
        SKPaymentQueue.default().remove(self)
    }
    
    // Fetch available products from the App Store
    @objc public func fetchProducts(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(FetchProductsArgs.self)
        
        let productIdentifiers = Set(args.productIds)
        productRequest = SKProductsRequest(productIdentifiers: productIdentifiers)
        productRequest?.delegate = self
        productRequest?.start()
        
        // Store the invoke to resolve later when products are fetched
        self.fetchProductsInvoke = invoke
    }
    
    // Initiate a purchase for a specific product
    @objc public func purchaseProduct(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(PurchaseProductArgs.self)
        
        // Find the product in our available products
        guard let product = products.first(where: { $0.productIdentifier == args.productId }) else {
            invoke.reject("Product not found")
            return
        }
        
        // Create and add the payment to the queue
        let payment = SKPayment(product: product)
        SKPaymentQueue.default().add(payment)
        
        // Store the invoke to resolve later when the purchase completes
        purchaseInvokes[args.productId] = invoke
    }
    
    // Restore previously purchased products
    @objc public func restorePurchases(_ invoke: Invoke) {
        SKPaymentQueue.default().restoreCompletedTransactions()
        // Store the invoke to resolve later when the restore completes
        self.restorePurchasesInvoke = invoke
    }
}

// MARK: - SKProductsRequestDelegate

extension IAPPlugin: SKProductsRequestDelegate {
    // Called when the product request completes successfully
    func productsRequest(_ request: SKProductsRequest, didReceive response: SKProductsResponse) {
        self.products = response.products
        // Convert products to our custom ProductInfo struct
        let productList = response.products.map { product in
            ProductInfo(
                id: product.productIdentifier,
                title: product.localizedTitle,
                description: product.localizedDescription,
                price: product.price.stringValue,
                priceLocale: product.priceLocale.identifier
            )
        }
        
        // Resolve the fetchProducts invoke with the product list
        self.fetchProductsInvoke?.resolve(productList)
        self.fetchProductsInvoke = nil
    }
    
    // Called if the product request fails
    func request(_ request: SKRequest, didFailWithError error: Error) {
        // Reject the fetchProducts invoke with the error message
        self.fetchProductsInvoke?.reject(error.localizedDescription)
        self.fetchProductsInvoke = nil
    }
}

// MARK: - SKPaymentTransactionObserver

extension IAPPlugin: SKPaymentTransactionObserver {
    // Called when there are updates to payment transactions
    func paymentQueue(_ queue: SKPaymentQueue, updatedTransactions transactions: [SKPaymentTransaction]) {
        for transaction in transactions {
            switch transaction.transactionState {
            case .purchased, .restored:
                // Handle successful purchase or restore
                let productId = transaction.payment.productIdentifier
                let purchasedProduct = PurchasedProduct(
                    productId: productId,
                    transactionId: transaction.transactionIdentifier,
                    originalTransactionId: transaction.original?.transactionIdentifier ?? transaction.transactionIdentifier
                )
                let result = PurchaseResult(
                    success: true,
                    product: purchasedProduct,
                    error: nil
                )
                // Resolve the purchase invoke for this product
                self.purchaseInvokes[productId]?.resolve(result)
                self.purchaseInvokes.removeValue(forKey: productId)
                queue.finishTransaction(transaction)
            case .failed:
                // Handle failed purchase
                let productId = transaction.payment.productIdentifier
                let result = PurchaseResult(
                    success: false,
                    product: nil,
                    error: transaction.error?.localizedDescription ?? "Unknown error"
                )
                // Resolve the purchase invoke for this product with the error
                self.purchaseInvokes[productId]?.resolve(result)
                self.purchaseInvokes.removeValue(forKey: productId)
                queue.finishTransaction(transaction)
            case .deferred, .purchasing:
                // These states don't require any action
                break
            @unknown default:
                break
            }
        }
    }

    // Called when the restore purchases process completes successfully
    func paymentQueueRestoreCompletedTransactionsFinished(_ queue: SKPaymentQueue) {
        // Collect the details of all restored products
        let restoredProducts = queue.transactions
            .filter { $0.transactionState == .restored }
            .map { transaction in
                PurchasedProduct(
                    productId: transaction.payment.productIdentifier,
                    transactionId: transaction.transactionIdentifier,
                    originalTransactionId: transaction.original?.transactionIdentifier
                )
            }
        
        let result = RestoreResult(
            success: true,
            restoredProducts: restoredProducts,
            error: nil
        )
        
        // Resolve the restore purchases invoke with the result
        self.restorePurchasesInvoke?.resolve(result)
        self.restorePurchasesInvoke = nil
    }
    
    // Called if the restore purchases process fails
    func paymentQueue(_ queue: SKPaymentQueue, restoreCompletedTransactionsFailedWithError error: Error) {
        let result = RestoreResult(
            success: false,
            restoredProducts: [],
            error: error.localizedDescription
        )
        
        // Resolve the restore purchases invoke with the error
        self.restorePurchasesInvoke?.resolve(result)
        self.restorePurchasesInvoke = nil
    }
}

// Entry point for the plugin
@_cdecl("init_plugin_iap")
func initPlugin() -> Plugin {
    return IAPPlugin()
}