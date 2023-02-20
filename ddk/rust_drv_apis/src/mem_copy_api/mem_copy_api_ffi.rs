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

extern "C" {
    ///
    /// copy data from client
    ///
    /// # Parameters
    /// src: source addr
    /// src_size: source data size
    /// dst: dest addr
    /// dst_size: dest data size
    ///
    /// # Return
    /// 0 if success
    /// -1 if fail
    pub fn copy_from_client(src: u64, src_size: u32, dst: usize, dst_size: u32) -> i32;

    ///
    /// copy data to client
    ///
    /// # Parameters
    /// src: source addr
    /// src_size: source data size
    /// dst: dest addr
    /// dst_size: dest data size
    ///
    /// # Return
    /// 0 if success
    /// -1 if fail
    pub fn copy_to_client(src: usize, src_size: u32, dst: u64, dst_size: u32) -> i32;
}
