use std::ops::DerefMut;

use axum::http::StatusCode;
use chrono::Utc;
use md5::compute;
use rand::Rng;
use reqwest::Client;

use crate::sso::bean::{UserDetail, UserDetailResult};

pub async fn query_user_detail(satoken: &str) -> Result<UserDetail, StatusCode> {
    let client = Client::new();
    let mut get_data_param = vec![
        ("apiType", "userToken"),
        ("apiValue", satoken),
        ("timestamp", &Utc::now().timestamp().to_string()),
        ("nonce", &get_random_string(32)),
    ];
    let sign = create_sign(&mut get_data_param);
    get_data_param.push(("sign", &sign));
    let re = client
        .get("http://sso.beta.micun.cn/sso/data")
        .form(&get_data_param)
        .send()
        .await
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
    let re = re
        .json::<UserDetailResult>()
        .await
        .map_err(|e| StatusCode::UNAUTHORIZED)?;
    if re.code.is_none() || re.code.is_some_and(|x| x != 0) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    match re.data {
        Some(user) => Ok(user),
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

// public static String getRandomString(int length) {
// String str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
// StringBuilder sb = new StringBuilder();
// for (int i = 0; i < length; i++) {
// int number = ThreadLocalRandom.current().nextInt(62);
// sb.append(str.charAt(number));
// }
// return sb.toString();
// }
fn get_random_string(len: u32) -> String {
    let s = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut string = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        let ran = rng.gen_range(0..s.len());
        // 生成一个随机的 u32 数字
        let ch = s.chars().nth(ran).unwrap();
        string.push(ch);
    }
    string
}

// public String createSign(Map<String, ?> paramsMap) {
// String secretKey = getSecretKey();
// SaSignException.throwByNull(secretKey, "参与参数签名的秘钥不可为空", SaErrorCode.CODE_12201);
//
// 如果调用者不小心传入了 sign 参数，则此处需要将 sign 参数排除在外
// if(paramsMap.containsKey(sign)) {
// 为了保证不影响原有的 paramsMap，此处需要再复制一份
// paramsMap = new TreeMap<>(paramsMap);
// paramsMap.remove(sign);
// }
//
// 计算签名
// String paramsStr = joinParamsDictSort(paramsMap);
// String fullStr = paramsStr + "&" + key + "=" + secretKey;
// return abstractStr(fullStr);
// }
fn create_sign(params: &mut Vec<(&str, &str)>) -> String {
    params.sort_by(|a, b| a.0.cmp(b.0));

    let mut s = String::new();
    params.iter().for_each(|e| {
        let string = format!("{}={}", e.0, e.1);
        s.push_str(&string);
        s.push('&');
    });

    s.push_str("key=2VWGTBJKDynjxM5TMUxKLw4kQbMDfWZB");
    get_md5(&s)
}

/**
 * md5加密
 * @param str 指定字符串
 * @return 加密后的字符串
 */
// public static String md5(String str) {
// str = (str == null ? "" : str);
// char[] hexDigits = { '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f' };
// try {
// byte[] btInput = str.getBytes();
// MessageDigest mdInst = MessageDigest.getInstance("MD5");
// mdInst.update(btInput);
// byte[] md = mdInst.digest();
// int j = md.length;
// char[] strA = new char[j * 2];
// int k = 0;
// for (byte byte0 : md) {
// strA[k++] = hexDigits[byte0 >>> 4 & 0xf];
// strA[k++] = hexDigits[byte0 & 0xf];
// }
// return new String(strA);
// } catch (Exception e) {
// throw new SaTokenException(e).setCode(SaErrorCode.CODE_12111);
// }
// }

fn get_md5(s: &str) -> String {
    let hex_digits: [u8; 16] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e',
        b'f',
    ];
    let data = s.as_bytes();
    let mut digest = compute(data);
    let md = digest.deref_mut();
    let mut result = String::new();
    for byte0 in md {
        result.push(hex_digits[*byte0 >> 4 & 0xF]);
        result.push(hex_digits[*byte0 & 0xF]);
    }

    result
}
