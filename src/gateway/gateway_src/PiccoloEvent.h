// SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
//
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef _PiccoloEvent_
#define _PiccoloEvent_

#include <string>
#include <vector>
#include <memory>

struct PiccoloEvent
{
	std::string name;
	std::string express;
	std::string targetValue;
	std::string topic;
	std::string actionKey;
	std::string targetDest;
	std::string lifeCycle = std::string("oneTime");
};
#endif

