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

pub type HwiProcFunc = extern "C" fn(u32);

extern "C" {

    ///
    /// CODEREVIEW CHECKLIST
    /// ARG:  hwi_num: checked, can't bigger than OS_HWI_MAX_NUM
    ///        hwi_prio: checked, cant't bigger than 0xff.
    ///        mode: checked, user mode is either INT_NONSECURE or INT_SECURE.
    ///        handler: checked, mandatory irqHandler should be given
    ///        args: no need to check, args for user given irqHandler
    /// BUFOVF: the size of irq is MAX_IRQ
    /// LOG: Non critical info printed
    /// RET: cs_client_call: checked
    ///       hmapi_create_notification: checked
    ///       hmex_gic_set_group: checked
    ///       hmex_setup_irq_handler: checked
    ///       sre_hwi_cfg_gic: checked
    /// LEAK: If have allocated some resourse and fail in next steps, will
    ///        goto ErrHandler and do cleanup
    ///
    ///
    /// Note: handler is ignored here, you need to receive the irq notification
    /// on the registered `g_hwi_chnl` and trigger the handler by yourself.
    ///
    pub fn sys_hwi_create(
        hwi_num: u32,
        hwi_prio: u16,
        mode: u16,
        handler: HwiProcFunc,
        args: u32,
    ) -> u32;

    ///
    /// CODEREVIEW CHECKLIST
    /// ARG:  hwi_num: checked, can't bigger than OS_HWI_MAX_NUM
    ///      hwi_prio: checked, cant't bigger than 0xff.
    ///      mode: checked, user mode is either INT_NONSECURE or INT_SECURE.
    /// RET: return the error to caller if sre_hwi_cfg_gic fail.
    ///
    pub fn sys_hwi_resume(hwi_num: u32, hwi_prio: u16, mode: u16) -> u32;

    pub fn sys_hwi_delete(hwi_num: u32) -> u32;

    ///
    /// disable an interrupt
    /// CODEREVIEW CHECKLIST
    /// ARG:  hwi_num: checked, can't bigger than OS_HWI_MAX_NUM
    /// LOG: Non critical info printed
    /// RET: hmex_disable_irq: checked
    ///
    pub fn sys_hwi_disable(hwi_num: u32) -> u32;

    ///
    /// enable an interrupt
    /// CODEREVIEW CHECKLIST
    /// ARG:  hwi_num: checked, can't bigger than OS_HWI_MAX_NUM
    /// LOG: Non critical info printed
    /// RET: hmex_enable_irq: checked
    ///
    pub fn sys_hwi_enable(hwi_num: u32) -> u32;

    pub fn sys_hwi_notify(hwi_num: u32) -> u32;
}
