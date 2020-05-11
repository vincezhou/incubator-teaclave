// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::prelude::v1::*;
use teaclave_attestation::verifier;
use teaclave_config::build::AS_ROOT_CA_CERT;
use teaclave_config::RuntimeConfig;
use teaclave_proto::teaclave_authentication_service::*;
use teaclave_proto::teaclave_common::*;
use teaclave_rpc::config::SgxTrustedTlsClientConfig;
use teaclave_rpc::endpoint::Endpoint;
use teaclave_test_utils::test_case;
use teaclave_types::EnclaveInfo;

fn get_api_client() -> TeaclaveAuthenticationApiClient {
    let runtime_config = RuntimeConfig::from_toml("runtime.config.toml").expect("runtime");
    let enclave_info = EnclaveInfo::from_bytes(&runtime_config.audit.enclave_info_bytes);
    let enclave_attr = enclave_info
        .get_enclave_attr("teaclave_authentication_service")
        .expect("authentication");
    let config = SgxTrustedTlsClientConfig::new().attestation_report_verifier(
        vec![enclave_attr],
        AS_ROOT_CA_CERT,
        verifier::universal_quote_verifier,
    );

    let channel = Endpoint::new("localhost:7776")
        .config(config)
        .connect()
        .unwrap();
    TeaclaveAuthenticationApiClient::new(channel).unwrap()
}

fn get_internal_client() -> TeaclaveAuthenticationInternalClient {
    let runtime_config = RuntimeConfig::from_toml("runtime.config.toml").expect("runtime");
    let enclave_info = EnclaveInfo::from_bytes(&runtime_config.audit.enclave_info_bytes);
    let enclave_attr = enclave_info
        .get_enclave_attr("teaclave_authentication_service")
        .expect("authentication");
    let config = SgxTrustedTlsClientConfig::new().attestation_report_verifier(
        vec![enclave_attr],
        AS_ROOT_CA_CERT,
        verifier::universal_quote_verifier,
    );

    let channel = Endpoint::new(
        &runtime_config
            .internal_endpoints
            .authentication
            .advertised_address,
    )
    .config(config)
    .connect()
    .unwrap();
    TeaclaveAuthenticationInternalClient::new(channel).unwrap()
}

#[test_case]
fn test_login_success() {
    let mut client = get_api_client();
    let request = UserRegisterRequest::new("test_login_id1", "test_password");
    let response_result = client.user_register(request);
    assert!(response_result.is_ok());

    let request = UserLoginRequest::new("test_login_id1", "test_password");
    let response_result = client.user_login(request);
    info!("{:?}", response_result);
    assert!(response_result.is_ok());
}

#[test_case]
fn test_login_fail() {
    let mut client = get_api_client();
    let request = UserRegisterRequest::new("test_login_id2", "test_password");
    let response_result = client.user_register(request);
    assert!(response_result.is_ok());

    let request = UserLoginRequest::new("test_login_id2", "wrong_password");
    let response_result = client.user_login(request);
    info!("{:?}", response_result);
    assert!(response_result.is_err());
}

#[test_case]
fn test_authenticate_success() {
    let mut api_client = get_api_client();
    let mut internal_client = get_internal_client();
    let request = UserRegisterRequest::new("test_authenticate_id1", "test_password");
    let response_result = api_client.user_register(request);
    assert!(response_result.is_ok());

    let request = UserLoginRequest::new("test_authenticate_id1", "test_password");
    let response_result = api_client.user_login(request);
    assert!(response_result.is_ok());
    let credential = UserCredential::new("test_authenticate_id1", response_result.unwrap().token);
    let request = UserAuthenticateRequest::new(credential);
    let response_result = internal_client.user_authenticate(request);
    info!("{:?}", response_result);
    assert!(response_result.unwrap().accept);
}

#[test_case]
fn test_authenticate_fail() {
    let mut api_client = get_api_client();
    let mut internal_client = get_internal_client();

    let request = UserRegisterRequest::new("test_authenticate_id2", "test_password");
    let response_result = api_client.user_register(request);
    assert!(response_result.is_ok());

    let credential = UserCredential::new("test_authenticate_id2", "wrong_token");
    let request = UserAuthenticateRequest::new(credential);
    let response_result = internal_client.user_authenticate(request);
    info!("{:?}", response_result);
    assert!(!response_result.unwrap().accept);
}

#[test_case]
fn test_register_success() {
    let mut client = get_api_client();
    let request = UserRegisterRequest::new("test_register_id1", "test_password");
    let response_result = client.user_register(request);
    info!("{:?}", response_result);
    assert!(response_result.is_ok());
}

#[test_case]
fn test_register_fail() {
    let mut client = get_api_client();
    let request = UserRegisterRequest::new("test_register_id2", "test_password");
    let response_result = client.user_register(request);
    assert!(response_result.is_ok());
    let request = UserRegisterRequest::new("test_register_id2", "test_password");
    let response_result = client.user_register(request);
    info!("{:?}", response_result);
    assert!(response_result.is_err());
}
