use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
    Client, Error, Method, Response,
};

pub struct SellAppClient {
    api_key: String,
    http_client: Client,
}

impl SellAppClient {
    fn generate_headers(&self, headers_vec: Vec<[&'static str; 2]>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("Bearer {}", self.api_key).as_str()).unwrap(),
        );
        for header in headers_vec {
            let header_key = header[0];
            let header_value = HeaderValue::from_str(header[1]).unwrap();
            headers.insert(header_key, header_value);
        }
        return headers;
    }

    async fn send_request(&self, url: String, method: Method) -> Result<Response, Error> {
        let req_headers = self.generate_headers(vec![["Accept", "application/json"]]);
        let req_url = format!("https://sell.app/api/{}", url);

        let req = self
            .http_client
            .request(method, req_url)
            .headers(req_headers)
            .send()
            .await;

        return req;
    }

    async fn send_request_data(
        &self,
        url: String,
        method: Method,
        body: String,
    ) -> Result<Response, Error> {
        let req_headers = self.generate_headers(vec![
            ["Accept", "application/json"],
            ["Content-Type", "application/json"],
        ]);
        let req_url = format!("https://sell.app/api/{}", url);

        let req = self
            .http_client
            .request(method, req_url)
            .headers(req_headers)
            .body(body)
            .send()
            .await;

        return req;
    }

    /// Fetch all of your blacklist rules.
    ///
    /// ``url_params``: Optional attributes to append to the request URL, e.g. **"?limit=50&page=1"**
    ///
    /// https://developer.sell.app/blacklists#list-all-blacklist-rules
    pub async fn blacklist_list_all_rules(&self, url_params: &str) -> Result<Response, Error> {
        return self
            .send_request(format!("v1/blacklists{}", url_params), Method::GET)
            .await;
    }

    /// Create a new blacklist rule.
    ///
    /// ``data``: JSON with the attributes of the new rule, required.
    ///
    /// https://developer.sell.app/blacklists#create-a-blacklist-rule
    pub async fn blacklist_create_rule(&self, data: String) -> Result<Response, Error> {
        return self
            .send_request_data("v1/blacklists".to_string(), Method::POST, data)
            .await;
    }

    /// Get a blacklist rule by ID.
    ///
    /// ``rule_id``: The ID of the rule you want to fetch, required.
    ///
    /// https://developer.sell.app/blacklists#retrieve-a-blacklist-rule
    pub async fn blacklist_get_rule(&self, rule_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v1/blacklists/{}", rule_id), Method::GET)
            .await;
    }

    /// Update a blacklist rule by ID.
    ///
    /// ``rule_id``: The ID of the rule you want to update, required.
    ///
    /// ``data``: JSON with the updated attributes of the rule, required.
    ///
    /// https://developer.sell.app/blacklists#update-a-blacklist-rule
    pub async fn blacklist_update_rule(
        &self,
        rule_id: String,
        data: String,
    ) -> Result<Response, Error> {
        return self
            .send_request_data(format!("v1/blacklists/{}", rule_id), Method::PATCH, data)
            .await;
    }

    /// Delete a blacklist rule by ID.
    ///
    /// ``rule_id``: The ID of the rule you want to delete, required.
    ///
    /// https://developer.sell.app/blacklists#delete-a-blacklist-rule
    pub async fn blacklist_delete_rule(&self, rule_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v1/blacklists/{}", rule_id), Method::DELETE)
            .await;
    }

    /// Get all existing coupons.
    ///
    /// ``url_params``: Optional attributes to append to the request URL, e.g. **"?limit=50&with_trashed=false"**
    ///
    /// https://developer.sell.app/coupons#list-all-coupons
    pub async fn coupons_list_all(&self, url_params: &str) -> Result<Response, Error> {
        return self
            .send_request(format!("v1/coupons{}", url_params), Method::GET)
            .await;
    }

    /// Create a new coupon.
    ///
    /// ``data``: JSON with the attributes of the new coupon, required.
    ///
    /// https://developer.sell.app/coupons#create-a-coupon
    pub async fn coupons_create(&self, data: String) -> Result<Response, Error> {
        return self
            .send_request_data("v1/coupons".to_owned(), Method::POST, data)
            .await;
    }

    /// Get a coupon by ID.
    ///
    /// ``coupon_id``: The ID of the coupon you want to fetch, required.
    ///
    /// https://developer.sell.app/coupons#retrieve-a-coupon
    pub async fn coupons_get(&self, coupon_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v1/coupons/{}", coupon_id), Method::GET)
            .await;
    }

    /// Update a coupon by ID.
    ///
    /// ``coupon_id``: The ID of the coupon you want to update, required.
    ///
    /// ``data``: JSON with the updated attributes of the coupon, required.
    ///
    /// https://developer.sell.app/coupons#update-a-coupon
    pub async fn coupons_update(&self, coupon_id: String, data: String) -> Result<Response, Error> {
        return self
            .send_request_data(format!("v1/coupons/{}", coupon_id), Method::PATCH, data)
            .await;
    }

    /// Delete a coupon by ID.
    ///
    /// ``coupon_id``: The ID of the coupon you want to delete, required.
    ///
    /// https://developer.sell.app/coupons#delete-a-coupon
    pub async fn coupons_delete(&self, coupon_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v1/coupons/{}", coupon_id), Method::DELETE)
            .await;
    }

    /// Get all feedback (reviews).
    ///
    /// ``url_params``: Optional attributes to append to the request URL, e.g. **"?limit=50&page=1"**
    ///
    /// https://developer.sell.app/feedback#list-all-feedback
    pub async fn feedback_list_all(&self, url_params: &str) -> Result<Response, Error> {
        return self
            .send_request(format!("v1/feedback{}", url_params), Method::GET)
            .await;
    }

    /// Get specific feedback by ID.
    ///
    /// ``feedback_id``: The ID of the feedback you want to fetch, required.
    ///
    /// https://developer.sell.app/feedback#retrieve-specific-feedback
    pub async fn feedback_get(&self, feedback_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v1/feedback/{}", feedback_id), Method::GET)
            .await;
    }

    /// Reply to specifc feedback by ID.
    ///
    /// ``feedback_id``: The ID of the feedback you want to reply to, required.
    ///
    /// ``data``: JSON with the reply, required.
    ///
    /// https://developer.sell.app/feedback#reply-to-feedback
    pub async fn feedback_reply(
        &self,
        feedback_id: String,
        data: String,
    ) -> Result<Response, Error> {
        return self
            .send_request_data(format!("v1/coupons/{}", feedback_id), Method::PATCH, data)
            .await;
    }

    /// Get all existing product groups.
    ///
    /// ``url_params``: Optional attributes to append to the request URL, e.g. **"?limit=50&page=1"**
    ///
    /// https://developer.sell.app/groups#list-all-groups
    pub async fn groups_list_all(&self, url_params: &str) -> Result<Response, Error> {
        return self
            .send_request(format!("v2/groups{}", url_params), Method::GET)
            .await;
    }

    /// Create a product group.
    ///
    /// ``data``: JSON with the attributes of the group, required.
    ///
    /// https://developer.sell.app/groups#create-a-group
    pub async fn groups_create(&self, data: String) -> Result<Response, Error> {
        return self
            .send_request_data("v2/groups".to_string(), Method::POST, data)
            .await;
    }

    /// Get specific product group by ID.
    ///
    /// ``group_id``: The ID of the group you want to fetch, required.
    ///
    /// https://developer.sell.app/groups#retrieve-a-group
    pub async fn groups_get(&self, group_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v2/groups/{}", group_id), Method::GET)
            .await;
    }

    /// Update a specific product group by ID.
    ///
    /// ``group_id``: The ID of the group you want to update, required.
    ///
    /// ``data``: JSON with the updated attributes, required.
    ///
    /// https://developer.sell.app/groups#update-a-group
    pub async fn groups_update(&self, group_id: String, data: String) -> Result<Response, Error> {
        return self
            .send_request_data(format!("v2/groups/{}", group_id), Method::PATCH, data)
            .await;
    }

    /// Delete a product group by ID.
    ///
    /// ``group_id``: The ID of the group you want to delete, required.
    ///
    /// https://developer.sell.app/groups#delete-a-group
    pub async fn groups_delete(&self, group_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v2/groups/{}", group_id), Method::DELETE)
            .await;
    }

    /// Add products to an existing product group.
    ///
    /// ``group_id``: The ID of the group you want to add products to, required.
    ///
    /// ``data``: JSON with the product ID's you want to add, required.
    ///
    /// https://developer.sell.app/groups#add-products-to-group
    pub async fn groups_add_products(
        &self,
        group_id: String,
        data: String,
    ) -> Result<Response, Error> {
        return self
            .send_request_data(
                format!("v2/groups/{}/products/attach", group_id),
                Method::POST,
                data,
            )
            .await;
    }

    /// Remove products from an existing product group.
    ///
    /// ``group_id``: The ID of the group you want to remove products from, required.
    ///
    /// ``data``: JSON with the product ID's you want to remove, required.
    ///
    /// https://developer.sell.app/groups#remove-products-from-group
    pub async fn groups_remove_products(
        &self,
        group_id: String,
        data: String,
    ) -> Result<Response, Error> {
        return self
            .send_request_data(
                format!("v2/groups/{}/products/detach", group_id),
                Method::DELETE,
                data,
            )
            .await;
    }

    /// Get all products within a product group.
    ///
    /// ``group_id``: The ID of the group you want to list the products from, required.
    ///
    /// ``url_params``: Optional attributes to append to the request URL, e.g. **"?limit=50&page=1"**
    ///
    /// https://developer.sell.app/groups#list-all-products-within-group
    pub async fn groups_list_products(
        &self,
        group_id: String,
        url_params: &str,
    ) -> Result<Response, Error> {
        return self
            .send_request(
                format!("v2/groups/{}/products{}", group_id, url_params),
                Method::GET,
            )
            .await;
    }

    /// Get a specific product within a product group.
    ///
    /// ``group_id``: The ID of the group you want to get the product from, required.
    ///
    /// ``product_id``: The ID of the product you want to fetch, required.
    ///
    /// https://developer.sell.app/groups#list-specific-product-within-group
    pub async fn groups_get_product(
        &self,
        group_id: String,
        product_id: String,
    ) -> Result<Response, Error> {
        return self
            .send_request(
                format!("v2/groups/{}/products/{}", group_id, product_id),
                Method::GET,
            )
            .await;
    }

    /// Get all existing invoices (orders).
    ///
    /// ``url_params``: Optional attributes to append to the request URL, e.g. **"?limit=50&page=1"**
    ///
    /// https://developer.sell.app/invoices-v2#list-all-invoices
    pub async fn invoices_list_all(&self, url_params: &str) -> Result<Response, Error> {
        return self
            .send_request(format!("v2/invoices{}", url_params), Method::GET)
            .await;
    }

    /// Create a new invoice for a customer.
    ///
    /// ``data``: JSON with the invoice data, required.
    ///
    /// https://developer.sell.app/invoices-v2#create-an-invoice
    pub async fn invoices_create(&self, data: String) -> Result<Response, Error> {
        return self
            .send_request_data("v2/invoices".to_string(), Method::POST, data)
            .await;
    }

    /// Get specific invoice by ID.
    ///
    /// ``invoice_id``: The ID of the invoice you want to fetch, required.
    ///
    /// https://developer.sell.app/invoices-v2#retrieve-an-invoice
    pub async fn invoices_get(&self, invoice_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v2/invoices/{}", invoice_id), Method::GET)
            .await;
    }

    /// Start a checkout session for a speficic invoice.
    ///
    /// ``invoice_id``: The ID of the invoice you want to start checkout for, required.
    ///
    /// https://developer.sell.app/invoices-v2#create-a-checkout-session
    pub async fn invoices_checkout(&self, invoice_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v2/invoices/{}/checkout", invoice_id), Method::POST)
            .await;
    }

    /// Get the deliverables included in a specific invoice.
    ///
    /// ``invoice_id``: The ID of the invoice you want to get the items for, required.
    ///
    /// https://developer.sell.app/invoices-v2#view-invoice-deliverables
    pub async fn invoices_get_items(&self, invoice_id: String) -> Result<Response, Error> {
        return self
            .send_request(
                format!("v2/invoices/{}/deliverables", invoice_id),
                Method::GET,
            )
            .await;
    }

    /// **Note: SellApp automatically marks payments as completed, you shouldn't need to call this.**
    ///
    /// Mark a specific (pending) invoice as completed.
    ///
    /// ``invoice_id``: The ID of the invoice you want to set as completed, required.
    ///
    /// https://developer.sell.app/invoices-v2#mark-pending-invoice-completed
    pub async fn invoices_mark_completed(&self, invoice_id: String) -> Result<Response, Error> {
        return self
            .send_request(
                format!("v2/invoices/{}/mark-completed", invoice_id),
                Method::PATCH,
            )
            .await;
    }

    /// Mark a specific (pending) invoice as voided.
    ///
    /// ``invoice_id``: The ID of the invoice you want to set as voided, required.
    ///
    /// https://developer.sell.app/invoices-v2#mark-pending-invoice-voided
    pub async fn invoices_mark_voided(&self, invoice_id: String) -> Result<Response, Error> {
        return self
            .send_request(
                format!("v2/invoices/{}/mark-voided", invoice_id),
                Method::PATCH,
            )
            .await;
    }

    /// Issue a replacement for a specific invoice.
    ///
    /// ``invoice_id``: The ID of the invoice you want to issue a replacement for, required.
    ///
    /// ``data``: JSON with the product variant ID's to get the replacements from.
    ///
    /// https://developer.sell.app/invoices-v2#issue-replacement-for-completed-invoice
    pub async fn invoices_issue_replacement(
        &self,
        invoice_id: String,
        data: String,
    ) -> Result<Response, Error> {
        return self
            .send_request_data(
                format!("v2/invoices/{}/issue-replacement", invoice_id),
                Method::PATCH,
                data,
            )
            .await;
    }

    /// Get all existing products.
    ///
    /// ``url_params``: Optional attributes to append to the request URL, e.g. **"?limit=50&page=1"**
    ///
    /// https://developer.sell.app/products-v2#list-all-products
    pub async fn products_list_all(&self, url_params: &str) -> Result<Response, Error> {
        return self
            .send_request(format!("v2/products{}", url_params), Method::GET)
            .await;
    }

    /// Create a new product. Support for uploading images is WIP.
    ///
    /// ``data``: JSON with the attributes of the product, required.
    ///
    /// https://developer.sell.app/products-v2#create-a-product
    pub async fn products_create(&self, data: String) -> Result<Response, Error> {
        return self
            .send_request_data("v2/products".to_string(), Method::POST, data)
            .await;
    }

    /// Get a specific product by ID.
    ///
    /// ``product_id``: The ID of the product you want to fetch, required.
    ///
    /// https://developer.sell.app/products-v2#retrieve-a-product
    pub async fn products_get(&self, product_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v2/products/{}", product_id), Method::GET)
            .await;
    }

    /// Update a specific product.
    ///
    /// ``product_id``: The ID of the product you want to update, required.
    ///
    /// ``data``: JSON with the new product attributes, required.
    ///
    /// https://developer.sell.app/products-v2#update-a-product
    pub async fn products_update(
        &self,
        product_id: String,
        data: String,
    ) -> Result<Response, Error> {
        return self
            .send_request_data(format!("v2/products/{}", product_id), Method::PATCH, data)
            .await;
    }

    /// Delete a specific product.
    ///
    /// ``product_id``: The ID of the product you want to delete, required.
    ///
    /// https://developer.sell.app/products-v2#delete-a-product
    pub async fn products_delete(&self, product_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v2/products/{}", product_id), Method::DELETE)
            .await;
    }

    /// Get all existing product variants.
    ///
    /// ``product_id``: The ID of the product you want to get the variants of, required.
    ///
    /// ``url_params``: Optional attributes to append to the request URL, e.g. **"?limit=50&page=1"**
    ///
    /// https://developer.sell.app/product-variants-v2#list-all-product-variants
    pub async fn variants_list_all(
        &self,
        product_id: String,
        url_params: &str,
    ) -> Result<Response, Error> {
        return self
            .send_request(
                format!("v2/products/{}/variants{}", product_id, url_params),
                Method::GET,
            )
            .await;
    }

    /// Create a new product variant.
    ///
    /// ``product_id``: The ID of the product you want to add a new variant to, required.
    ///
    /// ``data``: JSON with the attributes of the variant, required.
    ///
    /// https://developer.sell.app/product-variants-v2#create-a-product-variant
    pub async fn variants_create(
        &self,
        product_id: String,
        data: String,
    ) -> Result<Response, Error> {
        return self
            .send_request_data(
                format!("v2/products/{}/variants", product_id),
                Method::POST,
                data,
            )
            .await;
    }

    /// Get a specific variant of a specific product.
    ///
    /// ``product_id``: The ID of the product, required.
    ///
    /// ``variant_id``: The ID of the variant you want to fetch, required.
    ///
    /// https://developer.sell.app/product-variants-v2#retrieve-a-product-variant
    pub async fn variants_get(
        &self,
        product_id: String,
        variant_id: String,
    ) -> Result<Response, Error> {
        return self
            .send_request(
                format!("v2/products/{}/variants/{}", product_id, variant_id),
                Method::GET,
            )
            .await;
    }

    /// Update a specific variant of a specific product.
    ///
    /// ``product_id``: The ID of the product, required.
    ///
    /// ``variant_id``: The ID of the variant you want to update, required.
    ///
    /// ``data``: JSON with the new variant attributes, required.
    ///
    /// https://developer.sell.app/product-variants-v2#update-a-product-variant
    pub async fn variants_update(
        &self,
        product_id: String,
        variant_id: String,
        data: String,
    ) -> Result<Response, Error> {
        return self
            .send_request_data(
                format!("v2/products/{}/variants/{}", product_id, variant_id),
                Method::PATCH,
                data,
            )
            .await;
    }

    /// Delete a specific product variant.
    ///
    /// ``product_id``: The ID of the product, required.
    ///
    /// ``variant_id``: The ID of the variant you want to delete, required.
    ///
    /// https://developer.sell.app/products-v2#delete-a-product
    pub async fn variants_delete(
        &self,
        product_id: String,
        variant_id: String,
    ) -> Result<Response, Error> {
        return self
            .send_request(
                format!("v2/products/{}/variants/{}", product_id, variant_id),
                Method::DELETE,
            )
            .await;
    }

    /// Get all existing sections.
    ///
    /// ``url_params``: Optional attributes to append to the request URL, e.g. **"?limit=50&page=1"**
    ///
    /// https://developer.sell.app/sections#list-all-sections
    pub async fn sections_list_all(&self, url_params: &str) -> Result<Response, Error> {
        return self
            .send_request(format!("v1/sections{}", url_params), Method::GET)
            .await;
    }

    /// Create a new section.
    ///
    /// ``data``: JSON with the attributes of the secton, required.
    ///
    /// https://developer.sell.app/sections#create-a-section
    pub async fn sections_create(&self, data: String) -> Result<Response, Error> {
        return self
            .send_request_data("v1/sections".to_string(), Method::POST, data)
            .await;
    }

    /// Get a specific section by ID.
    ///
    /// ``section_id``: The ID of the section, required.
    ///
    /// https://developer.sell.app/sections#retrieve-a-section
    pub async fn sections_get(&self, section_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v1/sections/{}", section_id), Method::GET)
            .await;
    }

    /// Update a specific section.
    ///
    /// ``section_id``: The ID of the section you want to update, required.
    ///
    /// ``data``: JSON with the new section attributes, required.
    ///
    /// https://developer.sell.app/sections#update-a-section
    pub async fn sections_update(
        &self,
        section_id: String,
        data: String,
    ) -> Result<Response, Error> {
        return self
            .send_request_data(format!("v1/sections/{}", section_id), Method::PATCH, data)
            .await;
    }

    /// Delete a specific product section.
    ///
    /// ``section_id``: The ID of the section you want to delete, required.
    ///
    /// https://developer.sell.app/sections#delete-a-section
    pub async fn sections_delete(&self, section_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v1/sections/{}", section_id), Method::DELETE)
            .await;
    }

    /// Get all existing tickets.
    ///
    /// ``url_params``: Optional attributes to append to the request URL, e.g. **"?limit=50&page=1"**
    ///
    /// https://developer.sell.app/tickets#list-all-tickets
    pub async fn tickets_list_all(&self, url_params: &str) -> Result<Response, Error> {
        return self
            .send_request(format!("v1/tickets{}", url_params), Method::GET)
            .await;
    }

    /// Get a specific ticket.
    ///
    /// ``ticket_id``: The ID of the ticket you want to fetch, required.
    ///
    /// https://developer.sell.app/tickets#retrieve-specific-ticket
    pub async fn tickets_get(&self, ticket_id: String) -> Result<Response, Error> {
        return self
            .send_request(format!("v1/tickets/{}", ticket_id), Method::GET)
            .await;
    }

    /// Get the messages from a specific ticket.
    ///
    /// ``ticket_id``: The ID of the ticket you want to get the messages from, required.
    ///
    /// ``url_params``: Optional attributes to append to the request URL, e.g. **"?limit=50&page=1"**
    ///
    /// https://developer.sell.app/tickets#list-all-ticket-messages
    pub async fn tickets_list_messages(
        &self,
        ticket_id: String,
        url_params: &str,
    ) -> Result<Response, Error> {
        return self
            .send_request(
                format!("v1/tickets/{}/messages{}", ticket_id, url_params),
                Method::GET,
            )
            .await;
    }

    /// Send a message to a specific ticket.
    ///
    /// ``ticket_id``: The ID of the ticket you want to send a messages to, required.
    ///
    /// ``data``: JSON with the message, required.
    ///
    /// https://developer.sell.app/tickets#reply-to-ticket
    pub async fn tickets_reply(&self, ticket_id: String, data: String) -> Result<Response, Error> {
        return self
            .send_request_data(
                format!("v1/tickets/{}/messages", ticket_id),
                Method::POST,
                data,
            )
            .await;
    }

    /// Get the a specific message from a specific ticket.
    ///
    /// ``ticket_id``: The ID of the ticket you want to get the message from, required.
    ///
    /// ``msg_id``: The ID of the message you want to get, required.
    ///
    /// https://developer.sell.app/tickets#retrieve-specific-ticket-message
    pub async fn tickets_get_message(
        &self,
        ticket_id: String,
        msg_id: String,
    ) -> Result<Response, Error> {
        return self
            .send_request(
                format!("v1/tickets/{}/messages/{}", ticket_id, msg_id),
                Method::GET,
            )
            .await;
    }
}

/// Initialize the client to make calls with your SellApp API key.
///
/// ```
/// let sellapp_api = sellapp::init("your_api_key");
/// ```
pub fn init(api_key: &str) -> SellAppClient {
    let key = api_key.to_string();
    let http_client = reqwest::Client::new();
    return SellAppClient {
        api_key: key,
        http_client,
    };
}
