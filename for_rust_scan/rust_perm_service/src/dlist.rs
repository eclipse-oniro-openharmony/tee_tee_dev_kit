// Copyright (C) 2023 Huawei Technologies Co., Ltd.
// Licensed under the Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//     http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
use core::mem::MaybeUninit;

// we do not read or write, just count offset, so it's safe to use raw pointer with ERROR addr.
#[macro_export]
macro_rules! container_of {
    ($ptr:expr, $kind: ty, $field: ident) => {
        unsafe {
            let outer = 0usize;
            let offset = (&(*(outer as *const $kind)).$field) as *const dlist_node as usize;
            &(*((($ptr as *mut dlist_node as usize) - offset) as *const $kind))
        }
    };
}

// we do not read or write, just count offset, so it's safe to use raw pointer with ERROR addr.
#[macro_export]
macro_rules! container_of_mut {
    ($ptr:expr, $kind: ty, $field: ident) => {
        unsafe {
            let outer = 0usize;
            let offset = (&(*(outer as *const $kind)).$field) as *const dlist_node as usize;
            &mut (*((($ptr as *mut dlist_node as usize) - offset) as *mut $kind))
        }
    };
}

#[repr(C)]
pub struct dlist_node {
    pub prev: *mut dlist_node,
    pub next: *mut dlist_node,
}

unsafe impl Send for dlist_node {}

#[inline(always)]
pub fn dlist_head() -> dlist_node {
    let mut node: dlist_node = unsafe { MaybeUninit::zeroed().assume_init() };
    node.prev = &mut node;
    node.next = &mut node;
    node
}

#[inline(always)]
pub fn dlist_head_init(node: &mut dlist_node) {
    node.prev = node;
    node.next = node;
}

/*
 * Initialize the empty dlist
 * PRE: a dlist_node struct for the head node, with unspecified field values
 * POST: the field set to point to the head node itself, thus initialized to be an empty dlist
 */
#[inline(always)]
pub fn dlist_init(node: &mut dlist_node) {
    node.prev = node;
    node.next = node;
}

/*
 * Check if the dlist is empty
 * PRE: head points to the head node of a well formed dlist
 * POST: return 1 if the dlist is empty, return 0 if it is not
 */
#[inline(always)]
pub fn dlist_empty(head: &dlist_node) -> bool {
    /* dlist is well formed, so only needs check the next ptr here */
    return head.next as *const dlist_node as u64 == head as *const dlist_node as u64;
}

/*
 * Get the first node of a dlist
 * PRE: head points to the head node of a well formed dlist
 * POST: return the pointer to the first node of the dlist if it's not empty, or to the head node if it's empty
 */
#[inline(always)]
pub fn dlist_get_first(head: &dlist_node) -> &mut dlist_node {
    return unsafe { &mut *(head.next) };
}

/*
 * Get the last node of a dlist
 * PRE: head points to the head node of a well formed dlist
 * POST: return the pointer to the last node of the dlist if it's not empty, or to the head node if it's empty
 */
#[inline(always)]
pub fn dlist_get_last(head: &dlist_node) -> &mut dlist_node {
    return unsafe { &mut *(head.prev) };
}

/*
 * Insert after a given position of the dlist
 * PRE: pos points to a node(can be the head node) in a well formed dlist, node points to a node to be inserted(not in
 * the dlist) POST: node has been inserted into the dlist after pos, the new dlist is well formed
 */
#[inline(always)]
pub fn dlist_insert(pos: &mut dlist_node, node: &mut dlist_node) {
    let tmp = unsafe { &mut *(pos.next) };
    tmp.prev = node;
    node.prev = pos;
    node.next = pos.next;
    pos.next = node;
}

/*
 * Insert a new node at head of a dlist
 * PRE: head points to the head node of a well formed dlist, node points to the node to be inserted(not in the dlist)
 * POST: the new node has been inserted to the head of the dlist, the new dlist is well formed
 */
#[inline(always)]
pub fn dlist_insert_head(node: &mut dlist_node, head: &mut dlist_node) {
    dlist_insert(head, node);
}

/*
 * Insert a new node at tail of a dlist
 * PRE: head points to the head node of a well formed dlist, node points to the node to be inserted(not in the dlist)
 * POST: the new node has been inserted to the tail of the dlist, the new dlist is well formed
 */
#[inline(always)]
pub fn dlist_insert_tail(node: &mut dlist_node, head: &dlist_node) {
    let tmp = dlist_get_last(head);
    dlist_insert(tmp, node);
}

/*
 * Delete a node from a dlist
 * PRE: node points to a node in a well formed dlist
 * POST: node has been taken out of the dlist, the remaining dlist is still well formed
 */
#[inline(always)]
pub fn dlist_delete(node: &mut dlist_node) {
    let mut tmp = unsafe { &mut *(node.prev) };
    tmp.next = node.next;
    tmp = unsafe { &mut *(node.next) };
    tmp.prev = node.prev;
    dlist_init(node);
}

/*
 * Replace an old node in the dlist with a new node
 * PRE: old node points to a node in the dlist, new node points a node not in the dlist, dlist well formed
 * POST: the new node has been inserted into the dlist, the old node has been taken out, the dlist is still well formed
 */
#[inline(always)]
pub fn dlist_replace(old_node: &dlist_node, new_node: &mut dlist_node) {
    new_node.prev = old_node.prev;
    new_node.next = old_node.next;
    let mut tmp = unsafe { &mut *(old_node.prev) };
    tmp.next = new_node;
    tmp = unsafe { &mut *(old_node.next) };
    tmp.prev = new_node;
}

/*
 * Get the prev node of a dlist node or a dlist head
 * PRE: node points to a dlist head or a dlist node of a well formed dlist
 * POST: return the pointer to the prev node of the dlist node or the dlist head
 */
#[inline(always)]
fn dlist_get_prev(node: &dlist_node) -> &mut dlist_node {
    return unsafe { &mut *(node.prev) };
}

/*
 * Get the next node of a dlist node or a dlist head
 * PRE: node points to a dlist head or a dlist node of a well formed dlist
 * POST: return the pointer to the next node of the dlist node or the dlist head
 */
#[inline(always)]
fn dlist_get_next(node: &dlist_node) -> &mut dlist_node {
    return unsafe { &mut *(node.next) };
}

/* get the address of the containing struct */
#[macro_export]
macro_rules! dlist_entry {
    ($ptr:expr, $kind: ty, $member: ident) => {
        crate::container_of_mut!($ptr, $kind, $member)
    };
}

/* dlist_fisrt_entry */
#[macro_export]
macro_rules! dlist_first_entry {
    ($ptr:expr, $kind: ty, $member: ident) => {
        crate::dlist_entry!(($ptr).next, $kind, $member)
    };
}

/* dlist_last_entry */
#[macro_export]
macro_rules! dlist_last_entry {
    ($ptr:expr, $kind: ty, $member: ident) => {
        crate::dlist_entry!(($ptr).prev, $kind, $member)
    };
}

/* get the address of the next containing struct on the dlist */
#[macro_export]
macro_rules! dlist_next_entry {
    ($ptr:expr, $kind: ty, $member: ident) => {
        crate::dlist_entry!(($ptr).member.next, $kind, $member)
    };
}

/* get the address of the previous containing struct on the dlist */
#[macro_export]
macro_rules! dlist_prev_entry {
    ($ptr:expr, $kind: ty, $member: ident) => {
        crate::dlist_entry!(($ptr).member.prev, $kind, $member)
    };
}

#[macro_export]
macro_rules! dlist_for_each {
    ($pos:ident, $head: expr, $body: tt) => {
        $pos = unsafe { &mut *(($head).next)};
        while $pos as *mut dlist_node as u64 != $head as *const dlist_node as u64 {
            $body
            $pos = unsafe { &mut *($pos.next)};
        }
    };
}

#[macro_export]
macro_rules! dlist_for_each_prev {
    ($pos:ident, $head: expr, $body: tt) => {
        $pos = ($head).prev;
        while pos as *mut dlist_node as u64 != $head as *const dlist_node as u64 {
            $body
            $pos = $pos.prev;
        }
    };
}

#[macro_export]
macro_rules! dlist_for_each_safe {
    ($pos:ident, $n:ident, $head: expr, $body: tt) => {
        $pos = unsafe { &mut *(($head).next)};
        $n = unsafe { &mut *($pos.next)};
        while $pos as *mut dlist_node as u64 != $head as *const dlist_node as u64 {
            $body
            $pos = $n;
            $n = unsafe { &mut *($pos.next)};
        }
    };
}

#[macro_export]
macro_rules! dlist_for_each_entry {
    ($pos:ident, $head: expr, $kind:ty, $member:ident, $body: tt) => {
        $pos = crate::dlist_first_entry!($head, $kind, $member);
        while &($pos.$member) as *mut dlist_node as u64 != $head as *const dlist_node as u64 {
            $body
            $pos = crate::dlist_next_entry!($pos, $kind, $member);
        }
    };
}

#[macro_export]
macro_rules! dlist_for_each_entry_safe {
    ($pos:ident, $n: ident, $head: expr, $kind:ty, $member:ident, $body: tt) => {
        $pos = crate::dlist_first_entry!($head, $kind, $member);
        $n = crate::dlist_next_entry!($pos, $kind, $member);
        while &($pos.$member) as *mut dlist_node as u64 != $head as *const dlist_node as u64 {
            $body
            $pos = $n;
            $n = crate::dlist_next_entry!($n, $kind, $member);
        }
    };
}
