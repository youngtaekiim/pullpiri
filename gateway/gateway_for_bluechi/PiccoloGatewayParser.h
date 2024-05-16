// SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
//
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef _PiccoloGatewayParser_
#define _PiccoloGatewayParser_

#include "yaml-cpp/yaml.h"
#include "PiccoloEvent.h"
#include "etcd/Client.hpp"

namespace PiccoloGatewayParser
{
	void parse(PiccoloEvent* pe);
}
#endif

