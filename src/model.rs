use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub trait ParamsTrait {
    fn to_json(&self) -> String;
}

#[derive(Serialize, Debug, Clone)]
pub enum Currency {
    CNY,
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::CNY => write!(f, "CNY"),
        }
    }
}

unsafe impl Send for Currency {}

unsafe impl Sync for Currency {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AmountInfo {
    ///【标价金额】 订单总金额，单位为分。
    pub total: i32,
}

impl From<i32> for AmountInfo {
    fn from(value: i32) -> Self {
        Self { total: value }
    }
}

unsafe impl Send for AmountInfo {}

unsafe impl Sync for AmountInfo {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayerInfo {
    ///【用户标识】 用户在直连商户appid下的唯一标识。
    pub openid: String,
}

impl From<&str> for PayerInfo {
    fn from(value: &str) -> Self {
        Self {
            openid: value.to_string(),
        }
    }
}

unsafe impl Send for PayerInfo {}

unsafe impl Sync for PayerInfo {}

#[derive(Serialize, Debug, Clone)]
pub struct GoodsDetail {
    ///【商户侧商品编码】 由半角的大小写字母、数字、中划线、下划线中的一种或几种组成。
    pub merchant_goods_id: String,
    ///【商品数量】 用户购买的数量
    pub quantity: i32,
    ///【商品单价】 单位为：分。如果商户有优惠，需传输商户优惠后的单价(例如：用户对一笔100元的订单使用了商场发的纸质优惠券100-50，则活动商品的单价应为原单价-50)
    pub unit_price: i32,
    ///【微信支付商品编码】 微信支付定义的统一商品编号（没有可不传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechatpay_goods_id: Option<String>,
    ///【商品名称】 商品的实际名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_name: Option<String>,
}

unsafe impl Send for GoodsDetail {}

unsafe impl Sync for GoodsDetail {}

#[derive(Serialize, Debug, Clone)]
pub struct OrderDetail {
    ///【订单原价】
    /// 1、商户侧一张小票订单可能被分多次支付，订单原价用于记录整张小票的交易金额。
    /// 2、当订单原价与支付金额不相等，则不享受优惠。
    /// 3、该字段主要用于防止同一张小票分多次支付，以享受多次优惠的情况，正常支付订单不必上传此参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_price: Option<i32>,
    ///【商品小票ID】 商家小票ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    ///【单品列表】 单品列表信息,条目个数限制：【1，6000】
    pub goods_detail: Vec<GoodsDetail>,
}

unsafe impl Send for OrderDetail {}

unsafe impl Sync for OrderDetail {}

#[derive(Serialize, Debug, Clone)]
pub struct StoreInfo {
    ///【门店编号】 商户侧门店编号
    pub id: String,
    ///【门店名称】 商户侧门店名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///【地区编码】 地区编码，详细请见省市区编号对照表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_code: Option<String>,
    ///【详细地址】 详细的商户门店地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SceneInfo {
    ///【用户终端IP】 用户的客户端IP，支持IPv4和IPv6两种格式的IP地址。
    pub payer_client_ip: String,
    ///【商户端设备号】 商户端设备号（门店号或收银设备ID）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    ///【商户门店信息】 商户门店信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_info: Option<StoreInfo>,
}

#[derive(Serialize, Debug, Clone)]
pub enum H5Type {
    Ios,
    Android,
    Wap,
}

impl Display for H5Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            H5Type::Ios => write!(f, "iOS"),
            H5Type::Android => write!(f, "Android"),
            H5Type::Wap => write!(f, "Wap"),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct H5Info {
    ///【场景类型】 场景类型
    #[serde(rename = "type")]
    pub h5_type: String,
    ///【应用名称】 应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    ///【网站URL】 网站URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_url: Option<String>,
    ///【iOS平台BundleID】 iOS平台BundleID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    ///【Android平台PackageName】 Android平台PackageName
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct H5SceneInfo {
    ///【用户终端IP】 用户的客户端IP，支持IPv4和IPv6两种格式的IP地址。
    pub payer_client_ip: String,
    ///【H5场景信息】
    pub h5_info: H5Info,
    ///【商户端设备号】 商户端设备号（门店号或收银设备ID）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    ///【商户门店信息】 商户门店信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_info: Option<StoreInfo>,
}

impl H5SceneInfo {
    pub fn new<S: AsRef<str>>(payer_client_ip: S, app_name: S, app_url: S) -> Self {
        Self {
            payer_client_ip: payer_client_ip.as_ref().to_string(),
            h5_info: H5Info {
                h5_type: H5Type::Wap.to_string(),
                app_name: Some(app_name.as_ref().to_string()),
                app_url: Some(app_url.as_ref().to_string()),
                bundle_id: None,
                package_name: None,
            },
            device_id: None,
            store_info: None,
        }
    }
}

unsafe impl Send for SceneInfo {}

unsafe impl Sync for SceneInfo {}

impl ParamsTrait for SceneInfo {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct JsapiParams {
    ///【AppID】微信开放平台(移动应用)或微信公众平台(小程序、公众号)为开发者的应用程序提供的唯一标识。
    pub appid: String,
    ///【商户号】 微信支付商户号
    pub mchid: String,
    ///【商品描述】 商品描述
    pub description: String,
    ///【商户订单号】 商户系统内部订单号，只能是数字、大小写字母_-*且在同一个商户号下唯一。
    pub out_trade_no: String,
    ///【订单金额】 订单金额信息
    pub amount: AmountInfo,
    ///【支付者】 支付者信息
    pub payer: PayerInfo,
    ///【附加数据】 附加数据，在查询API和支付通知中原样返回，可作为自定义参数使用，实际情况下只有支付完成状态才会返回该字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    ///【优惠功能】 优惠功能
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<OrderDetail>,
    ///【交易结束时间】 订单失效时间，遵循rfc3339标准格式，格式为yyyy-MM-DDTHH:mm:ss+TIMEZONE，yyyy-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日13点29分35秒。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    ///【场景信息】 支付场景描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<SceneInfo>,
}

impl ParamsTrait for JsapiParams {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct MicroParams {
    ///【AppID】微信开放平台(移动应用)或微信公众平台(小程序、公众号)为开发者的应用程序提供的唯一标识。
    pub appid: String,
    ///【商户号】 微信支付商户号
    pub mchid: String,
    ///【商品描述】 商品描述
    pub description: String,
    ///【商户订单号】 商户系统内部订单号，只能是数字、大小写字母_-*且在同一个商户号下唯一。
    pub out_trade_no: String,
    ///【订单金额】 订单金额信息
    pub amount: AmountInfo,
    ///【支付者】 支付者信息
    pub payer: PayerInfo,
    ///【附加数据】 附加数据，在查询API和支付通知中原样返回，可作为自定义参数使用，实际情况下只有支付完成状态才会返回该字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    ///【优惠功能】 优惠功能
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<OrderDetail>,
    ///【交易结束时间】 订单失效时间，遵循rfc3339标准格式，格式为yyyy-MM-DDTHH:mm:ss+TIMEZONE，yyyy-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日13点29分35秒。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    ///【场景信息】 支付场景描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<SceneInfo>,
}

impl ParamsTrait for MicroParams {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl MicroParams {
    pub fn new<S: AsRef<str>>(
        appid: S,
        mchid: S,
        description: S,
        out_trade_no: S,
        amount: AmountInfo,
        payer: PayerInfo,
    ) -> Self {
        Self {
            appid: appid.as_ref().to_string(),
            mchid: mchid.as_ref().to_string(),
            description: description.as_ref().to_string(),
            out_trade_no: out_trade_no.as_ref().to_string(),
            amount,
            payer,
            time_expire: None,
            attach: None,
            detail: None,
            scene_info: None,
        }
    }
}

impl JsapiParams {
    pub fn new<S: AsRef<str>>(
        appid: S,
        mchid: S,
        description: S,
        out_trade_no: S,
        amount: AmountInfo,
        payer: PayerInfo,
    ) -> Self {
        Self {
            appid: appid.as_ref().to_string(),
            mchid: mchid.as_ref().to_string(),
            description: description.as_ref().to_string(),
            out_trade_no: out_trade_no.as_ref().to_string(),
            amount,
            payer,
            time_expire: None,
            attach: None,
            detail: None,
            scene_info: None,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct SettleInfo {
    ///【是否指定分账】 是否指定分账，
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_sharing: Option<bool>,
}

unsafe impl Send for SettleInfo {}

unsafe impl Sync for SettleInfo {}

#[derive(Serialize, Debug, Clone)]
pub struct NativeParams {
    ///【AppID】微信开放平台(移动应用)或微信公众平台(小程序、公众号)为开发者的应用程序提供的唯一标识。
    pub appid: String,
    ///【商户号】 微信支付商户号
    pub mchid: String,
    ///【商品描述】 商品描述
    pub description: String,
    ///【通知地址】 异步接收微信支付结果通知的回调地址，通知URL必须为外网可访问的URL，不能携带参数。 公网域名必须为HTTPS，如果是走专线接入，使用专线NAT IP或者私有回调域名可使用HTTP
    pub notify_url: String,
    ///【商户订单号】 商户系统内部订单号，只能是数字、大小写字母_-*且在同一个商户号下唯一。
    pub out_trade_no: String,
    ///【订单金额】 订单金额信息
    pub amount: AmountInfo,
    ///【交易结束时间】 订单失效时间，遵循rfc3339标准格式，格式为yyyy-MM-DDTHH:mm:ss+TIMEZONE，yyyy-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日13点29分35秒。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    ///【附加数据】 附加数据，在查询API和支付通知中原样返回，可作为自定义参数使用，实际情况下只有支付完成状态才会返回该字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    ///【订单优惠标记】 商品标记，代金券或立减优惠功能的参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    ///【电子发票入口开放标识】 传入true时，支付成功消息和支付详情页将出现开票入口。需要在微信支付商户平台或微信公众平台开通电子发票功能，传此字段才可生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_fapiao: Option<bool>,
    ///【场景信息】 支付场景描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<SceneInfo>,
    ///【结算信息】 结算信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<SettleInfo>,
}

impl ParamsTrait for NativeParams {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct AppParams {
    ///【AppID】微信开放平台(移动应用)或微信公众平台(小程序、公众号)为开发者的应用程序提供的唯一标识。
    pub appid: String,
    ///【商户号】 微信支付商户号
    pub mchid: String,
    ///【商品描述】 商品描述
    pub description: String,
    ///【通知地址】 异步接收微信支付结果通知的回调地址，通知URL必须为外网可访问的URL，不能携带参数。 公网域名必须为HTTPS，如果是走专线接入，使用专线NAT IP或者私有回调域名可使用HTTP
    pub notify_url: String,
    ///【商户订单号】 商户系统内部订单号，只能是数字、大小写字母_-*且在同一个商户号下唯一。
    pub out_trade_no: String,
    ///【订单金额】 订单金额信息
    pub amount: AmountInfo,
    ///【交易结束时间】 订单失效时间，遵循rfc3339标准格式，格式为yyyy-MM-DDTHH:mm:ss+TIMEZONE，yyyy-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日13点29分35秒。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    ///【附加数据】 附加数据，在查询API和支付通知中原样返回，可作为自定义参数使用，实际情况下只有支付完成状态才会返回该字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    ///【订单优惠标记】 商品标记，代金券或立减优惠功能的参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    ///【电子发票入口开放标识】 传入true时，支付成功消息和支付详情页将出现开票入口。需要在微信支付商户平台或微信公众平台开通电子发票功能，传此字段才可生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_fapiao: Option<bool>,
    ///【优惠功能】 优惠功能
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<OrderDetail>,
    ///【场景信息】 支付场景描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<SceneInfo>,
    ///【结算信息】 结算信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<SettleInfo>,
}

impl ParamsTrait for AppParams {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl AppParams {
    pub fn new<S: AsRef<str>>(
        appid: S,
        mchid: S,
        notify_url: S,
        description: S,
        out_trade_no: S,
        amount: AmountInfo,
    ) -> Self {
        Self {
            appid: appid.as_ref().to_string(),
            mchid: mchid.as_ref().to_string(),
            description: description.as_ref().to_string(),
            notify_url: notify_url.as_ref().to_string(),
            out_trade_no: out_trade_no.as_ref().to_string(),
            amount,
            time_expire: None,
            attach: None,
            goods_tag: None,
            support_fapiao: None,
            detail: None,
            scene_info: None,
            settle_info: None,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct H5Params {
    ///【AppID】微信开放平台(移动应用)或微信公众平台(小程序、公众号)为开发者的应用程序提供的唯一标识。
    pub appid: String,
    ///【商户号】 微信支付商户号
    pub mchid: String,
    ///【商品描述】 商品描述
    pub description: String,
    ///【通知地址】 异步接收微信支付结果通知的回调地址，通知URL必须为外网可访问的URL，不能携带参数。 公网域名必须为HTTPS，如果是走专线接入，使用专线NAT IP或者私有回调域名可使用HTTP
    pub notify_url: String,
    ///【商户订单号】 商户系统内部订单号，只能是数字、大小写字母_-*且在同一个商户号下唯一。
    pub out_trade_no: String,
    ///【订单金额】 订单金额信息
    pub amount: AmountInfo,
    ///【交易结束时间】 订单失效时间，遵循rfc3339标准格式，格式为yyyy-MM-DDTHH:mm:ss+TIMEZONE，yyyy-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日13点29分35秒。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    ///【附加数据】 附加数据，在查询API和支付通知中原样返回，可作为自定义参数使用，实际情况下只有支付完成状态才会返回该字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    ///【订单优惠标记】 商品标记，代金券或立减优惠功能的参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    ///【电子发票入口开放标识】 传入true时，支付成功消息和支付详情页将出现开票入口。需要在微信支付商户平台或微信公众平台开通电子发票功能，传此字段才可生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_fapiao: Option<bool>,
    ///【场景信息】 支付场景描述
    pub scene_info: H5SceneInfo,
    ///【结算信息】 结算信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<SettleInfo>,
}

impl ParamsTrait for H5Params {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl H5Params {
    pub fn new<S: AsRef<str>>(
        appid: S,
        mchid: S,
        notify_url: S,
        description: S,
        out_trade_no: S,
        amount: AmountInfo,
        scene_info: H5SceneInfo,
    ) -> Self {
        Self {
            appid: appid.as_ref().to_string(),
            mchid: mchid.as_ref().to_string(),
            notify_url: notify_url.as_ref().to_string(),
            description: description.as_ref().to_string(),
            out_trade_no: out_trade_no.as_ref().to_string(),
            amount,
            time_expire: None,
            attach: None,
            goods_tag: None,
            support_fapiao: None,
            scene_info,
            settle_info: None,
        }
    }
}

impl NativeParams {
    pub fn new<S: AsRef<str>>(
        appid: S,
        mchid: S,
        notify_url: S,
        description: S,
        out_trade_no: S,
        amount: AmountInfo,
    ) -> Self {
        Self {
            appid: appid.as_ref().to_string(),
            mchid: mchid.as_ref().to_string(),
            notify_url: notify_url.as_ref().to_string(),
            description: description.as_ref().to_string(),
            out_trade_no: out_trade_no.as_ref().to_string(),
            amount,
            time_expire: None,
            attach: None,
            goods_tag: None,
            support_fapiao: None,
            scene_info: None,
            settle_info: None,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct TransferSceneReportInfo {
    // 【信息类型】 不能超过15个字符，商户所属转账场景下的信息类型，此字段内容为固定值，需严格按照转账场景报备信息字段说明传参。
    pub info_type: String,
    // 【信息内容】 不能超过32个字符，商户所属转账场景下的信息内容，商户可按实际业务场景自定义传参，需严格按照转账场景报备信息字段说明传参。
    pub info_content: String,
}

unsafe impl Send for TransferSceneReportInfo {}

unsafe impl Sync for TransferSceneReportInfo {}

#[derive(Serialize, Debug, Clone)]
pub struct TransferBillsParams {
    ///【商户AppID】 微信支付商户号绑定的appid
    pub appid: String,
    ///【商户单号】 商户系统内部的商家单号，要求此参数只能由数字、大小写字母组成，在商户系统内部唯一
    pub out_bill_no: String,
    ///【转账场景ID】 该笔转账使用的转账场景，可前往“商户平台-产品中心-商家转账”中申请。如：1001-现金营销
    pub transfer_scene_id: String,
    ///【收款用户OpenID】 用户在商户appid下的唯一标识。发起转账前需获取到用户的OpenID
    pub openid: String,
    ///【收款用户姓名】 收款方真实姓名。需要加密传入，支持标准RSA算法和国密算法，公钥由微信侧提供。
    ///    转账金额 >= 2,000元时，该笔明细必须填写
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    ///【转账金额】 转账金额单位为“分”。
    pub transfer_amount: i32,
    ///【通知地址】 异步接收微信支付结果通知的回调地址，通知URL必须为外网可访问的URL，不能携带参数。 公网域名必须为HTTPS，如果是走专线接入，使用专线NAT IP或者私有回调域名可使用HTTP
    pub notify_url: String,
    ///【转账备注】 转账备注，用户收款时可见该备注信息，UTF8编码，最多允许32个字符
    pub transfer_remark: String,

    ///【用户收款感知】 用户收款时感知到的收款原因将根据转账场景自动展示默认内容。如有其他展示需求，可在本字段传入。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_recv_perception: Option<String>,
    ///【转账场景报备信息】 各转账场景下需报备的内容，商户需要按照所属转账场景规则传参，详见转账场景报备信息字段说明。
    pub transfer_scene_report_infos: Vec<TransferSceneReportInfo>,
}

impl ParamsTrait for TransferBillsParams {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

unsafe impl Send for TransferBillsParams {}

unsafe impl Sync for TransferBillsParams {}

unsafe impl Send for NativeParams {}

unsafe impl Sync for NativeParams {}

unsafe impl Send for JsapiParams {}

unsafe impl Sync for JsapiParams {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WechatPayNotifySource {
    pub algorithm: String,
    pub ciphertext: String,
    pub associated_data: Option<String>,
    pub original_type: String,
    pub nonce: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WechatPayNotify {
    pub id: String,
    pub create_time: String,
    pub event_type: String,
    pub resource_type: String,
    pub resource: WechatPayNotifySource,
    pub summary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WechatPayDecodeData {
    pub mchid: String,
    pub appid: String,
    pub out_trade_no: String,
    pub transaction_id: String,
    pub trade_type: String,
    pub trade_state: String,
    pub trade_state_desc: String,
    pub bank_type: String,
    pub attach: String,
    pub success_time: String,
    pub payer: PayerInfo,
    pub amount: AmountInfo,
}
