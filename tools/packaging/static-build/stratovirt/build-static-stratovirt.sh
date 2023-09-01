#!/usr/bin/env bash
#
# Copyright (c) 2023 Huawei Technologies Co.,Ltd.
#
# SPDX-License-Identifier: Apache-2.0

set -o errexit
set -o nounset
set -o pipefail

ARCH=$(uname -m)

# Currently, StratoVirt only support x86_64 and aarch64.
[ "${ARCH}" != "x86_64" ] && [ "${ARCH}" != "aarch64" ] && exit

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${script_dir}/../../scripts/lib.sh"

info "Get stratovirt information from runtime versions.yaml"
stratovirt_url="${stratovirt_url:-}"
[ -n "$stratovirt_url" ] || stratovirt_url=$(get_from_kata_deps "assets.hypervisor.stratovirt.url")
[ -n "$stratovirt_url" ] || die "failed to get stratovirt url"

stratovirt_version="${stratovirt_version:-}"
[ -n "$stratovirt_version" ] || stratovirt_version=$(get_from_kata_deps "assets.hypervisor.stratovirt.version")
[ -n "$stratovirt_version" ] || die "failed to get stratovirt version"

build_stratovirt_from_release_source() {
	stratovirt_tarball_name=stratovirt-${stratovirt_version}.tar.gz
	stratovirt_tarball_url="${stratovirt_url}/releases/tag/v${stratovirt_version}/${stratovirt_tarball_name}"
	
	info "Download ${stratovirt_tarball_url}, version: v${stratovirt_version}"
	curl -L ${stratovirt_tarball_url} -o ${stratovirt_tarball_name}
	tar zxvf ${stratovirt_tarball_name}

	pushd stratovirt-${stratovirt_version}
	tools/build_stratovirt/build-stratovirt-from-docker.sh kata_stratovirt_build
	popd

	mkdir -p static-stratovirt
	cp stratovirt-${stratovirt_version}/target/${ARCH}-unknown-linux-musl/release/stratovirt static-stratovirt/stratovirt
	rm -rf stratovirt stratovirt.tar.gz
}

info "Build StratoVirt from release source."
build_stratovirt_from_release_source

