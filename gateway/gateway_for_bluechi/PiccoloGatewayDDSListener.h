// SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
//
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef _PiccoloGatewayDDSListener_
#define _PiccoloGatewayDDSListener_

#include "PiccoloGatewayManager.h"

#include "dds/dds.h"
#include "dds/dds.hpp"

class PiccoloGatewayDDSListener{
	public:
		virtual ~PiccoloGatewayDDSListener(){}
		virtual void run() = 0;

		bool keepRunning_;
	private:
		class PiccoloGatewayManager* manager_;
};

#endif

