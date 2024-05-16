// SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
//
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef _PiccoloGatewayComparator_
#define _PiccoloGatewayComparator_

#include "PiccoloGatewayManager.h"
#include "PiccoloEvent.h"
#include <string>

class PiccoloGatewayComparator
{
	public:
		virtual ~PiccoloGatewayComparator(){}
		virtual void compare(void* data) = 0;
		std::string name;
		class PiccoloEvent* pe;

	private:
		class PiccoloGatewayManager* manager_;
};

#endif

