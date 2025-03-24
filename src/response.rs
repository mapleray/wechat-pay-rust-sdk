use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub trait ResponseTrait: DeserializeOwned {}

#[derive(Debug, Deserialize)]
pub struct NativeResponse {
    pub code: Option<String>,
    pub message: Option<String>,
    ///【支付跳转链接】 h5_url为拉起微信支付收银台的中间页面，可通过访问该URL来拉起微信客户端，完成支付，h5_url的有效期为5分钟。
    pub code_url: Option<String>,
}

impl ResponseTrait for NativeResponse {}

#[derive(Debug, Deserialize)]
pub struct JsapiResponse {
    pub code: Option<String>,
    pub message: Option<String>,
    ///【预支付交易会话标识】 预支付交易会话标识。用于后续接口调用中使用，该值有效期为2小时
    pub prepay_id: Option<String>,
    ///【签名数据】
    pub sign_data: Option<SignData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignData {
    pub app_id: String,
    pub sign_type: String,
    pub package: String,
    pub nonce_str: String,
    pub timestamp: String,
    pub pay_sign: String,
}

impl ResponseTrait for JsapiResponse {}

#[derive(Debug, Deserialize)]
pub struct AppResponse {
    pub code: Option<String>,
    pub message: Option<String>,
    ///【预支付交易会话标识】 预支付交易会话标识。用于后续接口调用中使用，该值有效期为2小时
    pub prepay_id: Option<String>,
    ///【签名数据】
    pub sign_data: Option<SignData>,
}

impl ResponseTrait for AppResponse {}

#[derive(Debug, Deserialize)]
pub struct MicroResponse {
    pub code: Option<String>,
    pub message: Option<String>,
    ///【预支付交易会话标识】 预支付交易会话标识。用于后续接口调用中使用，该值有效期为2小时
    pub prepay_id: Option<String>,
    ///【签名数据】
    pub sign_data: Option<SignData>,
}

impl ResponseTrait for MicroResponse {}

#[derive(Debug, Deserialize)]
pub struct H5Response {
    pub code: Option<String>,
    pub message: Option<String>,
    ///【二维码链接】 此URL用于生成支付二维码，然后提供给用户扫码支付。
    /// 注意：code_url并非固定值，使用时按照URL格式转成二维码即可。
    pub h5_url: Option<String>,
}

impl ResponseTrait for H5Response {}

#[derive(Debug, Clone, Deserialize)]
pub struct EncryptCertificate {
    pub algorithm: String,
    pub nonce: String,
    pub associated_data: String,
    pub ciphertext: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Certificate {
    pub serial_no: String,
    pub effective_time: String,
    pub expire_time: String,
    pub encrypt_certificate: EncryptCertificate,
}

#[derive(Debug, Deserialize)]
pub struct CertificateResponse {
    pub data: Option<Vec<Certificate>>,
}

impl ResponseTrait for CertificateResponse {}

#[derive(Debug, Deserialize)]
pub struct WechatPayErrorDetail {
    /// 指示错误参数的位置
    pub field: Option<String>,
    /// 错误的值
    pub value: Option<serde_json::Value>,
    /// 具体错误原因
    pub issue: Option<String>,
    /// 出错的位置
    pub location: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransferBillsResponse {
    pub code: Option<String>,
    pub message: Option<String>,
    pub detail: Option<WechatPayErrorDetail>,
    // 【商户单号】 商户系统内部的商家单号，要求此参数只能由数字、大小写字母组成，在商户系统内部唯一
    pub out_bill_no: String,
    // 【微信转账单号】 微信转账单号，微信商家转账系统返回的唯一标识
    pub transfer_bill_no: String,
    // 【单据创建时间】 单据受理成功时返回，按照使用rfc3339所定义的格式，格式为yyyy-MM-DDThh:mm:ss+TIMEZONE
    pub create_time: String,
    // 【单据状态】 商家转账订单状态
    pub state: String,
    // 【失败原因】 订单已失败或者已退资金时，会返回订单失败原因
    pub fail_reason: Option<String>,
    // 【跳转领取页面的package信息】 跳转微信支付收款页的package信息，APP调起用户确认收款或者JSAPI调起用户确认收款 时需要使用的参数。
    pub package_info: Option<String>,
}

impl ResponseTrait for TransferBillsResponse {}
