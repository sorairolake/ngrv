#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2025 Shun Sakai
#
# SPDX-License-Identifier: GPL-3.0-or-later

set -euxCo pipefail

scriptDir=$(cd "$(dirname "$0")" && pwd)
cd "$scriptDir"

curl -O --skip-existing https://www.kernel.org/pub/software/scm/git/git-2.49.0.tar.xz
echo "618190cf590b7e9f6c11f91f23b1d267cd98c3ab33b850416d8758f8b5a85628  git-2.49.0.tar.xz" | sha256sum -c
curl -O --skip-existing https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.14.tar.xz
echo "a294b683e7b161bb0517bb32ec7ed1d2ea7603dfbabad135170ed12d00c47670  linux-6.14.tar.xz" | sha256sum -c

autocast --overwrite demo.yaml demo.cast
agg --font-family "Cascadia Code,Hack,Source Code Pro" demo.cast demo.gif
gifsicle -b -O3 demo.gif
