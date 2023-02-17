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

#[repr(transparent)]
#[non_exhaustive]
#[doc = " Login type definitions"]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TeeLoginMethod(u32);

impl TeeLoginMethod {
    pub const TEE_LOGIN_PUBLIC: Self = Self(0);
    pub const TEE_LOGIN_USER: Self = Self(1);
    pub const TEE_LOGIN_GROUP: Self = Self(2);
    pub const TEE_LOGIN_APPLICATION: Self = Self(4);
    pub const TEE_LOGIN_USER_APPLICATION: Self = Self(5);
    pub const TEE_LOGIN_GROUP_APPLICATION: Self = Self(6);
    #[doc = " iTrustee defined Login type"]
    pub const TEE_LOGIN_IDENTIFY: Self = Self(7);
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
