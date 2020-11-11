// Copyright (c) 2020 And Group
//
// SPDX-License-Identifier: Apache-2.0 or MIT
//

//! Integration tests about the hugetlb subsystem
use cgroups::error::*;
use cgroups::hugetlb::{self, HugeTlbController};
use cgroups::Cgroup;
use std::fs;

#[test]
fn test_hugetlb_sizes() {
    // now only v2
    if cgroups::hierarchies::is_cgroup2_unified_mode() {
        return;
    }

    let h = cgroups::hierarchies::auto();
    let cg = Cgroup::new(&*h, String::from("test_hugetlb_sizes"));
    {
        let hugetlb_controller: &HugeTlbController = cg.controller_of().unwrap();
        let _ = hugetlb_controller.get_sizes();

        // test sizes count
        let sizes = hugetlb_controller.get_sizes();
        let sizes_count = fs::read_dir(hugetlb::HUGEPAGESIZE_DIR).unwrap().count();
        assert_eq!(sizes.len(), sizes_count);

        for size in sizes {
            let supported = hugetlb_controller.size_supported(&size);
            assert_eq!(supported, true);
            assert_no_error(hugetlb_controller.failcnt(&size));
            assert_no_error(hugetlb_controller.limit_in_bytes(&size));
            assert_no_error(hugetlb_controller.usage_in_bytes(&size));
            assert_no_error(hugetlb_controller.max_usage_in_bytes(&size));
        }
    }
    cg.delete().unwrap();
}

fn assert_no_error(r: Result<u64>) {
    assert_eq!(!r.is_err(), true)
}
