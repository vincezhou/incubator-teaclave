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

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::prelude::v1::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum HandleFileCommand {
    Download,
    Upload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileAgentRequest {
    pub cmd: HandleFileCommand,
    pub info: Vec<HandleFileInfo>,
    pub fusion_base: PathBuf,
}

impl FileAgentRequest {
    pub fn new<T: IntoIterator>(
        cmd: HandleFileCommand,
        info: T,
        fusion_base: impl AsRef<Path>,
    ) -> Self
    where
        <T as IntoIterator>::Item: Into<HandleFileInfo>,
    {
        FileAgentRequest {
            cmd,
            info: info.into_iter().map(|x| x.into()).collect(),
            fusion_base: fusion_base.as_ref().to_owned(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandleFileInfo {
    pub local: PathBuf,
    pub remote: url::Url,
}

impl HandleFileInfo {
    pub fn new(local: impl AsRef<std::path::Path>, remote: &url::Url) -> Self {
        HandleFileInfo {
            local: local.as_ref().to_owned(),
            remote: remote.to_owned(),
        }
    }
}

impl std::convert::From<&HandleFileInfo> for HandleFileInfo {
    fn from(info: &HandleFileInfo) -> HandleFileInfo {
        info.clone()
    }
}
