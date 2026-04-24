use futures::TryStreamExt;
use mongodb::bson::{self, Bson, Document, doc, oid::ObjectId};
use serde::{Deserialize, Deserializer, Serialize};
use utoipa::ToSchema;

use crate::error::AppError;

use super::MongoService;

// Accepts both numeric and string representations of a u32 (some tenants
// store svgWidth/svgHeight as strings instead of integers).
fn deser_optional_u32<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u32>, D::Error> {
    struct V;
    impl serde::de::Visitor<'_> for V {
        type Value = Option<u32>;
        fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "a u32 or a string containing a u32")
        }
        fn visit_i32<E: serde::de::Error>(self, v: i32) -> Result<Self::Value, E> {
            Ok(Some(v as u32))
        }
        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
            Ok(Some(v as u32))
        }
        fn visit_u32<E: serde::de::Error>(self, v: u32) -> Result<Self::Value, E> {
            Ok(Some(v))
        }
        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(Some(v as u32))
        }
        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            v.parse().map(Some).map_err(serde::de::Error::custom)
        }
        fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
        fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }
    d.deserialize_any(V)
}

// ---------------------------------------------------------------------------
// Shared sub-types (used in both the internal Tenant and public TenantResponse)
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum OnOff {
    #[serde(rename = "ON")]
    On,
    #[serde(rename = "OFF")]
    Off,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VisualizationItem {
    pub url: Option<String>,
    #[serde(rename = "svgWidth", default, deserialize_with = "deser_optional_u32")]
    pub svg_width: Option<u32>,
    #[serde(rename = "svgHeight", default, deserialize_with = "deser_optional_u32")]
    pub svg_height: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Visualization {
    #[serde(rename = "STADIUM")]
    pub stadium: Option<VisualizationItem>,
    #[serde(rename = "CARD")]
    pub card: Option<VisualizationItem>,
    #[serde(rename = "PLAYER")]
    pub player: Option<VisualizationItem>,
    #[serde(rename = "STADIUM_2")]
    pub stadium_2: Option<VisualizationItem>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Webhooks {
    #[serde(rename = "LOYALTY_POINTS_ADDED")]
    pub loyalty_points_added: Option<String>,
    #[serde(rename = "SEASON_ORDER_PAID")]
    pub season_order_paid: Option<String>,
    #[serde(rename = "EVENT_CREATE")]
    pub event_create: Option<String>,
    #[serde(rename = "GDPR")]
    pub gdpr: Option<String>,
    #[serde(rename = "MEMBERSHIP_ORDER_PAID")]
    pub membership_order_paid: Option<String>,
    #[serde(rename = "MEMBERSHIP_STATUS_CHANGE")]
    pub membership_status_change: Option<String>,
    #[serde(rename = "MEMBERSHIP_STORNO")]
    pub membership_storno: Option<String>,
    #[serde(rename = "MEMBERSHIP_SUBSCRIPTION_PAYMENT")]
    pub membership_subscription_payment: Option<String>,
    #[serde(rename = "MEMBERSHIP_SUBSCRIPTION_PAYMENT_CHANGE")]
    pub membership_subscription_payment_change: Option<String>,
    #[serde(rename = "SEASON_BARCODE_CHANGE")]
    pub season_barcode_change: Option<String>,
    #[serde(rename = "SEASON_STORNO")]
    pub season_storno: Option<String>,
    #[serde(rename = "SEASON_TICKET_GIFT")]
    pub season_ticket_gift: Option<String>,
    #[serde(rename = "TICKET_BARCODE_CHANGE")]
    pub ticket_barcode_change: Option<String>,
    #[serde(rename = "TICKET_ORDER_PAID")]
    pub ticket_order_paid: Option<String>,
    #[serde(rename = "TICKET_STORNO")]
    pub ticket_storno: Option<String>,
    #[serde(rename = "TURNSTILE_ENTRANCE")]
    pub turnstile_entrance: Option<String>,
    #[serde(rename = "MOBILE_APP_PAIR_SEASON_TICKET")]
    pub mobile_app_pair_season_ticket: Option<String>,
    #[serde(rename = "MOBILE_APP_UNPAIR_SEASON_TICKET")]
    pub mobile_app_unpair_season_ticket: Option<String>,
    #[serde(rename = "USER_ONEID_DELETE")]
    pub user_oneid_delete: Option<String>,
    #[serde(rename = "USER_ONEID_REGISTRATION")]
    pub user_oneid_registration: Option<String>,
    #[serde(rename = "ESHOP_ORDER_PAID")]
    pub eshop_order_paid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FrequencyCap {
    pub enabled: Option<bool>,
    #[serde(rename = "maxEmails")]
    pub max_emails: Option<i32>,
    #[serde(rename = "periodDays")]
    pub period_days: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Security {
    #[serde(rename = "firstLoginPasswordChange")]
    pub first_login_password_change: Option<bool>,
    #[serde(rename = "twoFARequired")]
    pub two_fa_required: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AutomationMoveBusinessCase {
    pub label: Option<String>,
    // Nullable BSON value — serialises as extended JSON, represented as any object in the schema
    #[schema(value_type = Object)]
    pub value: Option<Bson>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BiUrl {
    pub name: Option<String>,
    pub public: Option<bool>,
    pub users: Option<Vec<String>>,
    #[serde(rename = "default")]
    pub is_default: Option<bool>,
    pub favorite: Option<bool>,
    pub id: Option<String>,
    pub tenant: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PriceListImport {
    pub enabled: Option<bool>,
    #[serde(rename = "allowedUsers")]
    pub allowed_users: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InternalFeatures {
    #[serde(rename = "priceListImport")]
    pub price_list_import: Option<PriceListImport>,
}

// ---------------------------------------------------------------------------
// Internal deserialization type (never leaves this module)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct Tenant {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    #[serde(rename = "__v")]
    version: i32,
    hostname: String,
    #[serde(rename = "dbName")]
    db_name: String,
    active: Option<bool>,

    // Modules
    #[serde(rename = "BUSINESS_MODULE")]
    business_module: OnOff,
    #[serde(rename = "FANS_MODULE")]
    fans_module: OnOff,
    #[serde(rename = "MOBILE_MODULE")]
    mobile_module: Option<OnOff>,

    // Content & branding
    #[serde(rename = "CONTENT_TYPE")]
    content_type: Option<String>,
    plan: Option<String>,
    #[serde(rename = "CLUB_LOGO")]
    club_logo: Option<String>,
    #[serde(rename = "VISUALIZATION")]
    visualization: Option<Visualization>,

    // Endpoints
    #[serde(rename = "DOMAIN_NAME")]
    domain_name: Option<String>,
    #[serde(rename = "API_ENDPOINT")]
    api_endpoint: Option<String>,

    // Organization info
    address: Option<String>,
    company: Option<String>,
    dic: Option<String>,
    ico: Option<String>,
    #[serde(rename = "defaultLanguage")]
    default_language: Option<String>,
    #[serde(rename = "messageProvider")]
    message_provider: Option<String>,
    #[serde(rename = "allowDefaultGdpr")]
    allow_default_gdpr: Option<bool>,
    #[serde(rename = "isGinaEnabled")]
    is_gina_enabled: Option<bool>,
    #[serde(rename = "CONTRACT_RS_TEMPLATE")]
    contract_rs_template: Option<String>,
    #[serde(rename = "APPROVAL_EMAIL")]
    approval_email: Option<String>,
    #[serde(rename = "CONTRACT_EMAIL")]
    contract_email: Option<String>,

    // Email
    #[serde(rename = "EMAIL_HOST")]
    email_host: Option<String>,
    #[serde(rename = "EMAIL_PASSWORD")]
    email_password: Option<String>,
    #[serde(rename = "EMAIL_PORT")]
    email_port: Option<i32>,
    #[serde(rename = "EMAIL_USERNAME")]
    email_username: Option<String>,

    // AWS
    #[serde(rename = "AWS_ACCESS_KEY")]
    aws_access_key: Option<String>,
    #[serde(rename = "AWS_ADDRESS")]
    aws_address: Option<String>,
    #[serde(rename = "AWS_BUCKET_NAME")]
    aws_bucket_name: Option<String>,
    #[serde(rename = "AWS_BUCKET_REGION")]
    aws_bucket_region: Option<String>,
    #[serde(rename = "AWS_SECRET_KEY")]
    aws_secret_key: Option<String>,

    // Google OAuth
    #[serde(rename = "GOOGLE")]
    google: Option<bool>,
    #[serde(rename = "GOOGLE_CALLBACK")]
    google_callback: Option<String>,
    #[serde(rename = "GOOGLE_CALLBACK_FE_FAIL")]
    google_callback_fe_fail: Option<String>,
    #[serde(rename = "GOOGLE_CLIENT_ID")]
    google_client_id: Option<String>,
    #[serde(rename = "GOOGLE_CLIENT_SECRET")]
    google_client_secret: Option<String>,

    // Microsoft OAuth
    #[serde(rename = "MICROSOFT")]
    microsoft: Option<bool>,
    #[serde(rename = "MICROSOFT_CALLBACK")]
    microsoft_callback: Option<String>,
    #[serde(rename = "MICROSOFT_CALLBACK_FE_FAIL")]
    microsoft_callback_fe_fail: Option<String>,
    #[serde(rename = "MICROSOFT_CLIENT_ID")]
    microsoft_client_id: Option<String>,
    #[serde(rename = "MICROSOFT_CLIENT_SECRET")]
    microsoft_client_secret: Option<String>,
    #[serde(rename = "MICROSOFT_TENANT_ID")]
    microsoft_tenant_id: Option<String>,

    // API access tokens
    #[serde(rename = "ACCESS_API_ESHOP_TOKEN")]
    access_api_eshop_token: Option<String>,
    #[serde(rename = "ACCESS_API_MOBILE_TOKEN")]
    access_api_mobile_token: Option<String>,
    #[serde(rename = "ACCESS_API_TICKETING")]
    access_api_ticketing: Option<String>,
    #[serde(rename = "ACCESS_API_ONEID")]
    access_api_oneid: Option<String>,
    #[serde(rename = "ACCESS_API_INTERNAL")]
    access_api_internal: Option<String>,

    // Push notifications (OneSignal)
    #[serde(rename = "ONE_SIGNAL_API_KEY")]
    one_signal_api_key: Option<String>,
    #[serde(rename = "ONE_SIGNAL_APP_ID")]
    one_signal_app_id: Option<String>,

    // SMS
    #[serde(rename = "NH_SMS_GATEWAY_KEY")]
    nh_sms_gateway_key: Option<String>,

    // RabbitMQ
    #[serde(rename = "RABBITMQ_VHOST")]
    rabbitmq_vhost: Option<String>,
    #[serde(rename = "RABBITMQ_CONSUMER_COUNT")]
    rabbitmq_consumer_count: Option<i32>,
    #[serde(rename = "RABBITMQ_MULTIPLE_PREFETCH")]
    rabbitmq_multiple_prefetch: Option<i32>,
    #[serde(rename = "RABBITMQ_SINGLE_PREFETCH")]
    rabbitmq_single_prefetch: Option<i32>,
    #[serde(rename = "RABBITMQ_IP")]
    rabbitmq_ip: Option<String>,
    #[serde(rename = "RABBITMQ_PASS")]
    rabbitmq_pass: Option<String>,
    #[serde(rename = "RABBITMQ_USER")]
    rabbitmq_user: Option<String>,

    // OneID integration
    #[serde(rename = "ONEID_INTEGRATION")]
    oneid_integration: Option<bool>,
    #[serde(rename = "ONEID_API")]
    oneid_api: Option<String>,
    #[serde(rename = "ONEID_PASS")]
    oneid_pass: Option<String>,
    #[serde(rename = "ONEID_TOKEN")]
    oneid_token: Option<String>,
    #[serde(rename = "ONEID_USER")]
    oneid_user: Option<String>,

    // Enigoo integration
    #[serde(rename = "ENIGOO_INTEGRATION")]
    enigoo_integration: Option<bool>,
    #[serde(rename = "ENIGOO_API")]
    enigoo_api: Option<String>,
    #[serde(rename = "ENIGOO_TOKEN")]
    enigoo_token: Option<String>,
    #[serde(rename = "ENIGOO_TOKEN_API")]
    enigoo_token_api: Option<String>,
    #[serde(rename = "ENIGOO_CHANNEL")]
    enigoo_channel: Option<String>,

    // SparkPost
    #[serde(rename = "SPARK_POST_API_KEY")]
    spark_post_api_key: Option<String>,
    #[serde(rename = "SPARK_POST_SUB_ACCOUNT")]
    spark_post_sub_account: Option<String>,
    #[serde(rename = "SPARK_POST_WEBHOOK_PASS")]
    spark_post_webhook_pass: Option<String>,
    #[serde(rename = "SPARK_POST_WEBHOOK_USERNAME")]
    spark_post_webhook_username: Option<String>,

    // ClickHouse
    #[serde(rename = "CLICKHOUSE_PASSWORD")]
    clickhouse_password: Option<String>,
    #[serde(rename = "CLICKHOUSE_USERNAME")]
    clickhouse_username: Option<String>,

    // Esports
    #[serde(rename = "ESPORTS_GAMES_KEY")]
    esports_games_key: Option<String>,
    #[serde(rename = "ESPORTS_GAMES_API")]
    esports_games_api: Option<String>,

    // Other integrations / feature flags
    #[serde(rename = "NEON_INTEGRATION")]
    neon_integration: Option<bool>,
    #[serde(rename = "TICKET_PORTAL_INTEGRATION")]
    ticket_portal_integration: Option<bool>,
    #[serde(rename = "SPARTAID_INTEGRATION")]
    spartaid_integration: Option<bool>,
    #[serde(rename = "FUTURED_INTEGRATION")]
    futured_integration: Option<bool>,
    #[serde(rename = "BONUS_GUIDELINE")]
    bonus_guideline: Option<bool>,
    #[serde(rename = "aiSegments")]
    ai_segments: Option<bool>,
    #[serde(rename = "ASSOCIATION_CRM")]
    association_crm: Option<bool>,

    // Business rules
    #[serde(rename = "HIGHLIGHT_BEFORE_EXPIRATION")]
    highlight_before_expiration: Option<i32>,
    #[serde(rename = "MAX_SEGMENT_RECALCULATE")]
    max_segment_recalculate: Option<i32>,
    #[serde(rename = "AUTOMATION_BUSINESS_CASE_STATE")]
    automation_business_case_state: Option<bool>,
    #[serde(rename = "AUTOMATION_MOVE_BUSINESS_CASE")]
    automation_move_business_case: Option<AutomationMoveBusinessCase>,
    #[serde(rename = "ONLY_OWNER_CAN_EDIT_BUSINESS_CASE")]
    only_owner_can_edit_business_case: Option<bool>,
    #[serde(rename = "SUBJECT_PREFIXES")]
    subject_prefixes: Option<Vec<Bson>>,
    #[serde(rename = "CREATE_FAN_FROM_TICKET_ATTRIBUTE")]
    create_fan_from_ticket_attribute: Option<bool>,
    #[serde(rename = "FAN_IMPORT_MATCHING_STRATEGY")]
    fan_import_matching_strategy: Option<String>,

    // Invoice notifications
    #[serde(rename = "INVOICE_NOTIFICATION")]
    invoice_notification: Option<bool>,
    #[serde(rename = "INVOICE_NOTIFICATION_DAYS_BEFORE")]
    invoice_notification_days_before: Option<i32>,
    #[serde(rename = "INVOICE_NOTIFICATION_EMAIL")]
    invoice_notification_email: Option<String>,

    // Frequency caps
    #[serde(rename = "emailFrequencyCap")]
    email_frequency_cap: Option<FrequencyCap>,
    #[serde(rename = "messageFrequencyCap")]
    message_frequency_cap: Option<FrequencyCap>,
    #[serde(rename = "pushNotificationFrequencyCap")]
    push_notification_frequency_cap: Option<FrequencyCap>,

    // Nested documents
    security: Option<Security>,
    webhooks: Option<Webhooks>,
    #[serde(rename = "BI_URL")]
    bi_url: Option<Vec<BiUrl>>,
    #[serde(rename = "internalFeatures")]
    internal_features: Option<InternalFeatures>,
    // gdprTranslation has a deeply nested, variable structure — kept as raw Document
    #[serde(rename = "gdprTranslation")]
    gdpr_translation: Option<Document>,
}

// ---------------------------------------------------------------------------
// Public response types
// ---------------------------------------------------------------------------

/// Public representation of a tenant returned by the API.
#[derive(Debug, Serialize, ToSchema)]
pub struct TenantResponse {
    /// MongoDB ObjectId serialised as a 24-character hex string.
    pub id: String,
    pub name: String,
    pub version: i32,
    pub hostname: String,
    pub db_name: String,
    pub active: Option<bool>,

    // Modules
    pub business_module: OnOff,
    pub fans_module: OnOff,
    pub mobile_module: Option<OnOff>,

    // Content & branding
    pub content_type: Option<String>,
    pub plan: Option<String>,
    pub club_logo: Option<String>,
    pub visualization: Option<Visualization>,

    // Endpoints
    pub domain_name: Option<String>,
    pub api_endpoint: Option<String>,

    // Organization info
    pub address: Option<String>,
    pub company: Option<String>,
    pub dic: Option<String>,
    pub ico: Option<String>,
    pub default_language: Option<String>,
    pub message_provider: Option<String>,
    pub allow_default_gdpr: Option<bool>,
    pub is_gina_enabled: Option<bool>,
    pub contract_rs_template: Option<String>,
    pub approval_email: Option<String>,
    pub contract_email: Option<String>,

    // Email
    pub email_host: Option<String>,
    pub email_password: Option<String>,
    pub email_port: Option<i32>,
    pub email_username: Option<String>,

    // AWS
    pub aws_access_key: Option<String>,
    pub aws_address: Option<String>,
    pub aws_bucket_name: Option<String>,
    pub aws_bucket_region: Option<String>,
    pub aws_secret_key: Option<String>,

    // Google OAuth
    pub google: Option<bool>,
    pub google_callback: Option<String>,
    pub google_callback_fe_fail: Option<String>,
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,

    // Microsoft OAuth
    pub microsoft: Option<bool>,
    pub microsoft_callback: Option<String>,
    pub microsoft_callback_fe_fail: Option<String>,
    pub microsoft_client_id: Option<String>,
    pub microsoft_client_secret: Option<String>,
    pub microsoft_tenant_id: Option<String>,

    // API access tokens
    pub access_api_eshop_token: Option<String>,
    pub access_api_mobile_token: Option<String>,
    pub access_api_ticketing: Option<String>,
    pub access_api_oneid: Option<String>,
    pub access_api_internal: Option<String>,

    // Push notifications (OneSignal)
    pub one_signal_api_key: Option<String>,
    pub one_signal_app_id: Option<String>,

    // SMS
    pub nh_sms_gateway_key: Option<String>,

    // RabbitMQ
    pub rabbitmq_vhost: Option<String>,
    pub rabbitmq_consumer_count: Option<i32>,
    pub rabbitmq_multiple_prefetch: Option<i32>,
    pub rabbitmq_single_prefetch: Option<i32>,
    pub rabbitmq_ip: Option<String>,
    pub rabbitmq_pass: Option<String>,
    pub rabbitmq_user: Option<String>,

    // OneID integration
    pub oneid_integration: Option<bool>,
    pub oneid_api: Option<String>,
    pub oneid_pass: Option<String>,
    pub oneid_token: Option<String>,
    pub oneid_user: Option<String>,

    // Enigoo integration
    pub enigoo_integration: Option<bool>,
    pub enigoo_api: Option<String>,
    pub enigoo_token: Option<String>,
    pub enigoo_token_api: Option<String>,
    pub enigoo_channel: Option<String>,

    // SparkPost
    pub spark_post_api_key: Option<String>,
    pub spark_post_sub_account: Option<String>,
    pub spark_post_webhook_pass: Option<String>,
    pub spark_post_webhook_username: Option<String>,

    // ClickHouse
    pub clickhouse_password: Option<String>,
    pub clickhouse_username: Option<String>,

    // Esports
    pub esports_games_key: Option<String>,
    pub esports_games_api: Option<String>,

    // Other integrations / feature flags
    pub neon_integration: Option<bool>,
    pub ticket_portal_integration: Option<bool>,
    pub spartaid_integration: Option<bool>,
    pub futured_integration: Option<bool>,
    pub bonus_guideline: Option<bool>,
    pub ai_segments: Option<bool>,
    pub association_crm: Option<bool>,

    // Business rules
    pub highlight_before_expiration: Option<i32>,
    pub max_segment_recalculate: Option<i32>,
    pub automation_business_case_state: Option<bool>,
    pub automation_move_business_case: Option<AutomationMoveBusinessCase>,
    pub only_owner_can_edit_business_case: Option<bool>,
    #[schema(value_type = Option<Vec<Object>>)]
    pub subject_prefixes: Option<Vec<Bson>>,
    pub create_fan_from_ticket_attribute: Option<bool>,
    pub fan_import_matching_strategy: Option<String>,

    // Invoice notifications
    pub invoice_notification: Option<bool>,
    pub invoice_notification_days_before: Option<i32>,
    pub invoice_notification_email: Option<String>,

    // Frequency caps
    pub email_frequency_cap: Option<FrequencyCap>,
    pub message_frequency_cap: Option<FrequencyCap>,
    pub push_notification_frequency_cap: Option<FrequencyCap>,

    // Nested documents
    pub security: Option<Security>,
    pub webhooks: Option<Webhooks>,
    pub bi_url: Option<Vec<BiUrl>>,
    pub internal_features: Option<InternalFeatures>,
    /// Deeply nested, variable-shape GDPR translation map.
    #[schema(value_type = Object)]
    pub gdpr_translation: Option<Document>,
}

impl From<Tenant> for TenantResponse {
    fn from(t: Tenant) -> Self {
        Self {
            id: t.id.to_hex(),
            name: t.name,
            version: t.version,
            hostname: t.hostname,
            db_name: t.db_name,
            active: t.active,
            business_module: t.business_module,
            fans_module: t.fans_module,
            mobile_module: t.mobile_module,
            content_type: t.content_type,
            plan: t.plan,
            club_logo: t.club_logo,
            visualization: t.visualization,
            domain_name: t.domain_name,
            api_endpoint: t.api_endpoint,
            address: t.address,
            company: t.company,
            dic: t.dic,
            ico: t.ico,
            default_language: t.default_language,
            message_provider: t.message_provider,
            allow_default_gdpr: t.allow_default_gdpr,
            is_gina_enabled: t.is_gina_enabled,
            contract_rs_template: t.contract_rs_template,
            approval_email: t.approval_email,
            contract_email: t.contract_email,
            email_host: t.email_host,
            email_password: t.email_password,
            email_port: t.email_port,
            email_username: t.email_username,
            aws_access_key: t.aws_access_key,
            aws_address: t.aws_address,
            aws_bucket_name: t.aws_bucket_name,
            aws_bucket_region: t.aws_bucket_region,
            aws_secret_key: t.aws_secret_key,
            google: t.google,
            google_callback: t.google_callback,
            google_callback_fe_fail: t.google_callback_fe_fail,
            google_client_id: t.google_client_id,
            google_client_secret: t.google_client_secret,
            microsoft: t.microsoft,
            microsoft_callback: t.microsoft_callback,
            microsoft_callback_fe_fail: t.microsoft_callback_fe_fail,
            microsoft_client_id: t.microsoft_client_id,
            microsoft_client_secret: t.microsoft_client_secret,
            microsoft_tenant_id: t.microsoft_tenant_id,
            access_api_eshop_token: t.access_api_eshop_token,
            access_api_mobile_token: t.access_api_mobile_token,
            access_api_ticketing: t.access_api_ticketing,
            access_api_oneid: t.access_api_oneid,
            access_api_internal: t.access_api_internal,
            one_signal_api_key: t.one_signal_api_key,
            one_signal_app_id: t.one_signal_app_id,
            nh_sms_gateway_key: t.nh_sms_gateway_key,
            rabbitmq_vhost: t.rabbitmq_vhost,
            rabbitmq_consumer_count: t.rabbitmq_consumer_count,
            rabbitmq_multiple_prefetch: t.rabbitmq_multiple_prefetch,
            rabbitmq_single_prefetch: t.rabbitmq_single_prefetch,
            rabbitmq_ip: t.rabbitmq_ip,
            rabbitmq_pass: t.rabbitmq_pass,
            rabbitmq_user: t.rabbitmq_user,
            oneid_integration: t.oneid_integration,
            oneid_api: t.oneid_api,
            oneid_pass: t.oneid_pass,
            oneid_token: t.oneid_token,
            oneid_user: t.oneid_user,
            enigoo_integration: t.enigoo_integration,
            enigoo_api: t.enigoo_api,
            enigoo_token: t.enigoo_token,
            enigoo_token_api: t.enigoo_token_api,
            enigoo_channel: t.enigoo_channel,
            spark_post_api_key: t.spark_post_api_key,
            spark_post_sub_account: t.spark_post_sub_account,
            spark_post_webhook_pass: t.spark_post_webhook_pass,
            spark_post_webhook_username: t.spark_post_webhook_username,
            clickhouse_password: t.clickhouse_password,
            clickhouse_username: t.clickhouse_username,
            esports_games_key: t.esports_games_key,
            esports_games_api: t.esports_games_api,
            neon_integration: t.neon_integration,
            ticket_portal_integration: t.ticket_portal_integration,
            spartaid_integration: t.spartaid_integration,
            futured_integration: t.futured_integration,
            bonus_guideline: t.bonus_guideline,
            ai_segments: t.ai_segments,
            association_crm: t.association_crm,
            highlight_before_expiration: t.highlight_before_expiration,
            max_segment_recalculate: t.max_segment_recalculate,
            automation_business_case_state: t.automation_business_case_state,
            automation_move_business_case: t.automation_move_business_case,
            only_owner_can_edit_business_case: t.only_owner_can_edit_business_case,
            subject_prefixes: t.subject_prefixes,
            create_fan_from_ticket_attribute: t.create_fan_from_ticket_attribute,
            fan_import_matching_strategy: t.fan_import_matching_strategy,
            invoice_notification: t.invoice_notification,
            invoice_notification_days_before: t.invoice_notification_days_before,
            invoice_notification_email: t.invoice_notification_email,
            email_frequency_cap: t.email_frequency_cap,
            message_frequency_cap: t.message_frequency_cap,
            push_notification_frequency_cap: t.push_notification_frequency_cap,
            security: t.security,
            webhooks: t.webhooks,
            bi_url: t.bi_url,
            internal_features: t.internal_features,
            gdpr_translation: t.gdpr_translation,
        }
    }
}

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl MongoService {
    /// Return all tenants from `mt_admin.tenants`.
    pub async fn list_tenants(&self) -> Result<Vec<TenantResponse>, AppError> {
        let raw_docs: Vec<Document> = self
            .admin_db()
            .collection::<Document>("tenants")
            .find(None, None)
            .await?
            .try_collect()
            .await?;

        let mut tenants = Vec::with_capacity(raw_docs.len());
        for doc in raw_docs {
            let id = doc
                .get("_id")
                .and_then(|v| v.as_object_id())
                .map(|o| o.to_hex())
                .unwrap_or_else(|| "<unknown>".to_owned());
            let name = doc
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("<unknown>")
                .to_owned();
            let deser = mongodb::bson::de::Deserializer::new(Bson::Document(doc));
            match serde_path_to_error::deserialize::<_, Tenant>(deser) {
                Ok(tenant) => tenants.push(TenantResponse::from(tenant)),
                Err(err) => tracing::error!(
                    tenant_id = %id,
                    tenant_name = %name,
                    field = %err.path(),
                    error = %err.inner(),
                    "failed to deserialize tenant — skipping"
                ),
            }
        }

        Ok(tenants)
    }

    // Return a single tenant by ID
    pub async fn get_tenant(&self, tenant_id: &str) -> Result<TenantResponse, AppError> {
        let obj_id = ObjectId::parse_str(tenant_id)
            .map_err(|err| AppError::Internal(format!("Invalid tenant id: {err}")))?;
        let doc = self
            .admin_db()
            .collection::<Document>("tenants")
            .find_one(doc! { "_id": obj_id }, None)
            .await?
            .ok_or_else(|| AppError::NotFound)?;

        let deser = mongodb::bson::de::Deserializer::new(Bson::Document(doc));
        let tenant = serde_path_to_error::deserialize::<_, Tenant>(deser).map_err(|err| {
            tracing::error!(
                tenant_id = %tenant_id,
                field = %err.path(),
                error = %err.inner(),
                "failed to deserialize tenant"
            );
            AppError::Internal("Failed to deserialize tenant".to_owned())
        })?;

        Ok(TenantResponse::from(tenant))
    }

    /// Returns the number of fans that are not locked
    /// (`locked` is `false`, `null`, or absent — excludes `locked: true`).
    pub async fn get_tenant_fans_count(&self, tenant_db_name: &str) -> Result<u64, AppError> {
        let count = self
            .client
            .database(tenant_db_name)
            .collection::<Document>("fans")
            .count_documents(doc! { "locked": { "$ne": true } }, None)
            .await?;

        Ok(count)
    }

    /// Returns new fans registered in the last 30 days and a cumulative
    /// monthly time-series from the first fan ever up to now (including a
    /// leading zero point one month before the first registration).
    pub async fn get_tenant_fans_count_in_time(
        &self,
        tenant_db_name: &str,
    ) -> Result<(u64, Vec<FansDataPoint>), AppError> {
        use time::{Duration, OffsetDateTime};

        let collection = self
            .client
            .database(tenant_db_name)
            .collection::<Document>("fans");

        // --- new fans in last 30 days ---
        let thirty_days_ago = OffsetDateTime::now_utc() - Duration::days(30);
        let threshold_bson =
            bson::DateTime::from_millis(thirty_days_ago.unix_timestamp().saturating_mul(1_000));

        let new_fans_last_month = collection
            .count_documents(doc! { "createdAt": { "$gte": threshold_bson } }, None)
            .await?;

        // --- monthly fan registrations (aggregated) ---
        let pipeline = vec![
            doc! {
                "$match": { "createdAt": { "$exists": true, "$ne": null } }
            },
            doc! {
                "$group": {
                    "_id": {
                        "year":  { "$year":  "$createdAt" },
                        "month": { "$month": "$createdAt" }
                    },
                    "count": { "$sum": 1_i32 }
                }
            },
            doc! {
                "$sort": { "_id.year": 1_i32, "_id.month": 1_i32 }
            },
        ];

        let mut cursor = collection.aggregate(pipeline, None).await?;
        let mut monthly: Vec<MonthlyFanCount> = Vec::new();
        while let Some(raw) = cursor.try_next().await? {
            match bson::from_document::<MonthlyFanCount>(raw) {
                Ok(item) => monthly.push(item),
                Err(err) => tracing::warn!(error = %err, "skipping undeserializable monthly fan bucket"),
            }
        }

        // Build cumulative points; prepend a zero point one month before first.
        let mut points: Vec<FansDataPoint> = Vec::with_capacity(monthly.len().saturating_add(1));

        if !monthly.is_empty() {
            let (py, pm) = prev_month(monthly[0].id.year, monthly[0].id.month);
            points.push(FansDataPoint { timestamp: month_start_ms(py, pm), count: 0 });

            let mut running = 0_u64;
            for item in &monthly {
                running = running.saturating_add(
                    u64::try_from(item.count).unwrap_or(0),
                );
                points.push(FansDataPoint {
                    timestamp: month_start_ms(item.id.year, item.id.month),
                    count: running,
                });
            }
        }

        Ok((new_fans_last_month, points))
    }
}

// ---------------------------------------------------------------------------
// Distribution stats
// ---------------------------------------------------------------------------

/// A single slice in a percentage breakdown.
#[derive(Debug, Serialize, ToSchema)]
pub struct DistributionEntry {
    /// Human-readable label (e.g. "Samsung", "Female", "Prague").
    pub label: String,
    /// Raw document count for this slice.
    pub count: u64,
    /// Share of the total, 0–100 rounded to two decimal places.
    pub percentage: f64,
}

/// Fan breakdowns by device, city and gender.
#[derive(Debug, Serialize, ToSchema)]
pub struct FansDistributions {
    /// Most-common devices first; empty-string / null → "Unknown".
    pub devices: Vec<DistributionEntry>,
    /// Most-common cities first; empty-string / null → "Unknown".
    pub cities: Vec<DistributionEntry>,
    /// Female / Male / Unknown derived from the boolean `gender` field.
    pub genders: Vec<DistributionEntry>,
}

impl MongoService {
    /// Returns a breakdown of fans by device, city, and gender.
    pub async fn get_tenant_fans_distributions(
        &self,
        tenant_db_name: &str,
    ) -> Result<FansDistributions, AppError> {
        let col = self
            .client
            .database(tenant_db_name)
            .collection::<Document>("fans");

        let (devices, cities, genders) = tokio::try_join!(
            aggregate_string_dist(&col, "device"),
            aggregate_string_dist(&col, "city"),
            aggregate_gender_dist(&col),
        )?;

        Ok(FansDistributions { devices, cities, genders })
    }
}

/// Group `field` by value, normalise null/missing/"" → "Unknown", sort by count desc.
async fn aggregate_string_dist(
    col: &mongodb::Collection<Document>,
    field: &str,
) -> Result<Vec<DistributionEntry>, AppError> {
    let field_ref = format!("${field}");
    let pipeline = vec![
        doc! {
            "$group": {
                "_id": { "$ifNull": [field_ref, ""] },
                "count": { "$sum": 1_i32 }
            }
        },
        doc! { "$sort": { "count": -1_i32 } },
    ];

    let mut cursor = col.aggregate(pipeline, None).await?;
    let mut raw: Vec<(String, u64)> = Vec::new();
    let mut total = 0_u64;

    while let Some(doc) = cursor.try_next().await? {
        let count = bson_count(&doc);
        let label = doc
            .get("_id")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map_or_else(|| "Unknown".to_owned(), ToOwned::to_owned);
        total = total.saturating_add(count);
        raw.push((label, count));
    }

    Ok(to_distribution_entries(raw, total))
}

/// Group by the boolean `gender` field: true → Female, false → Male, other → Unknown.
async fn aggregate_gender_dist(
    col: &mongodb::Collection<Document>,
) -> Result<Vec<DistributionEntry>, AppError> {
    let pipeline = vec![doc! {
        "$group": { "_id": "$gender", "count": { "$sum": 1_i32 } }
    }];

    let mut cursor = col.aggregate(pipeline, None).await?;
    let mut raw: Vec<(String, u64)> = Vec::new();
    let mut total = 0_u64;

    while let Some(doc) = cursor.try_next().await? {
        let count = bson_count(&doc);
        let label = match doc.get("_id") {
            Some(Bson::Boolean(true))  => "Female".to_owned(),
            Some(Bson::Boolean(false)) => "Male".to_owned(),
            _                          => "Unknown".to_owned(),
        };
        total = total.saturating_add(count);
        raw.push((label, count));
    }

    // Stable order: Female → Male → Unknown
    raw.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(to_distribution_entries(raw, total))
}

/// Extract the `count` field from an aggregation result document.
fn bson_count(doc: &Document) -> u64 {
    doc.get("count")
        .and_then(|v| {
            v.as_i64()
                .or_else(|| v.as_i32().map(i64::from))
        })
        .and_then(|n| u64::try_from(n).ok())
        .unwrap_or(0)
}

/// Convert (label, count) pairs + total into `DistributionEntry` with percentages.
#[allow(clippy::arithmetic_side_effects)] // float division for percentage is intentional
fn to_distribution_entries(raw: Vec<(String, u64)>, total: u64) -> Vec<DistributionEntry> {
    raw.into_iter()
        .map(|(label, count)| {
            let percentage = if total == 0 {
                0.0_f64
            } else {
                let p = (count as f64) / (total as f64) * 100.0_f64;
                (p * 100.0_f64).round() / 100.0_f64 // 2 decimal places
            };
            DistributionEntry { label, count, percentage }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Helpers for time-series
// ---------------------------------------------------------------------------

/// One point in the fans-over-time series.
#[derive(Debug, Serialize, ToSchema)]
pub struct FansDataPoint {
    /// Unix timestamp in milliseconds (UTC midnight, 1st of the month).
    pub timestamp: i64,
    /// Cumulative fan count up to and including this month.
    pub count: u64,
}

#[derive(serde::Deserialize)]
struct MonthlyFanCount {
    #[serde(rename = "_id")]
    id: MonthlyId,
    count: i64,
}

#[derive(serde::Deserialize)]
struct MonthlyId {
    year: i32,
    month: i32,
}

fn prev_month(year: i32, month: i32) -> (i32, i32) {
    if month == 1 {
        (year.saturating_sub(1), 12)
    } else {
        (year, month.saturating_sub(1))
    }
}

fn month_start_ms(year: i32, month: i32) -> i64 {
    use time::{Date, Month, PrimitiveDateTime, Time};
    let m = u8::try_from(month)
        .ok()
        .and_then(|m| Month::try_from(m).ok())
        .unwrap_or(Month::January);
    let date = Date::from_calendar_date(year, m, 1).unwrap_or(Date::MIN);
    PrimitiveDateTime::new(date, Time::MIDNIGHT)
        .assume_utc()
        .unix_timestamp()
        .saturating_mul(1_000)
}
