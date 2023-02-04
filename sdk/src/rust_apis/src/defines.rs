// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::tee_defines::TeeUuid;

pub struct TeeMutexHandle {
    _unused: *mut u8,
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpawnUuid {
    pub uuid_valid: u64,
    pub uuid: TeeUuid,
}

#[repr(u32)]
#[non_exhaustive]
#[doc = " Login type definitions"]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TeeLoginMethod {
    TEE_LOGIN_PUBLIC = 0,
    TEE_LOGIN_USER = 1,
    TEE_LOGIN_GROUP = 2,
    TEE_LOGIN_APPLICATION = 4,
    TEE_LOGIN_USER_APPLICATION = 5,
    TEE_LOGIN_GROUP_APPLICATION = 6,
    #[doc = " iTrustee defined Login type"]
    TEE_LOGIN_IDENTIFY = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeeDateTime {
    pub seconds: i32,
    pub millis: i32,
    pub min: i32,
    pub hour: i32,
    pub day: i32,
    pub month: i32,
    pub year: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeeTimerProperty {
    pub type_: u32,
    pub timer_id: u32,
    pub timer_class: u32,
    pub reserved2: u32,
}
