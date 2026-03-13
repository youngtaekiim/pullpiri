#!/bin/bash
# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

podman run -it -d --name timpani-o -p 50052:50052 -p 7777:7777 sdv.lge.com/timpani/timpani-o:v0.1.0
